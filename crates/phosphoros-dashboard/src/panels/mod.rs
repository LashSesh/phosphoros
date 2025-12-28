//! Panel definitions and state management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Panel identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelId {
    /// Live Overview (Home)
    Home,
    /// Seed & Wallet Management
    SeedManagement,
    /// Resonance & Spectrography
    Resonance,
    /// Cluster Explorer & Forensik
    ClusterExplorer,
    /// Search Space Explorer
    SearchSpaceExplorer,
    /// Network Topology Explorer
    NetworkExplorer,
    /// Infogenetic Database Browser
    InfogeneticBrowser,
    /// Anomaly Investigation
    AnomalyInvestigation,
    /// Forensic Workflows
    ForensicWorkflows,
    /// Stealth/Privacy Controls
    Stealth,
    /// System Log / Notifications
    SystemLog,
    /// Settings & Tasks
    Settings,
}

impl PanelId {
    /// Get panel display name
    pub fn name(&self) -> &'static str {
        match self {
            PanelId::Home => "Live Overview",
            PanelId::SeedManagement => "Seed & Wallet",
            PanelId::Resonance => "Resonance & Spectro",
            PanelId::ClusterExplorer => "Cluster Explorer",
            PanelId::SearchSpaceExplorer => "Search Space Explorer",
            PanelId::NetworkExplorer => "Network Topology",
            PanelId::InfogeneticBrowser => "Infogenetic Database",
            PanelId::AnomalyInvestigation => "Anomaly Investigation",
            PanelId::ForensicWorkflows => "Forensic Workflows",
            PanelId::Stealth => "Stealth/Privacy",
            PanelId::SystemLog => "System Log",
            PanelId::Settings => "Settings & Tasks",
        }
    }

    /// Get panel icon (using simple unicode symbols)
    pub fn icon(&self) -> &'static str {
        match self {
            PanelId::Home => "🏠",
            PanelId::SeedManagement => "🔑",
            PanelId::Resonance => "📊",
            PanelId::ClusterExplorer => "🔍",
            PanelId::SearchSpaceExplorer => "🗺️",
            PanelId::NetworkExplorer => "🕸️",
            PanelId::InfogeneticBrowser => "🧬",
            PanelId::AnomalyInvestigation => "🚨",
            PanelId::ForensicWorkflows => "🔬",
            PanelId::Stealth => "🔒",
            PanelId::SystemLog => "📝",
            PanelId::Settings => "⚙️",
        }
    }

    /// Get all panel IDs
    pub fn all() -> Vec<PanelId> {
        vec![
            PanelId::Home,
            PanelId::SeedManagement,
            PanelId::Resonance,
            PanelId::ClusterExplorer,
            PanelId::SearchSpaceExplorer,
            PanelId::NetworkExplorer,
            PanelId::InfogeneticBrowser,
            PanelId::AnomalyInvestigation,
            PanelId::ForensicWorkflows,
            PanelId::Stealth,
            PanelId::SystemLog,
            PanelId::Settings,
        ]
    }
}

/// Panel state container
#[derive(Debug, Default)]
pub struct PanelState {
    /// Home panel state
    pub home: HomeState,
    /// Seed management state
    pub seed_management: SeedManagementState,
    /// Resonance panel state
    pub resonance: ResonanceState,
    /// Cluster explorer state
    pub cluster: ClusterState,
    /// Search space explorer state
    pub search_space: SearchSpaceState,
    /// Network explorer state
    pub network: NetworkExplorerState,
    /// Infogenetic browser state
    pub infogenetic: InfogeneticBrowserState,
    /// Anomaly investigation state
    pub anomaly: AnomalyInvestigationState,
    /// Forensic workflows state
    pub forensic: ForensicWorkflowsState,
    /// Stealth/Privacy state
    pub stealth: StealthState,
    /// System log state
    pub log: LogState,
    /// Settings state
    pub settings: SettingsState,
}

