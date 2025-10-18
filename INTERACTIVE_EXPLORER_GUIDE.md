# PHOSPHOROS Interactive Explorer Guide

## Overview

This guide explains how to use the new interactive exploration features in the PHOSPHOROS Dashboard. These features transform the system from a passive monitoring tool into an active forensic investigation platform.

## 🗺️ Search Space Explorer

### Purpose
Manually navigate and explore the cryptographic seed/address space using various navigation strategies and visualization modes.

### Key Features

#### Navigation Modes

1. **Manual Mode** (Default)
   - Point-and-click exploration
   - Select specific nodes to explore
   - Full user control over navigation path

2. **Sequential Mode**
   - Systematically step through the search space
   - Follows a deterministic path
   - Useful for comprehensive coverage

3. **Random Walk Mode**
   - Stochastic exploration
   - Discovers unexpected patterns
   - Good for finding outliers

4. **Directed Search Mode**
   - Follows resonance gradients
   - Automatically navigates toward high-resonance regions
   - Efficient for finding optimal solutions

#### View Modes

1. **Tree View**
   - Hierarchical representation
   - Shows parent-child relationships
   - Best for understanding structure

2. **Graph View**
   - Network visualization
   - Shows connections and relationships
   - Useful for seeing global patterns

3. **List View**
   - Simple linear display
   - Easy to scan and filter
   - Good for detailed inspection

4. **5D Projection View**
   - High-dimensional visualization
   - Shows spectral properties
   - Advanced analysis capabilities

### Usage Workflow

1. **Start Exploration**
   - Select a navigation mode
   - Choose preferred view mode
   - Begin from root position

2. **Navigate the Space**
   - Use "Forward" to advance
   - Use "Back" to retrace steps
   - Use "Jump to High Resonance" for quick navigation

3. **Monitor Progress**
   - Check exploration history (last 10 positions)
   - View visible nodes with resonance scores
   - Track distance from current position

4. **Export Results**
   - Save exploration path
   - Document discovered high-resonance regions
   - Share findings with team

### Best Practices

- Start with **Manual** mode to familiarize yourself with the space
- Use **Directed Search** when you have a specific goal
- Switch to **Random Walk** to discover unexpected patterns
- Keep an eye on resonance scores to identify interesting regions
- Regularly check history to avoid revisiting same areas

## 🕸️ Network Topology Explorer

### Purpose
Visualize and analyze blockchain network structures, identify communities, find critical nodes, and explore relationships.

### Key Features

#### Layout Modes

1. **Force-Directed** (Default)
   - Physics-based layout
   - Naturally clusters related nodes
   - Interactive and dynamic

2. **Hierarchical**
   - Tree-like structure
   - Shows clear hierarchies
   - Good for understanding layers

3. **Circular**
   - Nodes arranged in a ring
   - Equal spacing
   - Good for symmetric structures

4. **Geographic**
   - Uses real-world coordinates (if available)
   - Shows geographic distribution
   - Useful for location-based analysis

#### Analysis Tools

1. **Community Detection**
   - Identifies clusters of related nodes
   - Shows internal density
   - Calculates average resonance per community

2. **Critical Node Identification**
   - Computes degree centrality
   - Calculates betweenness centrality
   - Highlights bridge nodes and hubs

3. **Path Finding**
   - Find shortest path between two nodes
   - Calculate path resonance
   - Identify bottlenecks

### Usage Workflow

1. **Load Network**
   - Select network ID or cluster
   - Wait for visualization to render

2. **Explore Structure**
   - Switch layout modes to find best view
   - Click nodes to inspect details
   - Toggle edge display as needed

3. **Perform Analysis**
   - Click "Detect Communities" to find clusters
   - Click "Highlight Critical Nodes" to find key players
   - Use path finding to explore connections

4. **Export Findings**
   - Save network visualization
   - Export community data
   - Document critical nodes

### Best Practices

- Start with **Force-Directed** layout for initial exploration
- Use **Community Detection** before analyzing individual nodes
- Pay attention to nodes with high betweenness centrality (bridges)
- Filter node types to reduce visual clutter
- Adjust edge threshold to focus on strong connections

## 🧬 Infogenetic Database Browser

### Purpose
Query and explore the spectral database using advanced filtering, sorting, and search capabilities.

### Key Features

#### Query Types

1. **Address Search**
   - Direct address lookup
   - Supports partial matches
   - Fast and precise

2. **Spectral Signature Range**
   - Search by (ψ, ρ, ω) values
   - Find similar signatures
   - Range queries supported

