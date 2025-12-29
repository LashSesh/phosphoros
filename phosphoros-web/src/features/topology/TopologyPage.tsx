import { useState, useMemo } from 'react'
import { ZoomIn, ZoomOut, Maximize2, Filter } from 'lucide-react'
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
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null)
  const [hoveredNode, setHoveredNode] = useState<GraphNode | null>(null)
  const [layout, setLayout] = useState<'force' | 'hierarchical' | 'circular'>('force')

  const { nodes, links } = useMemo(() => generateMockData(), [])

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
        <Card className="lg:col-span-3">
          <CardHeader className="flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-lg">Entity Graph</CardTitle>
            <div className="flex gap-1">
              <Button variant="ghost" size="icon">
                <ZoomIn className="h-4 w-4" />
              </Button>
              <Button variant="ghost" size="icon">
                <ZoomOut className="h-4 w-4" />
              </Button>
              <Button variant="ghost" size="icon">
                <Maximize2 className="h-4 w-4" />
              </Button>
            </div>
          </CardHeader>
          <CardContent>
            <ForceGraph
              nodes={nodes}
              links={links}
              width={800}
              height={500}
              onNodeClick={setSelectedNode}
              onNodeHover={setHoveredNode}
            />
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
                <Button className="w-full" size="sm">
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
                <Badge variant="default">Wallets</Badge>
                <Badge variant="outline">Contracts</Badge>
                <Badge variant="outline">Exchanges</Badge>
              </div>
            </div>
            <Separator orientation="vertical" className="h-auto" />
            <div className="space-y-2">
              <label className="text-sm font-medium">Min Edge Weight</label>
              <Input type="number" defaultValue="0.1" step="0.1" className="w-24" />
            </div>
            <Separator orientation="vertical" className="h-auto" />
            <div className="space-y-2">
              <label className="text-sm font-medium">Show Communities</label>
              <Badge variant="success">Enabled</Badge>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
