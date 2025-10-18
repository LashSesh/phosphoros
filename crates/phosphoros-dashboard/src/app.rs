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