3. **Cluster Membership**
   - Find all members of a cluster
   - Cross-cluster analysis
   - Relationship exploration

4. **Full-Text Search**
   - Search across all fields
   - Flexible and comprehensive
   - Good for exploratory queries

5. **Advanced Query**
   - Custom query builder
   - Multiple criteria
   - Boolean operators

#### Filtering & Sorting

- **Multi-Criteria Filters**: Combine multiple conditions
- **Sort Options**: By timestamp, resonance, address, or cluster size
- **Pagination**: Navigate large result sets efficiently
- **Detailed View**: Inspect individual entries in depth

### Usage Workflow

1. **Select Query Type**
   - Choose appropriate query type for your goal
   - Start with Address or Full-Text for exploratory searches

2. **Enter Query**
   - Type search terms
   - Add filters as needed
   - Set sort order

3. **Execute Search**
   - Click "Search" button
   - Wait for results to load
   - Review result count

4. **Explore Results**
   - Scroll through paginated results
   - Click entries for detailed view
   - Note spectral signatures

5. **Export Data**
   - Save search results
   - Export for further analysis
   - Share with team

### Best Practices

- Use **Spectral Signature** queries to find similar addresses
- Apply filters to narrow down large result sets
- Sort by resonance to find most interesting entries
- Use pagination efficiently (set appropriate items per page)
- Export results for offline analysis

## 🚨 Anomaly Investigation

### Purpose
Deep forensic analysis of detected blockchain anomalies with timeline analysis and related entity discovery.

### Key Features

#### Anomaly Types

1. **Sybil Attacks**
   - Multiple coordinated fake identities
   - Temporal correlation patterns
   - Network structure analysis

2. **Money Laundering**
   - Suspicious fund flow patterns
   - Mixer/tumbler usage
   - Destination clustering

3. **Volume Anomalies**
   - Unusual transaction volumes
   - Spike detection
   - Statistical outliers

4. **Temporal Anomalies**
   - Timing pattern irregularities
   - Periodic behavior
   - Synchronization analysis

5. **Structural Anomalies**
   - Network topology irregularities
   - Clustering patterns
   - Connection analysis

#### Investigation Tools

1. **Related Entity Discovery**
   - Find connected addresses
   - Map relationship network
   - Identify co-conspirators

2. **Timeline Analysis**
   - Visualize anomaly evolution
   - Track severity changes
   - Identify trigger events

3. **Similar Case Matching**
   - Find historical precedents
   - Pattern matching
   - Learn from past cases

### Usage Workflow

1. **Select Anomaly**
   - Browse detected anomalies
   - Sort by severity or type
   - Click to investigate

2. **Review Details**
   - Read anomaly description
   - Check affected entities
   - Note severity level

3. **Deep Dive Investigation**
   - Click "Find Related Entities"
   - Click "Load Timeline" to see evolution
   - Click "Find Similar Cases" for context

4. **Document Findings**
   - Add investigation notes
   - Mark as investigated when complete
   - Export investigation report

### Best Practices

- **Prioritize by severity**: Start with Critical and High severity anomalies
- **Check timeline first**: Understand how anomaly developed
- **Map all related entities**: Don't miss connections
- **Compare with similar cases**: Learn from patterns
- **Document thoroughly**: Maintain investigation notes

## 🔬 Forensic Workflows

### Purpose
Step-by-step guided investigation processes for common forensic analysis tasks.

### Available Workflows

#### 1. Sybil Attack Investigation (30-45 min)

**Purpose**: Systematic investigation of potential Sybil attacks across wallet clusters.

**Steps**:
1. Identify Seed Cluster
   - Locate initial cluster of suspicious addresses
   - Collect cluster member list

2. Analyze Temporal Patterns
   - Examine transaction timing
   - Calculate frequency patterns
   - Compute temporal correlation scores

3. Map Network Topology
   - Visualize connections between members
   - Generate network graph
   - Identify central nodes

4. Calculate Resonance Signatures
   - Compute spectral signatures for each member
   - Find pattern similarities
   - Document spectral properties

5. Generate Report
   - Compile all findings
   - Create investigation summary
   - Export comprehensive report

#### 2. Money Laundering Trace (45-60 min)

**Purpose**: Track suspicious fund flows and identify laundering patterns.

**Steps**:
1. Identify Origin Transaction
   - Locate starting point of funds
   - Document transaction details

2. Trace Fund Flow
   - Follow transaction chain
   - Map all intermediate addresses

