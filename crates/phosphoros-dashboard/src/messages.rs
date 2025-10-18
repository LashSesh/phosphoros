//! Application messages for state updates

use crate::panels::PanelId;
use std::path::PathBuf;

/// Main application message types following Elm architecture
#[derive(Debug, Clone)]
pub enum Message {
    /// Panel-related messages
    Panel(PanelMessage),

    /// Service-related messages
    Service(ServiceMessage),

    /// UI interaction messages
    UI(UIMessage),

    /// System messages
    System(SystemMessage),

    /// Notification messages
    Notification(NotificationMessage),
}

/// Panel-specific messages
#[derive(Debug, Clone)]
pub enum PanelMessage {
    /// Switch active panel
    SwitchTo(PanelId),

    /// Home panel messages
    Home(HomeMessage),

    /// Seed management messages
    SeedManagement(SeedMessage),

    /// Resonance panel messages
    Resonance(ResonanceMessage),

    /// Cluster explorer messages
    Cluster(ClusterMessage),

    /// Search space explorer messages
    SearchSpace(SearchSpaceMessage),

    /// Network explorer messages
    NetworkExplorer(NetworkExplorerMessage),

    /// Infogenetic browser messages
    InfogeneticBrowser(InfogeneticBrowserMessage),

    /// Anomaly investigation messages
    AnomalyInvestigation(AnomalyInvestigationMessage),

    /// Forensic workflows messages
    ForensicWorkflows(ForensicWorkflowsMessage),

    /// Stealth/Privacy messages
    Stealth(StealthMessage),

    /// Log panel messages
    Log(LogMessage),

    /// Settings panel messages
    Settings(SettingsMessage),
}

/// Home panel messages
#[derive(Debug, Clone)]
pub enum HomeMessage {
    /// Refresh status
    Refresh,
    /// Update received from services
    StatusUpdate {
        /// Active tasks count
        active_tasks: usize,
        /// Seeds collected
        seeds_count: usize,
        /// Clusters discovered
        clusters_count: usize,
    },
}

/// Seed management messages
#[derive(Debug, Clone)]
pub enum SeedMessage {
    /// Input seed/mnemonic text changed
    InputChanged(String),
    /// Import seed from input
    ImportSeed,
    /// Import from file
    ImportFile(PathBuf),
    /// Clear seed input
    Clear,
    /// Copy address to clipboard
    CopyAddress(String),
    /// Seed imported successfully
    SeedImported {
        /// Number of addresses generated
        addresses: usize,
    },
    /// Export all seeds to JSON
    ExportAllJson,
    /// Export all seeds to CSV
    ExportAllCsv,
    /// Export all seeds to Markdown
    ExportAllMarkdown,
}

/// Resonance analysis messages
#[derive(Debug, Clone)]
pub enum ResonanceMessage {
    /// Start resonance analysis
    StartAnalysis,
    /// Analysis progress update
    Progress {
        /// Current step
        step: usize,
        /// Total steps
        total: usize,
        /// Current resonance
        resonance: f64,
    },
    /// Analysis completed
    Completed {
        /// Best resonance found
        best_resonance: f64,
        /// Pruning factor
        pruning_factor: f64,
    },
    /// Operator configuration changed
    ConfigureOperator {
        /// Operator name
        operator: String,
        /// Parameter value
        value: f64,
    },
}

/// Cluster explorer messages
#[derive(Debug, Clone)]
pub enum ClusterMessage {
    /// Search clusters
    Search(String),
    /// Cluster found
    ClusterFound {
        /// Cluster ID
        id: String,
        /// Members count
        members: usize,
        /// Resonance magnitude
        resonance: f64,
    },
    /// Export cluster
    Export(String),
    /// Export all clusters to JSON
    ExportAllJson,
    /// Export all clusters to CSV
    ExportAllCsv,
    /// Export all clusters to Markdown
    ExportAllMarkdown,
}

