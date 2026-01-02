import { useState, useEffect } from 'react'
import {
  Network, Plus, Trash2, Play, Loader2, AlertCircle,
  CheckCircle2, Users, BarChart3, Box, ChevronDown, ChevronRight
} from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { Label } from '@/components/ui/label'
import {
  useComputeClusters,
  useAllClusters,
  useClustersForSnapshot,
  useClusterMembers,
  type Entity,
  type ClusterSummary,
} from '@/hooks/useApi'
import { useWebSocket } from '@/hooks/useWebSocket'

type Algorithm = 'knn' | 'dbscan' | 'hierarchical'

interface EntityInput extends Entity {
  id: string
}

export function ClusterPage() {
  // Form state
  const [snapshotId, setSnapshotId] = useState('')
  const [entities, setEntities] = useState<EntityInput[]>([])
  const [newAddress, setNewAddress] = useState('')
  const [newFeatures, setNewFeatures] = useState('')
  const [algorithm, setAlgorithm] = useState<Algorithm>('knn')
  const [k, setK] = useState(3)
  const [threshold, setThreshold] = useState(0.5)

  // Cluster exploration state
  const [selectedSnapshot, setSelectedSnapshot] = useState<string | null>(null)
  const [expandedClusters, setExpandedClusters] = useState<Set<string>>(new Set())

  // API hooks
  const { mutate: computeClusters, isPending: isComputing, error: computeError } = useComputeClusters()
  const { data: allClustersData, isLoading: isLoadingAll, refetch: refetchAll } = useAllClusters()

  // Get clusters for selected snapshot
  const { data: snapshotClusters } = useClustersForSnapshot(
    selectedSnapshot || '',
    !!selectedSnapshot
  )

  // WebSocket for real-time updates
  const [lastClusterEvent, setLastClusterEvent] = useState<{
    snapshotId: string
    numClusters: number
    timestamp: string
  } | null>(null)

  useWebSocket({
    onEvent: (event) => {
      if (event.type === 'ClusterComputed') {
        setLastClusterEvent({
          snapshotId: event.snapshot_id,
          numClusters: event.num_clusters,
          timestamp: event.timestamp,
        })
        // Refresh cluster list
        refetchAll()
      }
    },
  })

  // Clear event notification after 5 seconds
  useEffect(() => {
    if (lastClusterEvent) {
      const timeout = setTimeout(() => setLastClusterEvent(null), 5000)
      return () => clearTimeout(timeout)
    }
  }, [lastClusterEvent])

  const handleAddEntity = () => {
    if (!newAddress.trim() || !newFeatures.trim()) return

    const features = newFeatures
      .split(',')
      .map((f) => parseFloat(f.trim()))
      .filter((f) => !isNaN(f))

    if (features.length === 0) {
      alert('Please enter valid feature values (comma-separated numbers)')
      return
    }

    const newEntity: EntityInput = {
      id: crypto.randomUUID(),
      address: newAddress.trim(),
      features,
    }

    setEntities([...entities, newEntity])
    setNewAddress('')
    setNewFeatures('')
  }

  const handleRemoveEntity = (id: string) => {
    setEntities(entities.filter((e) => e.id !== id))
  }

  const handleComputeClusters = () => {
    if (!snapshotId.trim() || entities.length === 0) {
      alert('Please provide a snapshot ID and at least one entity')
      return
    }

    if (algorithm === 'knn' && k < 1) {
      alert('k must be at least 1')
      return
    }

    if ((algorithm === 'dbscan' || algorithm === 'hierarchical') && threshold <= 0) {
      alert('Threshold must be greater than 0')
      return
    }

    const request = {
      snapshot_id: snapshotId.trim(),
      entities: entities.map(({ id, ...rest }) => rest),
      algorithm,
      k: algorithm === 'knn' ? k : undefined,
      threshold: algorithm !== 'knn' ? threshold : undefined,
    }

    computeClusters(request, {
      onSuccess: (response) => {
        if (response.success) {
          setSelectedSnapshot(snapshotId.trim())
        } else {
          alert(`Clustering failed: ${response.error}`)
        }
      },
      onError: (error) => {
        alert(`Clustering error: ${error.message}`)
      },
    })
  }

  const toggleClusterExpand = (clusterId: string) => {
    const newExpanded = new Set(expandedClusters)
    if (newExpanded.has(clusterId)) {
      newExpanded.delete(clusterId)
    } else {
      newExpanded.add(clusterId)
    }
    setExpandedClusters(newExpanded)
  }

  const snapshots = allClustersData?.snapshots || {}
  const snapshotList = Object.keys(snapshots)

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Cluster Analysis</h1>
        <p className="text-muted-foreground">
          Entity clustering with KNN, DBSCAN, and Hierarchical algorithms
        </p>
      </div>

      {/* WebSocket Event Notification */}
      {lastClusterEvent && (
        <Card className="border-green-500 bg-green-50 dark:bg-green-950">
          <CardContent className="pt-6">
            <div className="flex items-center gap-2 text-green-700 dark:text-green-300">
              <CheckCircle2 className="h-5 w-5" />
              <span className="font-medium">
                Computed {lastClusterEvent.numClusters} cluster
                {lastClusterEvent.numClusters !== 1 ? 's' : ''} for snapshot{' '}
                {lastClusterEvent.snapshotId}
              </span>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Cluster Computation Form */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Network className="h-5 w-5" />
            Compute Clusters
          </CardTitle>
          <CardDescription>
            Add entities with feature vectors and run clustering algorithms
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Snapshot ID */}
          <div className="space-y-2">
            <Label htmlFor="snapshot">Snapshot ID</Label>
            <Input
              id="snapshot"
              placeholder="my-cluster-snapshot"
              value={snapshotId}
              onChange={(e) => setSnapshotId(e.target.value)}
            />
          </div>

          {/* Entity Input */}
          <div className="space-y-3">
            <Label>Add Entities</Label>
            <div className="flex gap-2">
              <Input
                placeholder="Address (e.g., 0x1234...)"
                value={newAddress}
                onChange={(e) => setNewAddress(e.target.value)}
                className="flex-1"
              />
              <Input
                placeholder="Features (e.g., 0.5, 1.2, 0.8)"
                value={newFeatures}
                onChange={(e) => setNewFeatures(e.target.value)}
                className="flex-1"
              />
              <Button onClick={handleAddEntity} size="icon">
                <Plus className="h-4 w-4" />
              </Button>
            </div>
            <p className="text-xs text-muted-foreground">
              Enter comma-separated feature values for each entity
            </p>
          </div>

          {/* Entity List */}
          {entities.length > 0 && (
            <div className="space-y-2">
              <Label>Entities ({entities.length})</Label>
              <ScrollArea className="h-[200px] rounded-md border p-3">
                <div className="space-y-2">
                  {entities.map((entity) => (
                    <div
                      key={entity.id}
                      className="flex items-center gap-2 rounded-md bg-muted/50 p-2"
                    >
                      <code className="text-xs flex-1 truncate">
                        {entity.address} → [{entity.features.join(', ')}]
                      </code>
                      <Button
                        variant="ghost"
                        size="icon"
                        className="h-6 w-6 flex-shrink-0"
                        onClick={() => handleRemoveEntity(entity.id)}
                      >
                        <Trash2 className="h-3 w-3 text-red-500" />
                      </Button>
                    </div>
                  ))}
                </div>
              </ScrollArea>
            </div>
          )}

          <Separator />

          {/* Algorithm Selection */}
          <div className="space-y-3">
            <Label>Algorithm</Label>
            <div className="grid grid-cols-3 gap-3">
              {(['knn', 'dbscan', 'hierarchical'] as const).map((alg) => (
                <div
                  key={alg}
                  className={`flex items-center justify-center rounded-md border p-3 cursor-pointer transition-colors ${
                    algorithm === alg
                      ? 'border-primary bg-primary/5'
                      : 'border-muted hover:border-primary/50'
                  }`}
                  onClick={() => setAlgorithm(alg)}
                >
                  <span className="text-sm font-medium uppercase">{alg}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Parameters */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="k">k (for KNN)</Label>
              <Input
                id="k"
                type="number"
                min="1"
                value={k}
                onChange={(e) => setK(parseInt(e.target.value) || 1)}
                disabled={algorithm !== 'knn'}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="threshold">Threshold (for DBSCAN/Hierarchical)</Label>
              <Input
                id="threshold"
                type="number"
                step="0.01"
                min="0.01"
                value={threshold}
                onChange={(e) => setThreshold(parseFloat(e.target.value) || 0.1)}
                disabled={algorithm === 'knn'}
              />
            </div>
          </div>

          {/* Error Display */}
          {computeError && (
            <div className="flex items-start gap-2 rounded-md border border-red-500 bg-red-50 dark:bg-red-950 p-3">
              <AlertCircle className="h-5 w-5 text-red-500 mt-0.5" />
              <div className="flex-1">
                <p className="text-sm font-medium text-red-700 dark:text-red-300">
                  {computeError.message}
                </p>
              </div>
            </div>
          )}

          {/* Compute Button */}
          <Button
            onClick={handleComputeClusters}
            disabled={
              !snapshotId.trim() || entities.length === 0 || isComputing
            }
            className="w-full"
          >
            {isComputing ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Computing Clusters...
              </>
            ) : (
              <>
                <Play className="mr-2 h-4 w-4" />
                Compute Clusters
              </>
            )}
          </Button>
        </CardContent>
      </Card>

      {/* Cluster Results */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Cluster Snapshots ({snapshotList.length})</CardTitle>
            {isLoadingAll && (
              <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
            )}
          </div>
          <CardDescription>
            View computed clusters from all snapshots
          </CardDescription>
        </CardHeader>
        <CardContent>
          {snapshotList.length === 0 ? (
            <div className="text-center py-12 text-muted-foreground">
              <Network className="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p>No clusters computed yet</p>
              <p className="text-sm mt-2">
                Use the form above to compute clusters for entities
              </p>
            </div>
          ) : (
            <ScrollArea className="h-[500px]">
              <div className="space-y-4">
                {snapshotList.map((snapshot) => {
                  const clusters = snapshots[snapshot] || []
                  return (
                    <div key={snapshot} className="rounded-lg border p-4 space-y-3">
                      {/* Snapshot Header */}
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <Box className="h-5 w-5 text-blue-500" />
                          <div>
                            <h3 className="font-medium">{snapshot}</h3>
                            <p className="text-xs text-muted-foreground">
                              {clusters.length} cluster{clusters.length !== 1 ? 's' : ''}
                            </p>
                          </div>
                        </div>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() =>
                            setSelectedSnapshot(selectedSnapshot === snapshot ? null : snapshot)
                          }
                        >
                          {selectedSnapshot === snapshot ? 'Hide' : 'View'}
                        </Button>
                      </div>

                      {/* Cluster List */}
                      {selectedSnapshot === snapshot && (
                        <>
                          <Separator />
                          <div className="space-y-2">
                            {clusters.map((cluster) => (
                              <ClusterItem
                                key={cluster.id}
                                cluster={cluster}
                                snapshotId={snapshot}
                                isExpanded={expandedClusters.has(cluster.id)}
                                onToggle={() => toggleClusterExpand(cluster.id)}
                              />
                            ))}
                          </div>
                        </>
                      )}
                    </div>
                  )
                })}
              </div>
            </ScrollArea>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

// ============================================================================
// Cluster Item Component with Member Exploration
// ============================================================================

interface ClusterItemProps {
  cluster: ClusterSummary
  snapshotId: string
  isExpanded: boolean
  onToggle: () => void
}

function ClusterItem({ cluster, snapshotId, isExpanded, onToggle }: ClusterItemProps) {
  const { data: membersData, isLoading: isLoadingMembers } = useClusterMembers(
    snapshotId,
    cluster.id,
    isExpanded
  )

  const members = membersData?.members || []

  return (
    <div className="rounded-md border p-3 space-y-2">
      {/* Cluster Header */}
      <div
        className="flex items-center justify-between cursor-pointer"
        onClick={onToggle}
      >
        <div className="flex items-center gap-2">
          {isExpanded ? (
            <ChevronDown className="h-4 w-4 text-muted-foreground" />
          ) : (
            <ChevronRight className="h-4 w-4 text-muted-foreground" />
          )}
          <BarChart3 className="h-4 w-4 text-purple-500" />
          <span className="font-medium text-sm">
            {cluster.label || `Cluster ${cluster.id}`}
          </span>
        </div>
        <div className="flex items-center gap-2">
          <Badge variant="outline" className="text-xs">
            <Users className="h-3 w-3 mr-1" />
            {cluster.size}
          </Badge>
          <Badge variant="outline" className="text-xs">
            Cohesion: {cluster.cohesion.toFixed(3)}
          </Badge>
        </div>
      </div>

      {/* Cluster Members */}
      {isExpanded && (
        <div className="ml-6 space-y-1">
          {isLoadingMembers ? (
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <Loader2 className="h-3 w-3 animate-spin" />
              Loading members...
            </div>
          ) : members.length > 0 ? (
            members.map((member, idx) => (
              <div
                key={`${member}-${idx}`}
                className="text-xs font-mono bg-muted/30 rounded px-2 py-1"
              >
                {member}
              </div>
            ))
          ) : (
            <p className="text-xs text-muted-foreground">No members found</p>
          )}
        </div>
      )}
    </div>
  )
}