3. Identify Mixing Services
   - Detect tumblers/mixers
   - Note privacy service usage

4. Map Destination Clusters
   - Identify final destinations
   - Analyze destination patterns

#### 3. Comprehensive Address Profiling (20-30 min)

**Purpose**: Build detailed behavioral profile of a specific address or wallet.

**Steps**:
1. Gather Address Data
   - Collect transaction history
   - Gather all metadata

2. Analyze Transaction Patterns
   - Examine timing, amounts, frequency
   - Identify behavioral patterns

3. Compute Spectral Signature
   - Generate 5D spectral fingerprint
   - Calculate (ψ, ρ, ω) values

4. Find Similar Addresses
   - Search infogenetic database
   - Identify similar profiles

#### 4. Cluster Forensics Analysis (30-40 min)

**Purpose**: Deep forensic analysis of detected address clusters.

**Steps**:
1. Load Cluster Data
   - Import cluster from detection system
   - Verify cluster membership

2. Analyze Internal Structure
   - Examine intra-cluster connections
   - Calculate internal density

3. Identify External Connections
   - Map connections outside cluster
   - Analyze bridge addresses

4. Compute Cluster Resonance
   - Calculate collective spectral properties
   - Generate cluster metrics

### Usage Workflow

1. **Select Workflow**
   - Browse available workflows
   - Choose appropriate workflow for your investigation
   - Review estimated duration

2. **Start Workflow**
   - Click workflow to begin
   - Read step description carefully

3. **Execute Steps**
   - Complete each step in order
   - Collect required inputs
   - Document outputs

4. **Navigate Workflow**
   - Use "Complete Step" to advance
   - Use "Previous Step" if needed
   - Use "Cancel" to abort workflow

5. **Export Results**
   - Generate final report
   - Save workflow execution history
   - Share findings with team

### Best Practices

- **Read all steps before starting**: Understand the full process
- **Gather inputs beforehand**: Have necessary data ready
- **Document at each step**: Don't wait until the end
- **Don't skip steps**: Follow the process completely
- **Review similar cases**: Learn from workflow history

## Integration Between Explorers

The explorers are designed to work together:

1. **Search Space → Infogenetic Browser**
   - Find high-resonance seeds
   - Look up in database
   - Analyze spectral properties

2. **Infogenetic Browser → Network Explorer**
   - Select interesting addresses
   - Load into network visualization
   - Analyze relationships

3. **Network Explorer → Anomaly Investigation**
   - Identify suspicious patterns
   - Switch to anomaly panel
   - Deep dive into specific issues

4. **Anomaly Investigation → Forensic Workflows**
   - Select appropriate workflow
   - Follow guided investigation
   - Complete comprehensive analysis

## Tips for Maximum Effectiveness

1. **Start with Home Panel**: Check system status and activity
2. **Use Search Space Explorer**: Find interesting regions
3. **Switch to Infogenetic Browser**: Query discovered data
4. **Load Network Explorer**: Visualize relationships
5. **Check Anomaly Panel**: Investigate suspicious patterns
6. **Start Workflow**: Complete formal investigation
7. **Export Everything**: Document your findings

## Keyboard Shortcuts (Future Enhancement)

Currently, all operations are mouse-driven. Future versions will include:
- Arrow keys for navigation
- Spacebar for quick actions
- Ctrl+E for export
- Ctrl+F for search
- Esc to cancel operations

## Performance Considerations

- **Large Networks**: May be slow to render; use filters
- **Database Queries**: Set appropriate pagination limits
- **History**: Automatically limited to prevent memory issues
- **Export Operations**: May take time for large datasets

## Troubleshooting

**Problem**: No nodes visible in Search Space Explorer
**Solution**: Click "Forward" to generate initial nodes

**Problem**: Network won't load
**Solution**: Ensure services are running (check Home panel)

**Problem**: Search returns no results
**Solution**: Check query syntax and filters, try broader search

**Problem**: Workflow won't advance
**Solution**: Ensure all required inputs are provided

## Future Enhancements

Planned improvements include:
- Real-time 5D field visualization
- Drag-and-drop network editing
- Custom workflow creation
- Collaborative investigation features
- Advanced analytics integration
- Machine learning suggestions
- Predictive anomaly detection

## Support

For issues, questions, or feature requests:
- Check system logs (System Log panel)
- Review documentation in README files
- Submit issues on GitHub
- Consult DELTA_BLUEPRINT.md for architecture details

---

**Version**: 1.0.0  
**Last Updated**: 2025-10-18  
**Author**: PHOSPHOROS Project Team