/// Log panel messages
#[derive(Debug, Clone)]
pub enum LogMessage {
    /// Clear logs
    Clear,
    /// Filter changed
    FilterChanged(String),
    /// New log entry
    NewEntry {
        /// Timestamp
        timestamp: chrono::DateTime<chrono::Utc>,
        /// Log level
        level: LogLevel,
        /// Message
        message: String,
    },
}

/// Stealth/Privacy panel messages
#[derive(Debug, Clone)]
pub enum StealthMessage {
    /// Toggle stealth mode enabled/disabled
    ToggleEnabled,
    /// Set stealth mode
    SetMode(crate::panels::StealthMode),
    /// Set preferred API type
    SetPreferredApi(Option<crate::panels::ApiType>),
    /// Toggle proxy rotation
    ToggleProxy,
    /// Add proxy configuration
    AddProxy {
        /// Proxy host
        host: String,
        /// Proxy port
        port: u16,
    },
    /// Remove proxy
    RemoveProxy(usize),
    /// Toggle request logging
    ToggleLogging,
    /// Toggle temporal jitter
    ToggleJitter,
    /// Update jitter settings
    UpdateJitter {
        /// Minimum jitter (ms)
        min_ms: u64,
        /// Maximum jitter (ms)
        max_ms: u64,
    },
    /// Stealth task completed
    TaskCompleted {
        /// Task ID
        task_id: String,
        /// Success status
        success: bool,
    },
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
}

/// Settings panel messages
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// Toggle theme
    ToggleTheme,
    /// Change language
    ChangeLanguage(String),
    /// Task priority changed
    TaskPriority {
        /// Task name
        task: String,
        /// Priority level
        priority: usize,
    },
    /// Generate system report
    GenerateSystemReport,
    /// Toggle service auto-start
    ToggleAutoStart,
}

/// Service messages
#[derive(Debug, Clone)]
pub enum ServiceMessage {
    /// Scraper service messages
    Scraper(ScraperMessage),
    /// Analyzer service messages
    Analyzer(AnalyzerMessage),
    /// Cluster engine messages
    ClusterEngine(ClusterEngineMessage),
}

/// Scraper service messages
#[derive(Debug, Clone)]
pub enum ScraperMessage {
    /// Start scraper
    Start,
    /// Pause scraper
    Pause,
    /// Resume scraper
    Resume,
    /// Status update
    Status {
        /// Running status
        running: bool,
        /// Items processed
        processed: usize,
    },
}

/// Analyzer service messages
#[derive(Debug, Clone)]
pub enum AnalyzerMessage {
    /// Start analyzer
    Start,
    /// Pause analyzer
    Pause,
    /// Status update
    Status {
        /// Running status
        running: bool,
        /// Items analyzed
        analyzed: usize,
    },
}

/// Cluster engine messages
#[derive(Debug, Clone)]
pub enum ClusterEngineMessage {
    /// Start engine
    Start,
    /// Pause engine
    Pause,
    /// Status update
    Status {
        /// Running status
        running: bool,
        /// Clusters found
        clusters: usize,
    },
}

/// UI interaction messages
#[derive(Debug, Clone)]
pub enum UIMessage {
    /// Window resized
    WindowResized {
        /// New width
        width: f32,
        /// New height
        height: f32,
    },
    /// Sidebar toggled
    ToggleSidebar,
}

/// System messages
#[derive(Debug, Clone)]
pub enum SystemMessage {
    /// Tick for periodic updates
    Tick,
    /// Background task completed
    TaskCompleted {
        /// Task ID
        task_id: String,
        /// Success status
        success: bool,
    },
}

/// Notification messages
#[derive(Debug, Clone)]
pub enum NotificationMessage {
    /// Show info notification
    Info(String),
    /// Show success notification
    Success(String),
    /// Show warning notification
    Warning(String),
    /// Show error notification
    Error(String),
    /// Dismiss notification
    Dismiss(usize),
}