/// Home panel state
#[derive(Debug, Default)]
pub struct HomeState {
    /// Active tasks count
    pub active_tasks: usize,
    /// Seeds collected
    pub seeds_count: usize,
    /// Clusters discovered
    pub clusters_count: usize,
    /// Coverage percentage
    pub coverage: f64,
    /// Activity history (last 100 points)
    pub activity_history: Vec<f64>,
}

/// Seed management state
#[derive(Debug, Default)]
pub struct SeedManagementState {
    /// Current input
    pub input: String,
    /// Imported seeds
    pub seeds: Vec<SeedInfo>,
    /// Currently selected seed index
    pub selected: Option<usize>,
}

/// Seed information
#[derive(Debug, Clone)]
pub struct SeedInfo {
    /// Seed ID
    pub id: String,
    /// Mnemonic phrase (masked)
    pub mnemonic_masked: String,
    /// Generated addresses
    pub addresses: Vec<AddressInfo>,
    /// Import timestamp
    pub imported_at: chrono::DateTime<chrono::Utc>,
}

/// Address information
#[derive(Debug, Clone)]
pub struct AddressInfo {
    /// Address string
    pub address: String,
    /// Chain type
    pub chain: String,
    /// Derivation path
    pub path: String,
}

/// Resonance panel state
#[derive(Debug, Default)]
pub struct ResonanceState {
    /// Analysis running
    pub running: bool,
    /// Current progress (0.0 to 1.0)
    pub progress: f64,
    /// Best resonance found
    pub best_resonance: f64,
    /// Operator configurations
    pub operators: HashMap<String, f64>,
    /// Resonance history for visualization
    pub resonance_history: Vec<(f64, f64, f64)>, // (psi, rho, omega)
}

/// Cluster explorer state
#[derive(Debug, Default)]
pub struct ClusterState {
    /// Search query
    pub search_query: String,
    /// Found clusters
    pub clusters: Vec<ClusterInfo>,
    /// Selected cluster
    pub selected: Option<usize>,
}

/// Cluster information
#[derive(Debug, Clone)]
pub struct ClusterInfo {
    /// Cluster ID
    pub id: String,
    /// Members count
    pub members: usize,
    /// Resonance magnitude
    pub resonance: f64,
    /// Discovery timestamp
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

/// System log state
#[derive(Debug, Default)]
pub struct LogState {
    /// Filter string
    pub filter: String,
    /// Auto-scroll enabled
    pub auto_scroll: bool,
}

/// Stealth/Privacy panel state
#[derive(Debug)]
pub struct StealthState {
    /// Whether stealth mode is enabled
    pub enabled: bool,
    /// Current stealth mode
    pub mode: StealthMode,
    /// Preferred API type for mimicry
    pub preferred_api: Option<ApiType>,
    /// Number of active stealth tasks
    pub active_tasks: usize,
    /// Number of proxies configured
    pub proxy_count: usize,
    /// Proxy rotation enabled
    pub proxy_enabled: bool,
    /// Enable request logging
    pub logging_enabled: bool,
    /// Temporal jitter enabled
    pub jitter_enabled: bool,
    /// Minimum jitter (ms)
    pub min_jitter_ms: u64,
    /// Maximum jitter (ms)
    pub max_jitter_ms: u64,
    /// Stealth task history (last 100 entries)
    pub task_history: Vec<StealthTaskInfo>,
}

/// Stealth mode enumeration for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StealthMode {
    /// No stealth
    Open,
    /// Traffic mimicry
    Mimicry,
    /// Steganography
    Steganography,
    /// Adaptive mode
    Adaptive,
}

impl Default for StealthMode {
    fn default() -> Self {
        Self::Open
    }
}

/// API type for mimicry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiType {
    /// OpenAI
    OpenAI,
    /// Slack
    Slack,
    /// Telegram
    Telegram,
    /// Discord
    Discord,
    /// Generic
    Generic,
}

/// Information about a stealth task
#[derive(Debug, Clone)]
pub struct StealthTaskInfo {
    /// Task ID
    pub id: String,
    /// Task type
    pub task_type: String,
    /// Stealth mode used
    pub mode: StealthMode,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Success status
    pub success: bool,
}

