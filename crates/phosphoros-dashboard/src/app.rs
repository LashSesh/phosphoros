//! Simple functional app implementation for phosphoros-dashboard
//! This is a minimal working version that can be extended

use crate::config::Config;
use crate::messages::*;
use crate::panels::PanelId;
use crate::state::{AppState, LogEntry, LogLevel, Notification, NotificationKind};
use crate::theme::PhosphorosTheme;
use chrono::Utc;
use iced::widget::{button, column, container, row, scrollable, text, text_input, toggler, progress_bar, horizontal_rule, horizontal_space, vertical_space};
use iced::{Element, Length, Subscription, Task, Theme};
use std::time::Duration;

/// Main application
#[derive(Debug)]
pub struct PhosphorosApp {
    state: AppState,
    config: Config,
    theme: PhosphorosTheme,
    notification_counter: usize,
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
        };

        app.add_log(LogLevel::Info, "System", "PHOSPHOROS Dashboard initialized");

        if app.config.services.auto_start {
            app.state.service_manager.start_all();
            app.add_log(LogLevel::Info, "Services", "Auto-starting all services");
        }

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Panel(panel_msg) => self.handle_panel_message(panel_msg),
            Message::Service(_) => Task::none(),
            Message::UI(_) => Task::none(),
            Message::System(SystemMessage::Tick) => Task::none(),
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
        
        // Create stat cards inline to avoid lifetime issues
        let stats = row![
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
                    text(self.state.panels.home.clusters_count.to_string()).size(32),
                ]
            ),
            horizontal_space().width(16),
            crate::widgets::card(
                column![
                    text("Coverage").size(14),
                    vertical_space().height(8),
                    text(format!("{:.1}%", self.state.panels.home.coverage)).size(32),
                ]
            ),
        ];

        let status_card = crate::widgets::card(
            column![
                text("System Status").size(18),
                vertical_space().height(10),
                text(format!("Scraper: {}", if self.state.service_manager.scraper.read().running { "Running" } else { "Paused" })).size(14),
                text(format!("Analyzer: {}", if self.state.service_manager.analyzer.read().running { "Running" } else { "Paused" })).size(14),
                text(format!("Cluster Engine: {}", if self.state.service_manager.cluster_engine.read().running { "Running" } else { "Paused" })).size(14),
            ]
        );

        column![
            title,
            vertical_space().height(20),
            stats,
            vertical_space().height(20),
            status_card,
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

        let mut content = column![
            title,
            vertical_space().height(20),
            search,
        ];

        if self.state.panels.cluster.clusters.is_empty() {
            content = content.push(vertical_space().height(20));
            content = content.push(text("No clusters found. Start analysis to discover clusters.").size(14));
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

    fn settings_view(&self) -> Element<Message> {
        let title = text(PanelId::Settings.name()).size(28);

        let theme_row = row![
            text("Dark Mode").size(14),
            horizontal_space().width(10),
            toggler(self.state.dark_mode)
                .on_toggle(|_| Message::Panel(PanelMessage::Settings(SettingsMessage::ToggleTheme))),
        ];

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
            PanelMessage::SeedManagement(SeedMessage::Clear) => {
                self.state.panels.seed_management.input.clear();
            }
            PanelMessage::Resonance(ResonanceMessage::StartAnalysis) => {
                self.state.panels.resonance.running = !self.state.panels.resonance.running;
                self.add_log(LogLevel::Info, "Resonance", if self.state.panels.resonance.running { "Started" } else { "Stopped" });
            }
            PanelMessage::Cluster(ClusterMessage::Search(query)) => {
                self.state.panels.cluster.search_query = query;
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
