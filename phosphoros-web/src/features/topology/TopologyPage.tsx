import { useState, useMemo, useCallback } from 'react'
import { useNavigate } from 'react-router-dom'
import { ZoomIn, ZoomOut, Maximize2, Filter, Search, ExternalLink } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import { ForceGraph, GraphNode, GraphLink } from '@/components/graphs/ForceGraph'
import { formatAddress, formatResonance, getResonanceColor } from '@/lib/utils'
import { cn } from '@/lib/utils'

// Generate mock data for demo
function generateMockData(): { nodes: GraphNode[]; links: GraphLink[] } {
  const types: GraphNode['type'][] = ['wallet', 'contract', 'exchange', 'unknown']
  const nodes: GraphNode[] = []
  const links: GraphLink[] = []

  // Create nodes
  for (let i = 0; i < 30; i++) {
    nodes.push({
      id: `node-${i}`,
      label: `0x${Math.random().toString(16).slice(2, 10)}`,
      type: types[Math.floor(Math.random() * types.length)],
      resonance: Math.random(),
      cluster: `Cluster ${Math.floor(Math.random() * 5) + 1}`,
    })
  }

  // Create links
  for (let i = 0; i < 50; i++) {
    const sourceIdx = Math.floor(Math.random() * nodes.length)
    let targetIdx = Math.floor(Math.random() * nodes.length)
    while (targetIdx === sourceIdx) {
      targetIdx = Math.floor(Math.random() * nodes.length)
    }
    links.push({
      source: nodes[sourceIdx].id,
      target: nodes[targetIdx].id,
      weight: Math.random(),
    })
  }

  return { nodes, links }
}