impl Default for StealthState {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: StealthMode::default(),
            preferred_api: None,
            active_tasks: 0,
            proxy_count: 0,
            proxy_enabled: false,
            logging_enabled: true,
            jitter_enabled: true,
            min_jitter_ms: 100,
            max_jitter_ms: 1000,
            task_history: Vec::new(),
        }
    }
}

/// Settings state
#[derive(Debug)]
pub struct SettingsState {
    /// Theme (dark/light)
    pub dark_mode: bool,
    /// Language
    pub language: String,
    /// Task priorities
    pub task_priorities: HashMap<String, usize>,
    /// Auto-start services
    pub auto_start: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            dark_mode: true,
            language: "English".to_string(),
            task_priorities: HashMap::new(),
            auto_start: true,
        }
    }
}

/// Search Space Explorer state - allows manual navigation through seed/address space
#[derive(Debug, Default)]
pub struct SearchSpaceState {
    /// Current position in search space (BIP39 word indices)
    pub current_position: Vec<usize>,
    /// Search depth (how many words to explore)
    pub depth: usize,
    /// Navigation mode (sequential, random walk, directed search)
    pub mode: NavigationMode,
    /// Exploration history (breadcrumb trail)
    pub history: Vec<SearchPosition>,
    /// Current view mode (tree, graph, list)
    pub view_mode: ViewMode,
    /// Resonance scores for visible nodes
    pub visible_nodes: Vec<SearchNode>,
    /// Filter threshold for resonance
    pub resonance_threshold: f64,
    /// Auto-explore enabled
    pub auto_explore: bool,
}

/// Navigation mode for search space explorer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMode {
    /// Step through sequentially
    Sequential,
    /// Random walk through space
    RandomWalk,
    /// Directed search based on resonance gradient
    DirectedSearch,
    /// Manual selection
    Manual,
}

impl Default for NavigationMode {
    fn default() -> Self {
        Self::Manual
    }
}

/// View mode for visualizing search space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Tree hierarchy view
    Tree,
    /// Graph/network view
    Graph,
    /// Simple list view
    List,
    /// 5D projection view
    Projection5D,
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Tree
    }
}

/// A position in the search space
#[derive(Debug, Clone)]
pub struct SearchPosition {
    /// Word indices
    pub indices: Vec<usize>,
    /// Corresponding words
    pub words: Vec<String>,
    /// Resonance score at this position
    pub resonance: f64,
    /// Timestamp when visited
    pub visited_at: chrono::DateTime<chrono::Utc>,
}

/// A node in the search space visualization
#[derive(Debug, Clone)]
pub struct SearchNode {
    /// Node ID
    pub id: String,
    /// Word at this node
    pub word: String,
    /// Word index
    pub index: usize,
    /// Resonance score
    pub resonance: f64,
    /// Distance from current position
    pub distance: f64,
    /// Child count
    pub children_count: usize,
}

/// Network Topology Explorer state - visualize and navigate blockchain networks
#[derive(Debug, Default)]
pub struct NetworkExplorerState {
    /// Currently loaded network
    pub network_id: Option<String>,
    /// Selected node for inspection
    pub selected_node: Option<String>,
    /// Visualization layout mode
    pub layout_mode: LayoutMode,
    /// Filter for node types
    pub node_type_filter: Vec<String>,
    /// Show edges/connections
    pub show_edges: bool,
    /// Edge weight threshold
    pub edge_threshold: f64,
    /// Detected communities/clusters
    pub communities: Vec<CommunityInfo>,
    /// Critical nodes (high centrality)
    pub critical_nodes: Vec<NodeInfo>,
    /// Path finding from/to
    pub path_source: Option<String>,
    pub path_target: Option<String>,
    /// Found paths
    pub paths: Vec<NetworkPath>,
}

/// Network layout mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// Force-directed layout
    ForceDirected,
    /// Hierarchical layout
    Hierarchical,
    /// Circular layout
    Circular,
    /// Geographic (if coordinates available)
    Geographic,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self::ForceDirected
    }
}

/// Community/cluster information
#[derive(Debug, Clone)]
pub struct CommunityInfo {
    /// Community ID
    pub id: String,
    /// Member count
    pub size: usize,
    /// Internal edge density
    pub density: f64,
    /// Average resonance
    pub avg_resonance: f64,
}

