import { useState } from 'react'
import { Network, ZoomIn, ZoomOut, Maximize2, Filter } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'

export function TopologyPage() {
  const [selectedNode, setSelectedNode] = useState<string | null>(null)
  const [layout, setLayout] = useState<'force' | 'hierarchical' | 'circular'>('force')

  const mockStats = {
    nodes: 156,
    edges: 423,
    communities: 8,
    criticalNodes: 12,
  }

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
            <div className="text-2xl font-bold">{mockStats.nodes}</div>
            <p className="text-sm text-muted-foreground">Nodes</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold">{mockStats.edges}</div>
            <p className="text-sm text-muted-foreground">Edges</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-accent">{mockStats.communities}</div>
            <p className="text-sm text-muted-foreground">Communities</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-warning">{mockStats.criticalNodes}</div>
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
            <div className="h-[500px] rounded-lg border-2 border-dashed border-muted flex items-center justify-center bg-muted/20">
              <div className="text-center">
                <Network className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
                <p className="text-muted-foreground">D3.js force-directed graph will render here</p>
                <p className="text-sm text-muted-foreground mt-2">Layout: {layout}</p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Detail Panel */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Details</CardTitle>
            <CardDescription>
              {selectedNode ? 'Node properties' : 'Select a node to view details'}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {selectedNode ? (
              <div className="space-y-4">
                <div>
                  <p className="text-xs text-muted-foreground">Address</p>
                  <code className="text-sm font-address">{selectedNode}</code>
                </div>
                <Separator />
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-xs text-muted-foreground">Connections</p>
                    <p className="text-lg font-bold">24</p>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">Centrality</p>
                    <p className="text-lg font-bold">0.82</p>
                  </div>
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">Community</p>
                  <Badge>Cluster #3</Badge>
                </div>
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
