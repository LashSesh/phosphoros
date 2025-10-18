//! Simple functional app implementation for phosphoros-dashboard
//! This is a minimal working version that can be extended

use crate::config::Config;
use crate::messages::*;
use crate::panels::{PanelId, ClusterInfo, StealthMode, ApiType};
use crate::state::{AppState, LogEntry, LogLevel, Notification, NotificationKind};
use crate::theme::PhosphorosTheme;
use crate::integration::{WalletIntegration, ResonanceIntegration, AnalysisIntegration};
use crate::tasks::{TaskManager, TaskMessage};
use crate::export::{ExportService, ExportFormat, SystemStats};
use chrono::Utc;
use iced::widget::{button, column, container, row, scrollable, text, text_input, toggler, progress_bar, horizontal_rule, horizontal_space, vertical_space};
use iced::{Element, Length, Subscription, Task, Theme};
use std::time::Duration;
use std::sync::Arc;

/// Main application
#[derive(Debug)]
pub struct PhosphorosApp {
    state: AppState,
    config: Config,
    theme: PhosphorosTheme,
    notification_counter: usize,
    // Integration components (not Debug, so we store them separately)
    resonance_integration: Option<ResonanceIntegration>,
    analysis_integration: Option<AnalysisIntegration>,
    // Task manager for async operations
    task_manager: Option<TaskManager>,
    // Background task handles
    background_tasks_spawned: bool,
}