/// Node information
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Node ID
    pub id: String,
    /// Node label/address
    pub label: String,
    /// Node type
    pub node_type: String,
    /// Centrality measures
    pub degree_centrality: f64,
    pub betweenness_centrality: f64,
    /// Resonance score
    pub resonance: f64,
}

/// A path through the network
#[derive(Debug, Clone)]
pub struct NetworkPath {
    /// Path nodes
    pub nodes: Vec<String>,
    /// Total path length/cost
    pub length: f64,
    /// Path resonance (product of node resonances)
    pub resonance: f64,
}

/// Infogenetic Database Browser state - explore and query the spectral database
#[derive(Debug, Default)]
pub struct InfogeneticBrowserState {
    /// Search query
    pub query: String,
    /// Query type
    pub query_type: QueryType,
    /// Filter criteria
    pub filters: Vec<FilterCriterion>,
    /// Sort field and order
    pub sort_by: SortField,
    pub sort_ascending: bool,
    /// Current page for pagination
    pub page: usize,
    /// Items per page
    pub items_per_page: usize,
    /// Search results
    pub results: Vec<InfogeneticEntry>,
    /// Total results count
    pub total_results: usize,
    /// Selected entry for detailed view
    pub selected_entry: Option<usize>,
}

/// Query type for infogenetic database
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// Search by address
    Address,
    /// Search by spectral signature range
    SpectralSignature,
    /// Search by cluster membership
    Cluster,
    /// Full text search
    FullText,
    /// Advanced query (custom)
    Advanced,
}

impl Default for QueryType {
    fn default() -> Self {
        Self::Address
    }
}

/// Filter criterion for database queries
#[derive(Debug, Clone)]
pub struct FilterCriterion {
    /// Field name
    pub field: String,
    /// Operator (equals, greater than, less than, contains, etc.)
    pub operator: FilterOperator,
    /// Value
    pub value: String,
}

/// Filter operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
}

/// Sort field for database results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    /// Sort by timestamp
    Timestamp,
    /// Sort by resonance
    Resonance,
    /// Sort by address
    Address,
    /// Sort by cluster size
    ClusterSize,
}

impl Default for SortField {
    fn default() -> Self {
        Self::Timestamp
    }
}

/// An entry in the infogenetic database
#[derive(Debug, Clone)]
pub struct InfogeneticEntry {
    /// Entry ID
    pub id: String,
    /// Address or seed hash
    pub address: String,
    /// Spectral signature (ψ, ρ, ω)
    pub signature: (f64, f64, f64),
    /// Resonance score
    pub resonance: f64,
    /// Cluster membership
    pub cluster_id: Option<String>,
    /// Chain type
    pub chain: String,
    /// Discovery timestamp
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Anomaly Investigation state - deep dive into detected anomalies
#[derive(Debug, Default)]
pub struct AnomalyInvestigationState {
    /// List of detected anomalies
    pub anomalies: Vec<AnomalyInfo>,
    /// Selected anomaly for investigation
    pub selected: Option<usize>,
    /// Investigation notes
    pub notes: String,
    /// Anomaly type filter
    pub type_filter: Vec<AnomalyType>,
    /// Severity filter
    pub severity_filter: Vec<AnomalySeverity>,
    /// Related entities found
    pub related_entities: Vec<String>,
    /// Timeline of anomaly evolution
    pub timeline: Vec<AnomalyTimelineEntry>,
    /// Comparison with similar cases
    pub similar_cases: Vec<String>,
}

/// Anomaly type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyType {
    /// Sybil attack pattern
    SybilAttack,
    /// Money laundering indicator
    MoneyLaundering,
    /// Unusual transaction volume
    VolumeAnomaly,
    /// Temporal pattern anomaly
    TemporalAnomaly,
    /// Network structure anomaly
    StructuralAnomaly,
    /// Other/unknown
    Other,
}