/// Search Space Explorer messages
#[derive(Debug, Clone)]
pub enum SearchSpaceMessage {
    /// Navigate to a position
    NavigateTo(Vec<usize>),
    /// Step forward in current direction
    StepForward,
    /// Step backward (undo last step)
    StepBackward,
    /// Change navigation mode
    SetNavigationMode(crate::panels::NavigationMode),
    /// Change view mode
    SetViewMode(crate::panels::ViewMode),
    /// Set resonance threshold filter
    SetThreshold(f64),
    /// Toggle auto-explore
    ToggleAutoExplore,
    /// Clear exploration history
    ClearHistory,
    /// Jump to high-resonance region
    JumpToHighResonance,
    /// Export current path
    ExportPath,
}

/// Network Explorer messages
#[derive(Debug, Clone)]
pub enum NetworkExplorerMessage {
    /// Load a network by ID
    LoadNetwork(String),
    /// Select a node for inspection
    SelectNode(Option<String>),
    /// Change layout mode
    SetLayoutMode(crate::panels::LayoutMode),
    /// Toggle node type filter
    ToggleNodeTypeFilter(String),
    /// Toggle edge visibility
    ToggleEdges,
    /// Set edge weight threshold
    SetEdgeThreshold(f64),
    /// Find path between nodes
    FindPath {
        source: String,
        target: String,
    },
    /// Clear path finding
    ClearPaths,
    /// Detect communities
    DetectCommunities,
    /// Highlight critical nodes
    HighlightCriticalNodes,
    /// Export network data
    ExportNetwork,
}

/// Infogenetic Browser messages
#[derive(Debug, Clone)]
pub enum InfogeneticBrowserMessage {
    /// Update search query
    QueryChanged(String),
    /// Set query type
    SetQueryType(crate::panels::QueryType),
    /// Add filter criterion
    AddFilter(crate::panels::FilterCriterion),
    /// Remove filter
    RemoveFilter(usize),
    /// Clear all filters
    ClearFilters,
    /// Change sort field
    SetSortBy(crate::panels::SortField),
    /// Toggle sort direction
    ToggleSortDirection,
    /// Execute search
    Search,
    /// Navigate to page
    GoToPage(usize),
    /// Change items per page
    SetItemsPerPage(usize),
    /// Select entry for details
    SelectEntry(Option<usize>),
    /// Export results
    ExportResults,
}

/// Anomaly Investigation messages
#[derive(Debug, Clone)]
pub enum AnomalyInvestigationMessage {
    /// Select anomaly for investigation
    SelectAnomaly(Option<usize>),
    /// Update investigation notes
    NotesChanged(String),
    /// Add type filter
    AddTypeFilter(crate::panels::AnomalyType),
    /// Remove type filter
    RemoveTypeFilter(crate::panels::AnomalyType),
    /// Add severity filter
    AddSeverityFilter(crate::panels::AnomalySeverity),
    /// Remove severity filter
    RemoveSeverityFilter(crate::panels::AnomalySeverity),
    /// Clear all filters
    ClearFilters,
    /// Find related entities
    FindRelatedEntities,
    /// Load timeline
    LoadTimeline,
    /// Find similar cases
    FindSimilarCases,
    /// Mark anomaly as investigated
    MarkInvestigated,
    /// Export investigation report
    ExportReport,
}

/// Forensic Workflows messages
#[derive(Debug, Clone)]
pub enum ForensicWorkflowsMessage {
    /// Start a workflow
    StartWorkflow(String),
    /// Cancel active workflow
    CancelWorkflow,
    /// Complete current step
    CompleteStep,
    /// Go to previous step
    PreviousStep,
    /// Update step data
    UpdateStepData {
        key: String,
        value: String,
    },
    /// Load workflow history
    LoadHistory,
    /// View workflow execution
    ViewExecution(String),
    /// Export workflow results
    ExportResults,
}
