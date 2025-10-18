//! Example demonstrating programmatic interaction with dashboard state
//! This shows how the dashboard can be controlled and monitored programmatically

fn main() {
    println!("=== PHOSPHOROS Dashboard Architecture Demo ===\n");

    // Demonstrate state structure
    demo_state();
    
    // Demonstrate message system
    demo_messages();
    
    // Demonstrate service management
    demo_services();
    
    // Demonstrate panel system
    demo_panels();
}

fn demo_state() {
    println!("📊 STATE MANAGEMENT");
    println!("===================");
    println!("The dashboard uses a centralized AppState structure:");
    println!("  - active_panel: Currently displayed panel");
    println!("  - panels: State for all 6 panels");
    println!("  - service_manager: Controls 3 autonomous services");
    println!("  - notifications: Queue of up to 10 notifications");
    println!("  - logs: Ring buffer of up to 1000 log entries");
    println!("  - dark_mode: Current theme setting");
    println!();
}

fn demo_messages() {
    println!("💬 MESSAGE SYSTEM");
    println!("==================");
    println!("All interactions flow through typed messages:");
    println!();
    println!("Message::Panel(PanelMessage::SwitchTo(PanelId::Home))");
    println!("  → Switches to Home panel");
    println!();
    println!("Message::Panel(PanelMessage::Resonance(ResonanceMessage::StartAnalysis))");
    println!("  → Starts resonance analysis");
    println!();
    println!("Message::Service(ServiceMessage::Scraper(ScraperMessage::Pause))");
    println!("  → Pauses scraper service");
    println!();
    println!("Message::System(SystemMessage::Tick)");
    println!("  → Periodic update (every 1 second)");
    println!();
}

fn demo_services() {
    println!("⚙️  AUTONOMOUS SERVICES");
    println!("=======================");
    println!("Three independent background services:");
    println!();
    println!("1. SCRAPER SERVICE");
    println!("   Purpose: Crawl seedspace, collect data");
    println!("   Status: running=true/false, processed=0");
    println!("   Control: Start/Pause/Resume");
    println!();
    println!("2. ANALYZER SERVICE");
    println!("   Purpose: Pattern recognition, anomaly detection");
    println!("   Status: running=true/false, analyzed=0");
    println!("   Control: Start/Pause");
    println!();
    println!("3. CLUSTER ENGINE");
    println!("   Purpose: KNN graphs, resonance hotspots");
    println!("   Status: running=true/false, clusters=0");
    println!("   Control: Start/Pause");
    println!();
    println!("All services:");
    println!("  ✅ Thread-safe with parking_lot::RwLock");
    println!("  ✅ Independent operation");
    println!("  ✅ Persistent state");
    println!();
}

fn demo_panels() {
    println!("🎨 PANEL SYSTEM");
    println!("================");
    println!();
    println!("Panel Navigation:");
    for panel in &[
        ("Home", "🏠", "Live Overview - System status & metrics"),
        ("SeedManagement", "🔑", "Import & manage crypto seeds"),
        ("Resonance", "📊", "5D resonance analysis & operators"),
        ("ClusterExplorer", "🔍", "Search & explore clusters"),
        ("SystemLog", "📝", "Filterable system logs"),
        ("Settings", "⚙️", "Configure theme & services"),
    ] {
        println!("  {} {} - {}", panel.1, panel.0, panel.2);
    }
    println!();
    
    println!("Each panel has its own state:");
    println!("  - HomeState: active_tasks, seeds_count, clusters_count, coverage");
    println!("  - SeedManagementState: input, seeds[], selected");
    println!("  - ResonanceState: running, progress, best_resonance, operators");
    println!("  - ClusterState: search_query, clusters[], selected");
    println!("  - LogState: filter, auto_scroll");
    println!("  - SettingsState: dark_mode, language, task_priorities");
    println!();
}