/// Anomaly severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Anomaly information
#[derive(Debug, Clone)]
pub struct AnomalyInfo {
    /// Anomaly ID
    pub id: String,
    /// Anomaly type
    pub anomaly_type: AnomalyType,
    /// Severity level
    pub severity: AnomalySeverity,
    /// Affected entities
    pub affected_entities: Vec<String>,
    /// Detection timestamp
    pub detected_at: chrono::DateTime<chrono::Utc>,
    /// Anomaly score (higher = more anomalous)
    pub score: f64,
    /// Description
    pub description: String,
}

/// Timeline entry for anomaly evolution
#[derive(Debug, Clone)]
pub struct AnomalyTimelineEntry {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event description
    pub event: String,
    /// Severity at this point
    pub severity: AnomalySeverity,
}

/// Forensic Workflows state - guided investigation workflows
#[derive(Debug)]
pub struct ForensicWorkflowsState {
    /// Available workflows
    pub workflows: Vec<WorkflowTemplate>,
    /// Currently active workflow
    pub active_workflow: Option<ActiveWorkflow>,
    /// Workflow execution history
    pub history: Vec<WorkflowExecution>,
}

impl Default for ForensicWorkflowsState {
    fn default() -> Self {
        Self {
            workflows: Self::default_workflows(),
            active_workflow: None,
            history: Vec::new(),
        }
    }
}