impl PhosphorosApp {
    pub fn new() -> (Self, Task<Message>) {
        let config = Config::default();
        let state = AppState::default();
        let theme = PhosphorosTheme::new(config.theme.dark_mode);

        let mut app = Self {
            state,
            config,
            theme,
            notification_counter: 0,
            resonance_integration: Some(ResonanceIntegration::new()),
            analysis_integration: Some(AnalysisIntegration::new()),
            task_manager: Some(TaskManager::new()),
            background_tasks_spawned: false,
        };

        app.add_log(LogLevel::Info, "System", "PHOSPHOROS Dashboard initialized");

        if app.config.services.auto_start {
            app.state.service_manager.start_all();
            app.add_log(LogLevel::Info, "Services", "Auto-starting all services");
        }

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        // Spawn background tasks on first update if not already spawned
        if !self.background_tasks_spawned && self.task_manager.is_some() {
            self.spawn_background_tasks();
            self.background_tasks_spawned = true;
        }
        
        // Process task messages - collect them first to avoid borrow issues
        let task_messages: Vec<TaskMessage> = if let Some(ref task_manager) = self.task_manager {
            let mut messages = Vec::new();
            while let Some(task_msg) = task_manager.try_recv() {
                messages.push(task_msg);
            }
            messages
        } else {
            Vec::new()
        };
        
        // Handle collected messages
        for task_msg in task_messages {
            self.handle_task_message(task_msg);
        }
        
        match message {
            Message::Panel(panel_msg) => self.handle_panel_message(panel_msg),
            Message::Service(_) => Task::none(),
            Message::UI(_) => Task::none(),
            Message::System(SystemMessage::Tick) => {
                // Update home panel stats
                let stats = self.state.service_manager.stats_summary();
                self.state.panels.home.seeds_count = self.state.panels.seed_management.seeds.len();
                self.state.panels.home.clusters_count = stats.clusters_found;
                
                // Update cluster panel
                let clusters: Vec<ClusterInfo> = self.state.service_manager.data_pool.read()
                    .clusters.values()
                    .map(|c| ClusterInfo {
                        id: c.id.clone(),
                        members: c.members.len(),
                        resonance: c.resonance,
                        discovered_at: c.timestamp,
                    })
                    .collect();
                self.state.panels.cluster.clusters = clusters;
                
                // Update anomaly investigation panel
                use crate::panels::{AnomalyInfo, AnomalyType, AnomalySeverity};
                let anomalies: Vec<AnomalyInfo> = self.state.service_manager.data_pool.read()
                    .anomalies.iter()
                    .map(|a| {
                        let severity = if a.score > 0.9 {
                            AnomalySeverity::Critical
                        } else if a.score > 0.7 {
                            AnomalySeverity::High
                        } else if a.score > 0.5 {
                            AnomalySeverity::Medium
                        } else {
                            AnomalySeverity::Low
                        };
                        
                        AnomalyInfo {
                            id: a.id.to_string(),
                            anomaly_type: AnomalyType::Other, // Could be enhanced with real type detection
                            severity,
                            affected_entities: vec![a.entity_id.to_string()],
                            detected_at: a.timestamp,
                            score: a.score,
                            description: a.reason.clone(),
                        }
                    })
                    .collect();
                self.state.panels.anomaly.anomalies = anomalies;
                
                // Update infogenetic browser with entities
                use crate::panels::InfogeneticEntry;
                let entries: Vec<InfogeneticEntry> = self.state.service_manager.data_pool.read()
                    .entities.values()
                    .take(100) // Limit to 100 for performance
                    .map(|e| {
                        let (psi, rho, omega) = if e.features.len() >= 3 {
                            (e.features[0], e.features[1], e.features[2])
                        } else {
                            (0.0, 0.0, 0.0)
                        };
                        InfogeneticEntry {
                            id: e.id.to_string(),
                            address: e.address.clone(),
                            signature: (psi, rho, omega),
                            resonance: psi * rho * omega,
                            cluster_id: None,
                            chain: "Unknown".to_string(),
                            discovered_at: e.timestamp,
                            metadata: std::collections::HashMap::new(),
                        }
                    })
                    .collect();
                self.state.panels.infogenetic.results = entries;
                self.state.panels.infogenetic.total_results = self.state.service_manager.data_pool.read().entities.len();
                
                Task::none()
            }
            Message::System(_) => Task::none(),
            Message::Notification(_) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar_view();
        let main_content = self.main_view();

        let content = row![
            sidebar,
            horizontal_rule(1),
            main_content
        ]
        .spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1))
            .map(|_| Message::System(SystemMessage::Tick))
    }

    pub fn theme(&self) -> Theme {
        self.theme.theme()
    }

    fn sidebar_view(&self) -> Element<Message> {
        let mut sidebar_content = column![]
            .spacing(8)
            .padding(16);

        sidebar_content = sidebar_content.push(text("PHOSPHOROS").size(24));
        sidebar_content = sidebar_content.push(text("Living Lab").size(14));
        sidebar_content = sidebar_content.push(vertical_space().height(20));

        for panel_id in PanelId::all() {
            let btn_text = format!("{} {}", panel_id.icon(), panel_id.name());
            let btn = button(text(btn_text).size(14))
                .on_press(Message::Panel(PanelMessage::SwitchTo(panel_id)))
                .width(Length::Fill)
                .padding(12);
            sidebar_content = sidebar_content.push(btn);
        }

        sidebar_content = sidebar_content.push(vertical_space().height(20));
        let active_tasks = self.state.service_manager.active_tasks();
        let tasks_text = if active_tasks > 0 {
            format!("⚡ {} Active Tasks", active_tasks)
        } else {
            "⏸ All Tasks Paused".to_string()
        };
        sidebar_content = sidebar_content.push(text(tasks_text).size(12));

        container(sidebar_content)
            .width(250)
            .height(Length::Fill)
            .style(|_theme: &Theme| crate::theme::container_styles::sidebar())
            .into()
    }

    fn main_view(&self) -> Element<Message> {
        let content = match self.state.active_panel {
            PanelId::Home => self.home_view(),
            PanelId::SeedManagement => self.seed_view(),
            PanelId::Resonance => self.resonance_view(),
            PanelId::ClusterExplorer => self.cluster_view(),
            PanelId::SearchSpaceExplorer => self.search_space_view(),
            PanelId::NetworkExplorer => self.network_explorer_view(),
            PanelId::InfogeneticBrowser => self.infogenetic_browser_view(),
            PanelId::AnomalyInvestigation => self.anomaly_investigation_view(),
            PanelId::ForensicWorkflows => self.forensic_workflows_view(),
            PanelId::Stealth => self.stealth_view(),
            PanelId::SystemLog => self.log_view(),
            PanelId::Settings => self.settings_view(),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn home_view(&self) -> Element<Message> {
        let title = text(PanelId::Home.name()).size(28);
        
        // Get live statistics
        let stats = self.state.service_manager.stats_summary();
        
        // Create stat cards inline to avoid lifetime issues
        let stats_row = row![
            crate::widgets::card(
                column![
                    text("Seeds").size(14),
                    vertical_space().height(8),
                    text(self.state.panels.home.seeds_count.to_string()).size(32),
                ]
            ),
            horizontal_space().width(16),
            crate::widgets::card(
                column![
                    text("Clusters").size(14),
                    vertical_space().height(8),
                    text(stats.clusters_found.to_string()).size(32),
                ]
            ),
            horizontal_space().width(16),
            crate::widgets::card(
                column![
                    text("Entities").size(14),
                    vertical_space().height(8),
                    text(stats.entities_in_pool.to_string()).size(32),
                ]
            ),
        ];

        let status_card = crate::widgets::card(
            column![
                text("System Status").size(18),
                vertical_space().height(10),
                text(format!("Scraper: {} - {} entities", 
                    if self.state.service_manager.scraper.read().running { "🟢 Running" } else { "⏸ Paused" },
                    stats.scraper_processed
                )).size(14),
                vertical_space().height(5),
                text(format!("Analyzer: {} - {} analyzed", 
                    if self.state.service_manager.analyzer.read().running { "🟢 Running" } else { "⏸ Paused" },
                    stats.analyzer_analyzed
                )).size(14),
                vertical_space().height(5),
                text(format!("Cluster Engine: {} - {} clusters", 
                    if self.state.service_manager.cluster_engine.read().running { "🟢 Running" } else { "⏸ Paused" },
                    stats.clusters_found
                )).size(14),
            ]
        );
        
        // Add anomaly info
        let anomaly_count = self.state.service_manager.data_pool.read().anomalies.len();
        let anomaly_card = crate::widgets::card(
            column![
                text("Anomaly Detection").size(18),
                vertical_space().height(10),
                text(format!("Detected: {} anomalies", anomaly_count)).size(14),
                vertical_space().height(5),
                text("Real-time pattern recognition active").size(12),
            ]
        );

        column![
            title,
            vertical_space().height(20),
            stats_row,
            vertical_space().height(20),
            status_card,
            vertical_space().height(20),
            anomaly_card,
        ].into()
    }

    fn seed_view(&self) -> Element<Message> {
        let title = text(PanelId::SeedManagement.name()).size(28);

        let input = text_input(
            "Enter mnemonic, seed, or private key...",
            &self.state.panels.seed_management.input,
        )
        .on_input(|s| Message::Panel(PanelMessage::SeedManagement(SeedMessage::InputChanged(s))))
        .padding(12);

        let buttons = row![
            button(text("Import Seed"))
                .on_press(Message::Panel(PanelMessage::SeedManagement(SeedMessage::ImportSeed)))
                .padding(10),
            horizontal_space().width(10),
            button(text("Clear"))
                .on_press(Message::Panel(PanelMessage::SeedManagement(SeedMessage::Clear)))
                .padding(10),
        ];

        let mut content = column![
            title,
            vertical_space().height(20),
            input,
            vertical_space().height(10),
            buttons,
        ];

        if !self.state.panels.seed_management.seeds.is_empty() {
            content = content.push(vertical_space().height(20));
            content = content.push(text(format!("Imported Seeds ({})", self.state.panels.seed_management.seeds.len())).size(18));
            content = content.push(vertical_space().height(15));
            
            // Display each imported seed
            for seed_info in self.state.panels.seed_management.seeds.iter().take(5) {
                let seed_card = crate::widgets::card(
                    column![
                        text(&seed_info.mnemonic_masked).size(14),
                        vertical_space().height(5),
                        text(format!("{} addresses", seed_info.addresses.len())).size(12),
                        vertical_space().height(5),
                    ]
                    .push_maybe(seed_info.addresses.first().map(|addr| {
                        text(format!("{}: {}", addr.chain, &addr.address[..20])).size(11)
                    }))
                );
                content = content.push(seed_card);
                content = content.push(vertical_space().height(10));
            }
        }

        scrollable(content).into()
    }

    fn resonance_view(&self) -> Element<Message> {
        let title = text(PanelId::Resonance.name()).size(28);

        let status = if self.state.panels.resonance.running {
            "Analysis Running..."
        } else {
            "Ready to Analyze"
        };

        let prog = if self.state.panels.resonance.running {
            progress_bar(0.0..=1.0, self.state.panels.resonance.progress as f32)
        } else {
            progress_bar(0.0..=1.0, 0.0)
        };

        let best_res = text(format!("Best Resonance: {:.6}", self.state.panels.resonance.best_resonance)).size(16);

        let start_btn = button(text(if self.state.panels.resonance.running { "Stop" } else { "Start Analysis" }))
            .on_press(Message::Panel(PanelMessage::Resonance(ResonanceMessage::StartAnalysis)))
            .padding(10);

        column![
            title,
            vertical_space().height(20),
            text(status).size(18),
            vertical_space().height(10),
            prog,
            vertical_space().height(15),
            best_res,
            vertical_space().height(20),
            start_btn,
        ].into()
    }

    fn cluster_view(&self) -> Element<Message> {
        let title = text(PanelId::ClusterExplorer.name()).size(28);

        let search = text_input(
            "Search clusters...",
            &self.state.panels.cluster.search_query,
        )
        .on_input(|s| Message::Panel(PanelMessage::Cluster(ClusterMessage::Search(s))))
        .padding(12);

        // Export buttons
        let export_row = row![
            button(text("Export JSON").size(12))
                .on_press(Message::Panel(PanelMessage::Cluster(ClusterMessage::ExportAllJson)))
                .padding(8),
            horizontal_space().width(5),
            button(text("Export CSV").size(12))
                .on_press(Message::Panel(PanelMessage::Cluster(ClusterMessage::ExportAllCsv)))
                .padding(8),
            horizontal_space().width(5),
            button(text("Export Markdown").size(12))
                .on_press(Message::Panel(PanelMessage::Cluster(ClusterMessage::ExportAllMarkdown)))
                .padding(8),
        ];

        let mut content = column![
            title,
            vertical_space().height(20),
            search,
            vertical_space().height(10),
            export_row,
            vertical_space().height(20),
        ];

        if self.state.panels.cluster.clusters.is_empty() {
            content = content.push(text("No clusters found. Start services to discover clusters.").size(14));
        } else {
            content = content.push(text(format!("Found {} clusters", self.state.panels.cluster.clusters.len())).size(16));
            content = content.push(vertical_space().height(15));
            
            // Display clusters
            for (idx, cluster) in self.state.panels.cluster.clusters.iter().enumerate().take(10) {
                let cluster_card = crate::widgets::card(
                    column![
                        text(format!("Cluster #{}", idx + 1)).size(16),
                        vertical_space().height(5),
                        text(format!("Members: {}", cluster.members)).size(14),
                        text(format!("Resonance: {:.3}", cluster.resonance)).size(14),
                        text(format!("ID: {}", &cluster.id[..20])).size(11),
                    ]
                );
                content = content.push(cluster_card);
                content = content.push(vertical_space().height(10));
            }
        }

        scrollable(content).into()
    }

    fn log_view(&self) -> Element<Message> {
        let title = text(PanelId::SystemLog.name()).size(28);

        let filter = text_input(
            "Filter logs...",
            &self.state.panels.log.filter,
        )
        .on_input(|s| Message::Panel(PanelMessage::Log(LogMessage::FilterChanged(s))))
        .padding(12);

        let clear_btn = button(text("Clear"))
            .on_press(Message::Panel(PanelMessage::Log(LogMessage::Clear)))
            .padding(10);

        let mut log_entries = column![].spacing(3);
        let filtered = self.state.filtered_logs(&self.state.panels.log.filter);
        for log in filtered.iter().rev().take(50) {
            log_entries = log_entries.push(
                text(format!("{} [{}] {}: {}", 
                    log.timestamp.format("%H:%M:%S"),
                    log.level,
                    log.source,
                    log.message
                )).size(11)
            );
        }

        let content = column![
            title,
            vertical_space().height(20),
            row![filter, horizontal_space().width(10), clear_btn],
            vertical_space().height(20),
            scrollable(log_entries).height(Length::Fill),
        ];

        content.into()
    }

    fn stealth_view(&self) -> Element<Message> {
        let title = text(PanelId::Stealth.name()).size(28);
        
        let stealth_state = &self.state.panels.stealth;
        
        // Stealth mode toggle
        let enabled_row = row![
            text("Enable Stealth Mode").size(16),
            horizontal_space().width(10),
            toggler(stealth_state.enabled)
                .on_toggle(|_| Message::Panel(PanelMessage::Stealth(StealthMessage::ToggleEnabled))),
        ];
        
        // Mode selection
        let mode_label = text("Stealth Mode:").size(14);
        let mode_value = text(match stealth_state.mode {
            StealthMode::Open => "Open (No concealment)",
            StealthMode::Mimicry => "Mimicry (API traffic disguise)",
            StealthMode::Steganography => "Steganography (Hidden payloads)",
            StealthMode::Adaptive => "Adaptive (Context-aware)",
        }).size(14);
        
        let mode_buttons = row![
            button(text("Open")).padding(8)
                .on_press(Message::Panel(PanelMessage::Stealth(StealthMessage::SetMode(StealthMode::Open)))),
            horizontal_space().width(5),
            button(text("Mimicry")).padding(8)
                .on_press(Message::Panel(PanelMessage::Stealth(StealthMessage::SetMode(StealthMode::Mimicry)))),
            horizontal_space().width(5),
            button(text("Steganography")).padding(8)
                .on_press(Message::Panel(PanelMessage::Stealth(StealthMessage::SetMode(StealthMode::Steganography)))),
            horizontal_space().width(5),
            button(text("Adaptive")).padding(8)
                .on_press(Message::Panel(PanelMessage::Stealth(StealthMessage::SetMode(StealthMode::Adaptive)))),
        ];
        
        // Stats
        let stats = crate::widgets::card(
            column![
                text("Stealth Statistics").size(18),
                vertical_space().height(10),
                text(format!("Active Tasks: {}", stealth_state.active_tasks)).size(14),
                text(format!("Proxies Configured: {}", stealth_state.proxy_count)).size(14),
                text(format!("Logging: {}", if stealth_state.logging_enabled { "Enabled" } else { "Disabled" })).size(14),
            ]
        );
        
        // Proxy settings
        let proxy_row = row![
            text("Proxy Rotation").size(14),
            horizontal_space().width(10),
            toggler(stealth_state.proxy_enabled)
                .on_toggle(|_| Message::Panel(PanelMessage::Stealth(StealthMessage::ToggleProxy))),
        ];
        
        // Jitter settings
        let jitter_row = row![
            text("Temporal Jitter").size(14),
            horizontal_space().width(10),
            toggler(stealth_state.jitter_enabled)
                .on_toggle(|_| Message::Panel(PanelMessage::Stealth(StealthMessage::ToggleJitter))),
        ];
        
        let jitter_info = text(format!(
            "Jitter range: {} - {} ms",
            stealth_state.min_jitter_ms,
            stealth_state.max_jitter_ms
        )).size(12);
        
        // Compliance notice
        let compliance_notice = crate::widgets::card(
            column![
                text("⚠️ Compliance Notice").size(16),
                vertical_space().height(5),
                text("Stealth networking is intended ONLY for:").size(12),
                text("• Legitimate forensic analysis").size(11),
                text("• Scientific research").size(11),
                text("• Defense and security with authorization").size(11),
                text("• Compliance-approved use cases").size(11),
                vertical_space().height(5),
                text("Misuse may violate laws and regulations.").size(11),
            ]
        );
        
        let content = column![
            title,
            vertical_space().height(20),
            enabled_row,
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Mode Configuration").size(18),
                    vertical_space().height(10),
                    mode_label,
                    mode_value,
                    vertical_space().height(10),
                    mode_buttons,
                ]
            ),
            vertical_space().height(15),
            stats,
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Advanced Settings").size(18),
                    vertical_space().height(10),
                    proxy_row,
                    vertical_space().height(10),
                    jitter_row,
                    vertical_space().height(5),
                    jitter_info,
                ]
            ),
            vertical_space().height(15),
            compliance_notice,
        ];
        
        scrollable(content).into()
    }

    fn settings_view(&self) -> Element<Message> {
        let title = text(PanelId::Settings.name()).size(28);

        let theme_row = row![
            text("Dark Mode").size(14),
            horizontal_space().width(10),
            toggler(self.state.dark_mode)
                .on_toggle(|_| Message::Panel(PanelMessage::Settings(SettingsMessage::ToggleTheme))),
        ];
        
        let auto_start_row = row![
            text("Auto-start Services").size(14),
            horizontal_space().width(10),
            toggler(self.config.services.auto_start)
                .on_toggle(|_| Message::Panel(PanelMessage::Settings(SettingsMessage::ToggleAutoStart))),
        ];
        
        let report_btn = button(text("Generate System Report"))
            .on_press(Message::Panel(PanelMessage::Settings(SettingsMessage::GenerateSystemReport)))
            .padding(10);

        let content = column![
            title,
            vertical_space().height(20),
            crate::widgets::card(
                column![
                    text("Appearance").size(18),
                    vertical_space().height(10),
                    theme_row,
                ]
            ),
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Services").size(18),
                    vertical_space().height(10),
                    auto_start_row,
                ]
            ),
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Reports").size(18),
                    vertical_space().height(10),
                    report_btn,
                ]
            ),
        ];

        scrollable(content).into()
    }

    fn search_space_view(&self) -> Element<Message> {
        let title = text(PanelId::SearchSpaceExplorer.name()).size(28);
        
        let state = &self.state.panels.search_space;
        
        // Navigation controls
        let mode_buttons = row![
            button(text("Manual")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetNavigationMode(crate::panels::NavigationMode::Manual)))),
            horizontal_space().width(5),
            button(text("Sequential")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetNavigationMode(crate::panels::NavigationMode::Sequential)))),
            horizontal_space().width(5),
            button(text("Random Walk")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetNavigationMode(crate::panels::NavigationMode::RandomWalk)))),
            horizontal_space().width(5),
            button(text("Directed")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetNavigationMode(crate::panels::NavigationMode::DirectedSearch)))),
        ];
        
        let view_buttons = row![
            button(text("Tree")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetViewMode(crate::panels::ViewMode::Tree)))),
            horizontal_space().width(5),
            button(text("Graph")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetViewMode(crate::panels::ViewMode::Graph)))),
            horizontal_space().width(5),
            button(text("List")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetViewMode(crate::panels::ViewMode::List)))),
            horizontal_space().width(5),
            button(text("5D Projection")).padding(8)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::SetViewMode(crate::panels::ViewMode::Projection5D)))),
        ];
        
        // Current position display
        let position_text = if state.current_position.is_empty() {
            "Position: [Root]".to_string()
        } else {
            format!("Position: {:?}", state.current_position)
        };
        
        // Navigation buttons
        let nav_buttons = row![
            button(text("◀ Back")).padding(10)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::StepBackward))),
            horizontal_space().width(10),
            button(text("Forward ▶")).padding(10)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::StepForward))),
            horizontal_space().width(10),
            button(text("🎯 Jump to High Resonance")).padding(10)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::JumpToHighResonance))),
            horizontal_space().width(10),
            button(text("Clear History")).padding(10)
                .on_press(Message::Panel(PanelMessage::SearchSpace(SearchSpaceMessage::ClearHistory))),
        ];
        
        // History display
        let mut history_list = column![].spacing(5);
        for (idx, pos) in state.history.iter().enumerate().rev().take(10) {
            history_list = history_list.push(
                text(format!("#{}: {} words, resonance: {:.4}", 
                    idx + 1, 
                    pos.words.join(" "),
                    pos.resonance
                )).size(12)
            );
        }
        
        // Visible nodes display
        let mut nodes_list = column![].spacing(5);
        for node in state.visible_nodes.iter().take(20) {
            nodes_list = nodes_list.push(
                crate::widgets::card(
                    column![
                        text(format!("Word: {}", node.word)).size(14),
                        text(format!("Resonance: {:.4}", node.resonance)).size(12),
                        text(format!("Distance: {:.2}", node.distance)).size(12),
                    ]
                )
            );
            nodes_list = nodes_list.push(vertical_space().height(5));
        }
        
        let content = column![
            title,
            vertical_space().height(20),
            crate::widgets::card(
                column![
                    text("Navigation Mode").size(18),
                    vertical_space().height(10),
                    mode_buttons,
                ]
            ),
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("View Mode").size(18),
                    vertical_space().height(10),
                    view_buttons,
                ]
            ),
            vertical_space().height(15),
            text(position_text).size(16),
            vertical_space().height(10),
            nav_buttons,
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Exploration History (Last 10)").size(18),
                    vertical_space().height(10),
                    history_list,
                ]
            ),
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Visible Nodes").size(18),
                    vertical_space().height(10),
                    scrollable(nodes_list).height(300),
                ]
            ),
        ];
        
        scrollable(content).into()
    }

    fn network_explorer_view(&self) -> Element<Message> {
        let title = text(PanelId::NetworkExplorer.name()).size(28);
        
        let state = &self.state.panels.network;
        
        // Layout controls
        let layout_buttons = row![
            button(text("Force-Directed")).padding(8)
                .on_press(Message::Panel(PanelMessage::NetworkExplorer(NetworkExplorerMessage::SetLayoutMode(crate::panels::LayoutMode::ForceDirected)))),
            horizontal_space().width(5),
            button(text("Hierarchical")).padding(8)
                .on_press(Message::Panel(PanelMessage::NetworkExplorer(NetworkExplorerMessage::SetLayoutMode(crate::panels::LayoutMode::Hierarchical)))),
            horizontal_space().width(5),
            button(text("Circular")).padding(8)
                .on_press(Message::Panel(PanelMessage::NetworkExplorer(NetworkExplorerMessage::SetLayoutMode(crate::panels::LayoutMode::Circular)))),
        ];
        
        // Analysis tools
        let analysis_buttons = row![
            button(text("Detect Communities")).padding(10)
                .on_press(Message::Panel(PanelMessage::NetworkExplorer(NetworkExplorerMessage::DetectCommunities))),
            horizontal_space().width(10),
            button(text("Highlight Critical Nodes")).padding(10)
                .on_press(Message::Panel(PanelMessage::NetworkExplorer(NetworkExplorerMessage::HighlightCriticalNodes))),
            horizontal_space().width(10),
            button(text("Export Network")).padding(10)
                .on_press(Message::Panel(PanelMessage::NetworkExplorer(NetworkExplorerMessage::ExportNetwork))),
        ];
        
        // Network info
        let network_info = if let Some(ref network_id) = state.network_id {
            format!("Loaded Network: {}", network_id)
        } else {
            "No network loaded".to_string()
        };
        
        // Communities display
        let mut communities_list = column![].spacing(5);
        for community in state.communities.iter().take(10) {
            communities_list = communities_list.push(
                crate::widgets::card(
                    column![
                        text(format!("Community {}", community.id)).size(14),
                        text(format!("Size: {} nodes", community.size)).size(12),
                        text(format!("Density: {:.3}", community.density)).size(12),
                        text(format!("Avg Resonance: {:.3}", community.avg_resonance)).size(12),
                    ]
                )
            );
            communities_list = communities_list.push(vertical_space().height(5));
        }
        
        // Critical nodes display
        let mut critical_nodes_list = column![].spacing(5);
        for node in state.critical_nodes.iter().take(10) {
            critical_nodes_list = critical_nodes_list.push(
                crate::widgets::card(
                    column![
                        text(format!("Node: {}", node.label)).size(14),
                        text(format!("Type: {}", node.node_type)).size(12),
                        text(format!("Degree Centrality: {:.3}", node.degree_centrality)).size(12),
                        text(format!("Betweenness: {:.3}", node.betweenness_centrality)).size(12),
                    ]
                )
            );
            critical_nodes_list = critical_nodes_list.push(vertical_space().height(5));
        }
        
        let content = column![
            title,
            vertical_space().height(20),
            text(network_info).size(16),
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Layout Mode").size(18),
                    vertical_space().height(10),
                    layout_buttons,
                ]
            ),
            vertical_space().height(15),
            analysis_buttons,
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Communities").size(18),
                    vertical_space().height(10),
                    scrollable(communities_list).height(200),
                ]
            ),
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Critical Nodes").size(18),
                    vertical_space().height(10),
                    scrollable(critical_nodes_list).height(200),
                ]
            ),
        ];
        
        scrollable(content).into()
    }

    fn infogenetic_browser_view(&self) -> Element<Message> {
        let title = text(PanelId::InfogeneticBrowser.name()).size(28);
        
        let state = &self.state.panels.infogenetic;
        
        // Search input
        let search_input = text_input(
            "Search addresses, signatures, clusters...",
            &state.query,
        )
        .on_input(|s| Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::QueryChanged(s))))
        .padding(12);
        
        let search_btn = button(text("🔍 Search")).padding(10)
            .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::Search)));
        
        // Query type selector
        let query_type_buttons = row![
            button(text("Address")).padding(8)
                .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::SetQueryType(crate::panels::QueryType::Address)))),
            horizontal_space().width(5),
            button(text("Spectral Sig")).padding(8)
                .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::SetQueryType(crate::panels::QueryType::SpectralSignature)))),
            horizontal_space().width(5),
            button(text("Cluster")).padding(8)
                .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::SetQueryType(crate::panels::QueryType::Cluster)))),
            horizontal_space().width(5),
            button(text("Full Text")).padding(8)
                .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::SetQueryType(crate::panels::QueryType::FullText)))),
        ];
        
        // Results display
        let results_info = text(format!("Found {} results (showing page {} of {})", 
            state.total_results,
            state.page + 1,
            (state.total_results + state.items_per_page - 1) / state.items_per_page
        )).size(14);
        
        let mut results_list = column![].spacing(8);
        for (idx, entry) in state.results.iter().enumerate() {
            results_list = results_list.push(
                crate::widgets::card(
                    column![
                        text(format!("Address: {}", entry.address)).size(14),
                        text(format!("Chain: {}", entry.chain)).size(12),
                        text(format!("Signature: (ψ={:.3}, ρ={:.3}, ω={:.3})", 
                            entry.signature.0, entry.signature.1, entry.signature.2)).size(12),
                        text(format!("Resonance: {:.4}", entry.resonance)).size(12),
                        text(format!("Discovered: {}", entry.discovered_at.format("%Y-%m-%d %H:%M"))).size(11),
                    ]
                )
            );
        }
        
        // Pagination controls
        let page_controls = row![
            button(text("◀ Previous")).padding(10)
                .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::GoToPage(state.page.saturating_sub(1))))),
            horizontal_space().width(10),
            text(format!("Page {}", state.page + 1)).size(14),
            horizontal_space().width(10),
            button(text("Next ▶")).padding(10)
                .on_press(Message::Panel(PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::GoToPage(state.page + 1)))),
        ];
        
        let content = column![
            title,
            vertical_space().height(20),
            row![
                search_input,
                horizontal_space().width(10),
                search_btn,
            ],
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Query Type").size(18),
                    vertical_space().height(10),
                    query_type_buttons,
                ]
            ),
            vertical_space().height(15),
            results_info,
            vertical_space().height(10),
            scrollable(results_list).height(400),
            vertical_space().height(10),
            page_controls,
        ];
        
        scrollable(content).into()
    }

    fn anomaly_investigation_view(&self) -> Element<Message> {
        let title = text(PanelId::AnomalyInvestigation.name()).size(28);
        
        let state = &self.state.panels.anomaly;
        
        // Anomalies list
        let mut anomalies_list = column![].spacing(8);
        for (idx, anomaly) in state.anomalies.iter().enumerate() {
            let severity_icon = match anomaly.severity {
                crate::panels::AnomalySeverity::Low => "🟢",
                crate::panels::AnomalySeverity::Medium => "🟡",
                crate::panels::AnomalySeverity::High => "🟠",
                crate::panels::AnomalySeverity::Critical => "🔴",
            };
            
            let type_text = match anomaly.anomaly_type {
                crate::panels::AnomalyType::SybilAttack => "Sybil Attack",
                crate::panels::AnomalyType::MoneyLaundering => "Money Laundering",
                crate::panels::AnomalyType::VolumeAnomaly => "Volume Anomaly",
                crate::panels::AnomalyType::TemporalAnomaly => "Temporal Anomaly",
                crate::panels::AnomalyType::StructuralAnomaly => "Structural Anomaly",
                crate::panels::AnomalyType::Other => "Other",
            };
            
            anomalies_list = anomalies_list.push(
                button(
                    crate::widgets::card(
                        column![
                            text(format!("{} {} - Score: {:.2}", severity_icon, type_text, anomaly.score)).size(14),
                            text(format!("Affected: {} entities", anomaly.affected_entities.len())).size(12),
                            text(format!("{}", anomaly.description)).size(12),
                            text(format!("Detected: {}", anomaly.detected_at.format("%Y-%m-%d %H:%M"))).size(11),
                        ]
                    )
                )
                .on_press(Message::Panel(PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::SelectAnomaly(Some(idx)))))
                .padding(0)
            );
        }
        
        // Investigation tools
        let investigation_tools = row![
            button(text("Find Related Entities")).padding(10)
                .on_press(Message::Panel(PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::FindRelatedEntities))),
            horizontal_space().width(10),
            button(text("Load Timeline")).padding(10)
                .on_press(Message::Panel(PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::LoadTimeline))),
            horizontal_space().width(10),
            button(text("Find Similar Cases")).padding(10)
                .on_press(Message::Panel(PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::FindSimilarCases))),
        ];
        
        // Timeline display if selected
        let mut timeline_display = column![].spacing(5);
        if state.selected.is_some() {
            for entry in state.timeline.iter().take(10) {
                timeline_display = timeline_display.push(
                    text(format!("{}: {}", 
                        entry.timestamp.format("%H:%M:%S"),
                        entry.event
                    )).size(12)
                );
            }
        }
        
        let content = column![
            title,
            vertical_space().height(20),
            crate::widgets::card(
                column![
                    text(format!("Detected Anomalies: {}", state.anomalies.len())).size(18),
                    vertical_space().height(10),
                    scrollable(anomalies_list).height(300),
                ]
            ),
            vertical_space().height(15),
            investigation_tools,
            vertical_space().height(15),
            crate::widgets::card(
                column![
                    text("Timeline").size(18),
                    vertical_space().height(10),
                    if state.selected.is_some() {
                        scrollable(timeline_display).height(200)
                    } else {
                        scrollable(text("Select an anomaly to view timeline").size(12)).height(200)
                    },
                ]
            ),
        ];
        
        scrollable(content).into()
    }

    fn forensic_workflows_view(&self) -> Element<Message> {
        let title = text(PanelId::ForensicWorkflows.name()).size(28);
        
        let state = &self.state.panels.forensic;
        
        // Available workflows
        let mut workflows_list = column![].spacing(10);
        for workflow in state.workflows.iter() {
            workflows_list = workflows_list.push(
                button(
                    crate::widgets::card(
                        column![
                            text(&workflow.name).size(16),
                            vertical_space().height(5),
                            text(&workflow.description).size(12),
                            text(format!("Steps: {} | Duration: {}", workflow.steps.len(), workflow.estimated_duration)).size(11),
                        ]
                    )
                )
                .on_press(Message::Panel(PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::StartWorkflow(workflow.id.clone()))))
                .padding(0)
            );
        }
        
        // Active workflow display
        let active_workflow_display = if let Some(ref active) = state.active_workflow {
            let mut steps_list = column![].spacing(5);
            for (idx, step) in active.steps.iter().enumerate() {
                let status_icon = if step.completed {
                    "✅"
                } else if idx == active.current_step {
                    "▶"
                } else {
                    "⏳"
                };
                
                steps_list = steps_list.push(
                    text(format!("{} Step {}: {}", status_icon, step.number, step.title)).size(14)
                );
            }
            
            column![
                text(format!("Active Workflow (Step {}/{})", active.current_step + 1, active.steps.len())).size(18),
                vertical_space().height(10),
                steps_list,
                vertical_space().height(15),
                row![
                    button(text("◀ Previous")).padding(10)
                        .on_press(Message::Panel(PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::PreviousStep))),
                    horizontal_space().width(10),
                    button(text("Complete Step ✓")).padding(10)
                        .on_press(Message::Panel(PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::CompleteStep))),
                    horizontal_space().width(10),
                    button(text("Cancel")).padding(10)
                        .on_press(Message::Panel(PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::CancelWorkflow))),
                ],
            ]
        } else {
            column![
                text("No active workflow").size(16),
            ]
        };
        
        let content = column![
            title,
            vertical_space().height(20),
            crate::widgets::card(
                column![
                    text("Available Workflows").size(18),
                    vertical_space().height(10),
                    scrollable(workflows_list).height(250),
                ]
            ),
            vertical_space().height(15),
            crate::widgets::card(active_workflow_display),
        ];
        
        scrollable(content).into()
    }

    fn stat_card<'a>(&self, label: &'a str, value: &'a str) -> Element<'a, Message> {
        crate::widgets::card(
            column![
                text(label).size(14),
                vertical_space().height(8),
                text(value).size(32),
            ]
        )
    }

    fn add_log(&mut self, level: LogLevel, source: &str, message: &str) {
        self.state.add_log(LogEntry {
            timestamp: Utc::now(),
            level,
            source: source.to_string(),
            message: message.to_string(),
        });
    }

    fn handle_panel_message(&mut self, message: PanelMessage) -> Task<Message> {
        match message {
            PanelMessage::SwitchTo(panel_id) => {
                self.state.active_panel = panel_id;
                self.add_log(LogLevel::Debug, "UI", &format!("Switched to {}", panel_id.name()));
            }
            PanelMessage::SeedManagement(SeedMessage::InputChanged(input)) => {
                self.state.panels.seed_management.input = input;
            }
            PanelMessage::SeedManagement(SeedMessage::ImportSeed) => {
                // Real wallet integration
                let input = self.state.panels.seed_management.input.clone();
                match WalletIntegration::import_mnemonic(&input) {
                    Ok(seed_info) => {
                        let addr_count = seed_info.addresses.len();
                        self.state.panels.seed_management.seeds.push(seed_info);
                        self.state.panels.home.seeds_count = self.state.panels.seed_management.seeds.len();
                        self.add_log(LogLevel::Info, "Wallet", &format!("Imported seed with {} addresses", addr_count));
                        self.state.panels.seed_management.input.clear();
                    }
                    Err(e) => {
                        self.add_log(LogLevel::Error, "Wallet", &format!("Import failed: {}", e));
                    }
                }
            }
            PanelMessage::SeedManagement(SeedMessage::Clear) => {
                self.state.panels.seed_management.input.clear();
            }
            PanelMessage::Resonance(ResonanceMessage::StartAnalysis) => {
                let was_running = self.state.panels.resonance.running;
                self.state.panels.resonance.running = !was_running;
                
                if !was_running {
                    // Start analysis using real resonance integration
                    if let Some(ref mut integration) = self.resonance_integration {
                        // Simulate analysis with random points
                        let perception = [0.5, 0.5, 0.5, 0.5, 0.5];
                        let intention = [0.6, 0.6, 0.6, 0.6, 0.6];
                        
                        match integration.analyze_point(perception, intention) {
                            Ok(result) => {
                                self.state.panels.resonance.best_resonance = result.best_resonance;
                                self.state.panels.resonance.resonance_history.push(result.signature);
                                self.add_log(LogLevel::Info, "Resonance", &format!("Best resonance: {:.6}", result.best_resonance));
                            }
                            Err(e) => {
                                self.add_log(LogLevel::Error, "Resonance", &format!("Analysis failed: {}", e));
                            }
                        }
                    }
                }
                
                self.add_log(LogLevel::Info, "Resonance", if self.state.panels.resonance.running { "Started" } else { "Stopped" });
            }
            PanelMessage::Cluster(ClusterMessage::Search(query)) => {
                self.state.panels.cluster.search_query = query;
            }
            PanelMessage::Cluster(ClusterMessage::ExportAllJson) => {
                self.export_clusters(ExportFormat::Json);
            }
            PanelMessage::Cluster(ClusterMessage::ExportAllCsv) => {
                self.export_clusters(ExportFormat::Csv);
            }
            PanelMessage::Cluster(ClusterMessage::ExportAllMarkdown) => {
                self.export_clusters(ExportFormat::Markdown);
            }
            PanelMessage::SeedManagement(SeedMessage::ExportAllJson) => {
                self.export_seeds(ExportFormat::Json);
            }
            PanelMessage::SeedManagement(SeedMessage::ExportAllCsv) => {
                self.export_seeds(ExportFormat::Csv);
            }
            PanelMessage::SeedManagement(SeedMessage::ExportAllMarkdown) => {
                self.export_seeds(ExportFormat::Markdown);
            }
            PanelMessage::Settings(SettingsMessage::GenerateSystemReport) => {
                self.generate_system_report();
            }
            PanelMessage::Settings(SettingsMessage::ToggleAutoStart) => {
                self.config.services.auto_start = !self.config.services.auto_start;
                self.add_log(LogLevel::Info, "Settings", &format!("Auto-start: {}", self.config.services.auto_start));
            }
            PanelMessage::Log(LogMessage::Clear) => {
                self.state.clear_logs();
                self.add_log(LogLevel::Info, "Log", "Logs cleared");
            }
            PanelMessage::Log(LogMessage::FilterChanged(filter)) => {
                self.state.panels.log.filter = filter;
            }
            PanelMessage::Settings(SettingsMessage::ToggleTheme) => {
                self.state.dark_mode = !self.state.dark_mode;
                self.theme = PhosphorosTheme::new(self.state.dark_mode);
                self.add_log(LogLevel::Info, "Settings", &format!("Theme: {}", if self.state.dark_mode { "Dark" } else { "Light" }));
            }
            PanelMessage::Stealth(StealthMessage::ToggleEnabled) => {
                self.state.panels.stealth.enabled = !self.state.panels.stealth.enabled;
                self.add_log(LogLevel::Info, "Stealth", &format!("Stealth mode: {}", if self.state.panels.stealth.enabled { "Enabled" } else { "Disabled" }));
            }
            PanelMessage::Stealth(StealthMessage::SetMode(mode)) => {
                self.state.panels.stealth.mode = mode;
                self.add_log(LogLevel::Info, "Stealth", &format!("Mode set to: {:?}", mode));
            }
            PanelMessage::Stealth(StealthMessage::ToggleProxy) => {
                self.state.panels.stealth.proxy_enabled = !self.state.panels.stealth.proxy_enabled;
                self.add_log(LogLevel::Info, "Stealth", &format!("Proxy rotation: {}", if self.state.panels.stealth.proxy_enabled { "Enabled" } else { "Disabled" }));
            }
            PanelMessage::Stealth(StealthMessage::ToggleJitter) => {
                self.state.panels.stealth.jitter_enabled = !self.state.panels.stealth.jitter_enabled;
                self.add_log(LogLevel::Info, "Stealth", &format!("Temporal jitter: {}", if self.state.panels.stealth.jitter_enabled { "Enabled" } else { "Disabled" }));
            }
            PanelMessage::Stealth(StealthMessage::ToggleLogging) => {
                self.state.panels.stealth.logging_enabled = !self.state.panels.stealth.logging_enabled;
                self.add_log(LogLevel::Info, "Stealth", &format!("Request logging: {}", if self.state.panels.stealth.logging_enabled { "Enabled" } else { "Disabled" }));
            }
            PanelMessage::Stealth(StealthMessage::TaskCompleted { task_id, success }) => {
                if success {
                    self.add_log(LogLevel::Info, "Stealth", &format!("Task {} completed successfully", task_id));
                } else {
                    self.add_log(LogLevel::Warning, "Stealth", &format!("Task {} failed", task_id));
                }
            }
            // Search Space Explorer handlers
            PanelMessage::SearchSpace(SearchSpaceMessage::SetNavigationMode(mode)) => {
                self.state.panels.search_space.mode = mode;
                self.add_log(LogLevel::Info, "SearchSpace", &format!("Navigation mode changed to {:?}", mode));
            }
            PanelMessage::SearchSpace(SearchSpaceMessage::SetViewMode(view_mode)) => {
                self.state.panels.search_space.view_mode = view_mode;
                self.add_log(LogLevel::Info, "SearchSpace", &format!("View mode changed to {:?}", view_mode));
            }
            PanelMessage::SearchSpace(SearchSpaceMessage::StepForward) => {
                self.add_log(LogLevel::Info, "SearchSpace", "Stepping forward in search space");
                // Generate some visible nodes for demonstration
                use crate::panels::SearchNode;
                let sample_words = vec!["abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd", "abuse"];
                let mut nodes = Vec::new();
                for (idx, word) in sample_words.iter().enumerate() {
                    nodes.push(SearchNode {
                        id: format!("node_{}", idx),
                        word: word.to_string(),
                        index: idx,
                        resonance: (idx as f64 * 0.07) % 1.0,
                        distance: idx as f64 * 0.5,
                        children_count: 2048,
                    });
                }
                self.state.panels.search_space.visible_nodes = nodes;
            }
            PanelMessage::SearchSpace(SearchSpaceMessage::StepBackward) => {
                if !self.state.panels.search_space.history.is_empty() {
                    self.state.panels.search_space.history.pop();
                    self.add_log(LogLevel::Info, "SearchSpace", "Stepped backward");
                }
            }
            PanelMessage::SearchSpace(SearchSpaceMessage::ClearHistory) => {
                self.state.panels.search_space.history.clear();
                self.add_log(LogLevel::Info, "SearchSpace", "History cleared");
            }
            PanelMessage::SearchSpace(SearchSpaceMessage::JumpToHighResonance) => {
                self.add_log(LogLevel::Info, "SearchSpace", "Jumping to high resonance region");
                // Add a high-resonance position to history
                use crate::panels::SearchPosition;
                let position = SearchPosition {
                    indices: vec![0, 1, 2],
                    words: vec!["abandon".to_string(), "ability".to_string(), "able".to_string()],
                    resonance: 0.95,
                    visited_at: chrono::Utc::now(),
                };
                self.state.panels.search_space.history.push(position);
            }
            // Network Explorer handlers
            PanelMessage::NetworkExplorer(NetworkExplorerMessage::SetLayoutMode(layout)) => {
                self.state.panels.network.layout_mode = layout;
                self.add_log(LogLevel::Info, "NetworkExplorer", &format!("Layout mode changed to {:?}", layout));
            }
            PanelMessage::NetworkExplorer(NetworkExplorerMessage::DetectCommunities) => {
                self.add_log(LogLevel::Info, "NetworkExplorer", "Detecting communities...");
                // Generate sample communities for demonstration
                use crate::panels::CommunityInfo;
                let communities = vec![
                    CommunityInfo {
                        id: "community_1".to_string(),
                        size: 12,
                        density: 0.73,
                        avg_resonance: 0.68,
                    },
                    CommunityInfo {
                        id: "community_2".to_string(),
                        size: 8,
                        density: 0.85,
                        avg_resonance: 0.82,
                    },
                    CommunityInfo {
                        id: "community_3".to_string(),
                        size: 5,
                        density: 0.92,
                        avg_resonance: 0.71,
                    },
                ];
                self.state.panels.network.communities = communities;
            }
            PanelMessage::NetworkExplorer(NetworkExplorerMessage::HighlightCriticalNodes) => {
                self.add_log(LogLevel::Info, "NetworkExplorer", "Highlighting critical nodes...");
                // Generate sample critical nodes for demonstration
                use crate::panels::NodeInfo;
                let critical_nodes = vec![
                    NodeInfo {
                        id: "node_001".to_string(),
                        label: "0xabcd...1234".to_string(),
                        node_type: "Validator".to_string(),
                        degree_centrality: 0.89,
                        betweenness_centrality: 0.76,
                        resonance: 0.84,
                    },
                    NodeInfo {
                        id: "node_002".to_string(),
                        label: "0x5678...9abc".to_string(),
                        node_type: "Bridge".to_string(),
                        degree_centrality: 0.72,
                        betweenness_centrality: 0.91,
                        resonance: 0.68,
                    },
                    NodeInfo {
                        id: "node_003".to_string(),
                        label: "0xdef0...5432".to_string(),
                        node_type: "Hub".to_string(),
                        degree_centrality: 0.95,
                        betweenness_centrality: 0.64,
                        resonance: 0.77,
                    },
                ];
                self.state.panels.network.critical_nodes = critical_nodes;
            }
            PanelMessage::NetworkExplorer(NetworkExplorerMessage::ExportNetwork) => {
                self.add_log(LogLevel::Info, "NetworkExplorer", "Exporting network data");
                // TODO: Implement network export
            }
            // Infogenetic Browser handlers
            PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::QueryChanged(query)) => {
                self.state.panels.infogenetic.query = query;
            }
            PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::SetQueryType(query_type)) => {
                self.state.panels.infogenetic.query_type = query_type;
                self.add_log(LogLevel::Info, "InfogeneticBrowser", &format!("Query type changed to {:?}", query_type));
            }
            PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::Search) => {
                self.add_log(LogLevel::Info, "InfogeneticBrowser", &format!("Searching: {}", self.state.panels.infogenetic.query));
                // TODO: Implement search logic
            }
            PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::GoToPage(page)) => {
                self.state.panels.infogenetic.page = page;
            }
            PanelMessage::InfogeneticBrowser(InfogeneticBrowserMessage::SelectEntry(entry)) => {
                self.state.panels.infogenetic.selected_entry = entry;
            }
            // Anomaly Investigation handlers
            PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::SelectAnomaly(anomaly)) => {
                self.state.panels.anomaly.selected = anomaly;
                if let Some(idx) = anomaly {
                    if let Some(a) = self.state.panels.anomaly.anomalies.get(idx) {
                        self.add_log(LogLevel::Info, "AnomalyInvestigation", &format!("Selected anomaly: {}", a.id));
                    }
                }
            }
            PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::FindRelatedEntities) => {
                self.add_log(LogLevel::Info, "AnomalyInvestigation", "Finding related entities...");
                // TODO: Implement related entity search
            }
            PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::LoadTimeline) => {
                self.add_log(LogLevel::Info, "AnomalyInvestigation", "Loading timeline...");
                // TODO: Implement timeline loading
            }
            PanelMessage::AnomalyInvestigation(AnomalyInvestigationMessage::FindSimilarCases) => {
                self.add_log(LogLevel::Info, "AnomalyInvestigation", "Finding similar cases...");
                // TODO: Implement similar case search
            }
            // Forensic Workflows handlers
            PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::StartWorkflow(id)) => {
                self.add_log(LogLevel::Info, "ForensicWorkflows", &format!("Starting workflow: {}", id));
                // TODO: Implement workflow start logic
            }
            PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::CancelWorkflow) => {
                self.state.panels.forensic.active_workflow = None;
                self.add_log(LogLevel::Info, "ForensicWorkflows", "Workflow cancelled");
            }
            PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::CompleteStep) => {
                self.add_log(LogLevel::Info, "ForensicWorkflows", "Completing current step");
                // TODO: Implement step completion logic
            }
            PanelMessage::ForensicWorkflows(ForensicWorkflowsMessage::PreviousStep) => {
                if let Some(ref mut active) = self.state.panels.forensic.active_workflow {
                    if active.current_step > 0 {
                        active.current_step -= 1;
                        self.add_log(LogLevel::Info, "ForensicWorkflows", "Moved to previous step");
                    }
                }
            }
            _ => {}
        }
        Task::none()
    }
}