export function TopologyPage() {
  const navigate = useNavigate()
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null)
  const [hoveredNode, setHoveredNode] = useState<GraphNode | null>(null)
  const [layout, setLayout] = useState<'force' | 'hierarchical' | 'circular'>('force')
  const [zoomLevel, setZoomLevel] = useState(1)
  const [isFullscreen, setIsFullscreen] = useState(false)
  const [minEdgeWeight, setMinEdgeWeight] = useState(0.1)
  const [activeFilters, setActiveFilters] = useState<Set<GraphNode['type']>>(
    new Set(['wallet', 'contract', 'exchange', 'unknown'])
  )

  const { nodes: allNodes, links: allLinks } = useMemo(() => generateMockData(), [])

  // Filter nodes and links based on active filters and edge weight
  const { nodes, links } = useMemo(() => {
    const filteredNodes = allNodes.filter(n => activeFilters.has(n.type))
    const nodeIds = new Set(filteredNodes.map(n => n.id))
    const filteredLinks = allLinks.filter(
      l => nodeIds.has(l.source) && nodeIds.has(l.target) && l.weight >= minEdgeWeight
    )
    return { nodes: filteredNodes, links: filteredLinks }
  }, [allNodes, allLinks, activeFilters, minEdgeWeight])

  const stats = useMemo(() => {
    const communities = new Set(nodes.map(n => n.cluster)).size
    const criticalNodes = nodes.filter(n => n.resonance > 0.8).length
    return {
      nodes: nodes.length,
      edges: links.length,
      communities,
      criticalNodes,
    }
  }, [nodes, links])

  const displayNode = hoveredNode || selectedNode

  // Zoom controls
  const handleZoomIn = useCallback(() => {
    setZoomLevel(prev => Math.min(prev + 0.25, 3))
  }, [])

  const handleZoomOut = useCallback(() => {
    setZoomLevel(prev => Math.max(prev - 0.25, 0.5))
  }, [])

  const handleFullscreen = useCallback(() => {
    setIsFullscreen(prev => !prev)
  }, [])

  // Investigate entity - navigate to explorer with pre-filled search
  const handleInvestigateEntity = useCallback(() => {
    if (displayNode) {
      navigate(`/explorer?address=${encodeURIComponent(displayNode.label)}`)
    }
  }, [displayNode, navigate])

  // Toggle filter
  const toggleFilter = useCallback((type: GraphNode['type']) => {
    setActiveFilters(prev => {
      const newFilters = new Set(prev)
      if (newFilters.has(type)) {
        newFilters.delete(type)
      } else {
        newFilters.add(type)
      }
      return newFilters
    })
  }, [])

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Network Topology</h1>
          <p className="text-muted-foreground">
            Interactive entity relationship graph visualization
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            variant={layout === 'force' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setLayout('force')}
          >
            Force
          </Button>
          <Button
            variant={layout === 'hierarchical' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setLayout('hierarchical')}
          >
            Hierarchical
          </Button>
          <Button
            variant={layout === 'circular' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setLayout('circular')}
          >
            Circular
          </Button>
        </div>
      </div>

      {/* Stats */}
      <div className="grid gap-4 sm:grid-cols-4">
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold">{stats.nodes}</div>
            <p className="text-sm text-muted-foreground">Nodes</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold">{stats.edges}</div>
            <p className="text-sm text-muted-foreground">Edges</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-accent">{stats.communities}</div>
            <p className="text-sm text-muted-foreground">Communities</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-warning">{stats.criticalNodes}</div>
            <p className="text-sm text-muted-foreground">Critical Nodes</p>
          </CardContent>
        </Card>
      </div>

      {/* Main Content */}
      <div className="grid gap-6 lg:grid-cols-4">
        {/* Graph Visualization */}
        <Card className={cn("lg:col-span-3", isFullscreen && "fixed inset-4 z-50 lg:col-span-1")}>
          <CardHeader className="flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-lg">
              Entity Graph
              <span className="ml-2 text-xs text-muted-foreground font-normal">
                (Zoom: {(zoomLevel * 100).toFixed(0)}%)
              </span>
            </CardTitle>
            <div className="flex gap-1">
              <Button variant="ghost" size="icon" onClick={handleZoomIn} title="Zoom In">
                <ZoomIn className="h-4 w-4" />
              </Button>
              <Button variant="ghost" size="icon" onClick={handleZoomOut} title="Zoom Out">
                <ZoomOut className="h-4 w-4" />
              </Button>
              <Button variant="ghost" size="icon" onClick={handleFullscreen} title="Toggle Fullscreen">
                <Maximize2 className="h-4 w-4" />
              </Button>
            </div>
          </CardHeader>
          <CardContent>
            <div style={{ transform: `scale(${zoomLevel})`, transformOrigin: 'top left' }}>
              <ForceGraph
                nodes={nodes}
                links={links}
                width={isFullscreen ? window.innerWidth - 100 : 800}
                height={isFullscreen ? window.innerHeight - 200 : 500}
                onNodeClick={setSelectedNode}
                onNodeHover={setHoveredNode}
              />
            </div>
          </CardContent>
        </Card>

        {/* Detail Panel */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Details</CardTitle>
            <CardDescription>
              {displayNode ? 'Node properties' : 'Select a node to view details'}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {displayNode ? (
              <div className="space-y-4 animate-in">
                <div>
                  <p className="text-xs text-muted-foreground">Address</p>
                  <code className="text-sm font-address">{formatAddress(displayNode.label, 10)}</code>
                </div>
                <Separator />
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-xs text-muted-foreground">Type</p>
                    <Badge variant="outline" className="mt-1 capitalize">
                      {displayNode.type}
                    </Badge>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">Resonance</p>
                    <p className={cn('text-lg font-bold', getResonanceColor(displayNode.resonance))}>
                      {formatResonance(displayNode.resonance)}
                    </p>
                  </div>
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">Community</p>
                  <Badge className="mt-1">{displayNode.cluster}</Badge>
                </div>
                <Separator />
                <Button className="w-full" size="sm" onClick={handleInvestigateEntity}>
                  <Search className="mr-2 h-4 w-4" />
                  Investigate Entity
                </Button>
              </div>
            ) : (
              <div className="py-8 text-center text-sm text-muted-foreground">
                Click on a node in the graph to view its properties
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* Filters */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-lg flex items-center gap-2">
            <Filter className="h-5 w-5" />
            Filters
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Node Type</label>
              <div className="flex gap-2">
                <Badge
                  variant={activeFilters.has('wallet') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleFilter('wallet')}
                >
                  Wallets ({allNodes.filter(n => n.type === 'wallet').length})
                </Badge>
                <Badge
                  variant={activeFilters.has('contract') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleFilter('contract')}
                >
                  Contracts ({allNodes.filter(n => n.type === 'contract').length})
                </Badge>
                <Badge
                  variant={activeFilters.has('exchange') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleFilter('exchange')}
                >
                  Exchanges ({allNodes.filter(n => n.type === 'exchange').length})
                </Badge>
                <Badge
                  variant={activeFilters.has('unknown') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleFilter('unknown')}
                >
                  Unknown ({allNodes.filter(n => n.type === 'unknown').length})
                </Badge>
              </div>
            </div>
            <Separator orientation="vertical" className="h-auto" />
            <div className="space-y-2">
              <label className="text-sm font-medium">Min Edge Weight</label>
              <Input
                type="number"
                value={minEdgeWeight}
                onChange={(e) => setMinEdgeWeight(parseFloat(e.target.value) || 0)}
                step="0.1"
                min="0"
                max="1"
                className="w-24"
              />
            </div>
            <Separator orientation="vertical" className="h-auto" />
            <div className="space-y-2">
              <label className="text-sm font-medium">Showing</label>
              <Badge variant="success">
                {nodes.length} nodes, {links.length} edges
              </Badge>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