impl ForensicWorkflowsState {
    /// Get default forensic workflows
    fn default_workflows() -> Vec<WorkflowTemplate> {
        vec![
            WorkflowTemplate {
                id: "sybil-investigation".to_string(),
                name: "Sybil Attack Investigation".to_string(),
                description: "Systematic investigation of potential Sybil attacks across wallet clusters".to_string(),
                steps: vec![
                    WorkflowStep {
                        number: 1,
                        title: "Identify Seed Cluster".to_string(),
                        description: "Locate the initial cluster of suspicious addresses".to_string(),
                        inputs: vec!["Starting address or cluster ID".to_string()],
                        outputs: vec!["Cluster members list".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 2,
                        title: "Analyze Temporal Patterns".to_string(),
                        description: "Examine transaction timing and frequency patterns".to_string(),
                        inputs: vec!["Cluster members".to_string()],
                        outputs: vec!["Temporal correlation scores".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 3,
                        title: "Map Network Topology".to_string(),
                        description: "Visualize connections between cluster members".to_string(),
                        inputs: vec!["Cluster members".to_string()],
                        outputs: vec!["Network graph".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 4,
                        title: "Calculate Resonance Signatures".to_string(),
                        description: "Compute spectral signatures for pattern matching".to_string(),
                        inputs: vec!["Cluster data".to_string()],
                        outputs: vec!["Spectral signatures".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 5,
                        title: "Generate Report".to_string(),
                        description: "Compile findings into investigation report".to_string(),
                        inputs: vec!["All analysis results".to_string()],
                        outputs: vec!["Investigation report".to_string()],
                        completed: false,
                    },
                ],
                estimated_duration: "30-45 minutes".to_string(),
            },
            WorkflowTemplate {
                id: "money-laundering-trace".to_string(),
                name: "Money Laundering Trace".to_string(),
                description: "Track suspicious fund flows and identify laundering patterns".to_string(),
                steps: vec![
                    WorkflowStep {
                        number: 1,
                        title: "Identify Origin Transaction".to_string(),
                        description: "Locate the starting point of suspicious funds".to_string(),
                        inputs: vec!["Transaction hash or address".to_string()],
                        outputs: vec!["Origin transaction details".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 2,
                        title: "Trace Fund Flow".to_string(),
                        description: "Follow the trail of transactions".to_string(),
                        inputs: vec!["Origin transaction".to_string()],
                        outputs: vec!["Transaction chain".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 3,
                        title: "Identify Mixing Services".to_string(),
                        description: "Detect use of tumblers, mixers, or privacy services".to_string(),
                        inputs: vec!["Transaction chain".to_string()],
                        outputs: vec!["Mixing service usage".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 4,
                        title: "Map Destination Clusters".to_string(),
                        description: "Identify final destination addresses and clusters".to_string(),
                        inputs: vec!["Transaction chain".to_string()],
                        outputs: vec!["Destination clusters".to_string()],
                        completed: false,
                    },
                ],
                estimated_duration: "45-60 minutes".to_string(),
            },
            WorkflowTemplate {
                id: "address-profiling".to_string(),
                name: "Comprehensive Address Profiling".to_string(),
                description: "Build detailed behavioral profile of a specific address or wallet".to_string(),
                steps: vec![
                    WorkflowStep {
                        number: 1,
                        title: "Gather Address Data".to_string(),
                        description: "Collect all available transaction and metadata".to_string(),
                        inputs: vec!["Target address".to_string()],
                        outputs: vec!["Complete address history".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 2,
                        title: "Analyze Transaction Patterns".to_string(),
                        description: "Examine timing, amounts, and frequency patterns".to_string(),
                        inputs: vec!["Address history".to_string()],
                        outputs: vec!["Behavioral patterns".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 3,
                        title: "Compute Spectral Signature".to_string(),
                        description: "Generate unique 5D spectral fingerprint".to_string(),
                        inputs: vec!["Behavioral patterns".to_string()],
                        outputs: vec!["Spectral signature (ψ, ρ, ω)".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 4,
                        title: "Find Similar Addresses".to_string(),
                        description: "Search infogenetic database for similar profiles".to_string(),
                        inputs: vec!["Spectral signature".to_string()],
                        outputs: vec!["Similar address list".to_string()],
                        completed: false,
                    },
                ],
                estimated_duration: "20-30 minutes".to_string(),
            },
            WorkflowTemplate {
                id: "cluster-forensics".to_string(),
                name: "Cluster Forensics Analysis".to_string(),
                description: "Deep forensic analysis of detected address clusters".to_string(),
                steps: vec![
                    WorkflowStep {
                        number: 1,
                        title: "Load Cluster Data".to_string(),
                        description: "Import cluster from detection system".to_string(),
                        inputs: vec!["Cluster ID".to_string()],
                        outputs: vec!["Cluster members and metadata".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 2,
                        title: "Analyze Internal Structure".to_string(),
                        description: "Examine connections within the cluster".to_string(),
                        inputs: vec!["Cluster members".to_string()],
                        outputs: vec!["Internal topology".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 3,
                        title: "Identify External Connections".to_string(),
                        description: "Map connections to addresses outside the cluster".to_string(),
                        inputs: vec!["Cluster members".to_string()],
                        outputs: vec!["External connection graph".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 4,
                        title: "Compute Cluster Resonance".to_string(),
                        description: "Calculate collective spectral properties".to_string(),
                        inputs: vec!["Cluster data".to_string()],
                        outputs: vec!["Cluster resonance metrics".to_string()],
                        completed: false,
                    },
                ],
                estimated_duration: "30-40 minutes".to_string(),
            },
            // Monero-specific forensic workflow for IRS compliance
            WorkflowTemplate {
                id: "monero-ring-analysis".to_string(),
                name: "Monero Ring Signature Analysis".to_string(),
                description: "Specialized analysis of Monero transactions using ring signature heuristics and temporal correlation. Designed for law enforcement and IRS cryptocurrency tracing requirements.".to_string(),
                steps: vec![
                    WorkflowStep {
                        number: 1,
                        title: "Import Monero Transaction Data".to_string(),
                        description: "Load transaction hashes, key images, and ring member data from blockchain or provided evidence".to_string(),
                        inputs: vec!["Transaction hash(es)".to_string(), "Monero address (optional)".to_string()],
                        outputs: vec!["Transaction metadata".to_string(), "Ring member candidates".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 2,
                        title: "Key Image Analysis".to_string(),
                        description: "Analyze key images to detect spent outputs and identify transaction linkages".to_string(),
                        inputs: vec!["Key images from Step 1".to_string()],
                        outputs: vec!["Spent output mapping".to_string(), "Potential links".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 3,
                        title: "Temporal Pattern Correlation".to_string(),
                        description: "Apply timing analysis to identify real inputs among ring decoys based on transaction timing patterns".to_string(),
                        inputs: vec!["Ring members".to_string(), "Transaction timestamps".to_string()],
                        outputs: vec!["Temporal correlation scores".to_string(), "Probability rankings".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 4,
                        title: "5D Spectral Signature Mapping".to_string(),
                        description: "Compute unique spectral signatures for address clusters using PHOSPHOROS resonance engine".to_string(),
                        inputs: vec!["Identified addresses".to_string(), "Transaction patterns".to_string()],
                        outputs: vec!["Spectral signatures (psi, rho, omega)".to_string(), "Cluster assignments".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 5,
                        title: "Cross-Chain Exchange Detection".to_string(),
                        description: "Identify potential exchange deposits and cross-chain transfers for fund tracing".to_string(),
                        inputs: vec!["Identified outputs".to_string(), "Known exchange patterns".to_string()],
                        outputs: vec!["Exchange candidates".to_string(), "Cross-chain links".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 6,
                        title: "Generate Evidence Report".to_string(),
                        description: "Compile comprehensive forensic report suitable for legal proceedings and regulatory submissions".to_string(),
                        inputs: vec!["All analysis results".to_string()],
                        outputs: vec!["PDF Evidence Report".to_string(), "JSON Data Export".to_string(), "Chain of Custody Log".to_string()],
                        completed: false,
                    },
                ],
                estimated_duration: "60-90 minutes".to_string(),
            },
            // Quick evidence export workflow
            WorkflowTemplate {
                id: "evidence-export".to_string(),
                name: "Evidence Package Export".to_string(),
                description: "Generate court-admissible evidence packages with proper chain of custody documentation".to_string(),
                steps: vec![
                    WorkflowStep {
                        number: 1,
                        title: "Select Analysis Results".to_string(),
                        description: "Choose completed analyses to include in evidence package".to_string(),
                        inputs: vec!["Analysis session IDs".to_string()],
                        outputs: vec!["Selected result set".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 2,
                        title: "Verify Data Integrity".to_string(),
                        description: "Compute cryptographic hashes of all evidence items".to_string(),
                        inputs: vec!["Selected results".to_string()],
                        outputs: vec!["SHA-256 checksums".to_string(), "Merkle root".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 3,
                        title: "Generate Documentation".to_string(),
                        description: "Create formatted reports with methodology descriptions".to_string(),
                        inputs: vec!["Verified data".to_string()],
                        outputs: vec!["Technical report (PDF)".to_string(), "Executive summary".to_string()],
                        completed: false,
                    },
                    WorkflowStep {
                        number: 4,
                        title: "Package and Sign".to_string(),
                        description: "Create timestamped, signed evidence package".to_string(),
                        inputs: vec!["All documentation".to_string()],
                        outputs: vec!["Evidence archive (.zip)".to_string(), "Digital signature".to_string()],
                        completed: false,
                    },
                ],
                estimated_duration: "15-20 minutes".to_string(),
            },
        ]
    }
}

/// A workflow template for forensic investigation
#[derive(Debug, Clone)]
pub struct WorkflowTemplate {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Description
    pub description: String,
    /// Steps in the workflow
    pub steps: Vec<WorkflowStep>,
    /// Estimated duration
    pub estimated_duration: String,
}

/// A step in a forensic workflow
#[derive(Debug, Clone)]
pub struct WorkflowStep {
    /// Step number
    pub number: usize,
    /// Step title
    pub title: String,
    /// Step description
    pub description: String,
    /// Required inputs
    pub inputs: Vec<String>,
    /// Expected outputs
    pub outputs: Vec<String>,
    /// Completion status
    pub completed: bool,
}

/// An active workflow being executed
#[derive(Debug, Clone)]
pub struct ActiveWorkflow {
    /// Template ID
    pub template_id: String,
    /// Current step
    pub current_step: usize,
    /// Step states
    pub steps: Vec<WorkflowStep>,
    /// Collected data
    pub data: HashMap<String, String>,
    /// Started at
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// A completed workflow execution
#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    /// Execution ID
    pub id: String,
    /// Template ID
    pub template_id: String,
    /// Started at
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Completed at
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Success status
    pub success: bool,
    /// Results summary
    pub summary: String,
}