impl Default for PhosphorosApp {
    fn default() -> Self {
        Self::new().0
    }
}

impl std::fmt::Debug for ResonanceIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResonanceIntegration").finish()
    }
}

impl std::fmt::Debug for AnalysisIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalysisIntegration").finish()
    }
}

impl std::fmt::Debug for TaskManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskManager").finish()
    }
}

// Additional helper methods
impl PhosphorosApp {
    /// Spawn background tasks
    fn spawn_background_tasks(&mut self) {
        if let Some(ref task_manager) = self.task_manager {
            let service_mgr = Arc::new(self.state.service_manager.clone());
            
            // Spawn all three background services
            task_manager.spawn_scraper(service_mgr.clone());
            task_manager.spawn_analyzer(service_mgr.clone());
            task_manager.spawn_cluster_engine(service_mgr.clone());
            
            self.add_log(LogLevel::Info, "Tasks", "Background tasks spawned");
        }
    }
    
    /// Handle task messages from background services
    fn handle_task_message(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::EntityDiscovered(entity) => {
                self.add_log(LogLevel::Debug, "Scraper", &format!("Entity discovered: {}", entity.address));
            }
            TaskMessage::AnomalyDetected(anomaly) => {
                self.add_log(LogLevel::Warning, "Analyzer", &format!("Anomaly detected: score={:.2}", anomaly.score));
            }
            TaskMessage::ClusterFound(cluster) => {
                self.add_log(LogLevel::Info, "ClusterEngine", &format!("Cluster found: {} members, resonance={:.3}", cluster.members.len(), cluster.resonance));
            }
            TaskMessage::ServiceTick => {
                // Handle periodic updates
            }
            TaskMessage::TaskCompleted { id, success } => {
                let status = if success { "completed" } else { "failed" };
                self.add_log(LogLevel::Info, "Tasks", &format!("Task {} {}", id, status));
            }
        }
    }
    
    /// Export clusters to file
    fn export_clusters(&mut self, format: ExportFormat) {
        use dirs::home_dir;
        
        let home = home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let filename = match format {
            ExportFormat::Json => "phosphoros_clusters.json",
            ExportFormat::Csv => "phosphoros_clusters.csv",
            ExportFormat::Markdown => "phosphoros_clusters.md",
        };
        let path = home.join(filename);
        
        match ExportService::export_clusters(&self.state.panels.cluster.clusters, &path, format) {
            Ok(_) => {
                self.add_log(LogLevel::Info, "Export", &format!("Clusters exported to {:?}", path));
            }
            Err(e) => {
                self.add_log(LogLevel::Error, "Export", &format!("Export failed: {}", e));
            }
        }
    }
    
    /// Export seeds to file
    fn export_seeds(&mut self, format: ExportFormat) {
        use dirs::home_dir;
        
        let home = home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let filename = match format {
            ExportFormat::Json => "phosphoros_seeds.json",
            ExportFormat::Csv => "phosphoros_seeds.csv",
            ExportFormat::Markdown => "phosphoros_seeds.md",
        };
        let path = home.join(filename);
        
        match ExportService::export_seeds(&self.state.panels.seed_management.seeds, &path, format) {
            Ok(_) => {
                self.add_log(LogLevel::Info, "Export", &format!("Seeds exported to {:?}", path));
            }
            Err(e) => {
                self.add_log(LogLevel::Error, "Export", &format!("Export failed: {}", e));
            }
        }
    }
    
    /// Generate comprehensive system report
    fn generate_system_report(&mut self) {
        use dirs::home_dir;
        
        let home = home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let path = home.join("phosphoros_system_report.md");
        
        let stats = SystemStats {
            entities_scraped: self.state.service_manager.scraper.read().processed,
            anomalies_detected: self.state.service_manager.data_pool.read().anomalies.len(),
            analysis_runs: self.state.panels.resonance.resonance_history.len(),
            scraper_running: self.state.service_manager.scraper.read().running,
            analyzer_running: self.state.service_manager.analyzer.read().running,
            cluster_engine_running: self.state.service_manager.cluster_engine.read().running,
        };
        
        match ExportService::generate_system_report(
            &self.state.panels.seed_management.seeds,
            &self.state.panels.cluster.clusters,
            &stats,
            &path,
        ) {
            Ok(_) => {
                self.add_log(LogLevel::Info, "Export", &format!("System report generated: {:?}", path));
            }
            Err(e) => {
                self.add_log(LogLevel::Error, "Export", &format!("Report generation failed: {}", e));
            }
        }
    }
}
