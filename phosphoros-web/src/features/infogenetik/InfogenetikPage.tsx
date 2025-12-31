import { useState, useMemo } from 'react'
import {
  Dna,
  Plus,
  Trash2,
  Play,
  Square,
  RotateCcw,
  Sparkles,
  Hexagon,
  Activity,
  Layers,
  Target,
  Zap,
} from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useInfogenetikStore, InfogenomData, Point5D } from '@/stores/infogenetik'
import { cn, formatResonance, getResonanceColor } from '@/lib/utils'

export function InfogenetikPage() {
  const {
    infogenoms,
    activeInfogenomId,
    seedPhrase,
    embeddings,
    explorationResults,
    isExploring,
    explorationProgress,
    operators,
    createInfogenom,
    deleteInfogenom,
    setActiveInfogenom,
    setSeedPhrase,
    embedSeedPhrase,
    startExploration,
    stopExploration,
    setOperators,
    resetOperators,
  } = useInfogenetikStore()

  const [newInfogenomName, setNewInfogenomName] = useState('')
  const [newInfogenomCells, setNewInfogenomCells] = useState(8)
  const [seedInput, setSeedInput] = useState('')
  const [maxSteps, setMaxSteps] = useState(1000)

  const activeInfogenom = useMemo(
    () => infogenoms.find((i) => i.id === activeInfogenomId),
    [infogenoms, activeInfogenomId]
  )

  const latestResult = useMemo(
    () =>
      explorationResults
        .filter((r) => r.infogenomId === activeInfogenomId)
        .sort((a, b) => b.completedAt.getTime() - a.completedAt.getTime())[0],
    [explorationResults, activeInfogenomId]
  )

  const handleCreateInfogenom = () => {
    if (!newInfogenomName.trim()) return
    createInfogenom(newInfogenomName.trim(), newInfogenomCells)
    setNewInfogenomName('')
  }

  const handleParseSeedPhrase = () => {
    const words = seedInput
      .trim()
      .toLowerCase()
      .split(/\s+/)
      .filter((w) => w.length > 0)
    if (words.length > 0) {
      setSeedPhrase(words)
      embedSeedPhrase()
    }
  }

  const handleStartExploration = () => {
    if (activeInfogenomId && embeddings.length > 0) {
      startExploration(maxSteps)
    }
  }

  const Point5DDisplay = ({ point, label }: { point: Point5D; label?: string }) => (
    <div className="space-y-1">
      {label && <p className="text-xs text-muted-foreground">{label}</p>}
      <div className="grid grid-cols-5 gap-1 text-xs font-mono">
        {(['x', 'y', 'z', 'w', 'v'] as const).map((dim) => (
          <div key={dim} className="text-center">
            <span className="text-muted-foreground">{dim}:</span>
            <span className="ml-1">{point[dim].toFixed(3)}</span>
          </div>
        ))}
      </div>
    </div>
  )

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight flex items-center gap-3">
            <Dna className="h-8 w-8 text-primary" />
            Infogenetik
          </h1>
          <p className="text-muted-foreground">
            5D Information Genetics - Quantum-inspired keyspace exploration
          </p>
        </div>
        <div className="flex gap-2">
          {isExploring ? (
            <Button variant="destructive" onClick={stopExploration}>
              <Square className="mr-2 h-4 w-4" />
              Stop Exploration
            </Button>
          ) : (
            <Button
              onClick={handleStartExploration}
              disabled={!activeInfogenomId || embeddings.length === 0}
            >
              <Play className="mr-2 h-4 w-4" />
              Start Exploration
            </Button>
          )}
        </div>
      </div>

      {/* Progress Bar */}
      {isExploring && (
        <Card>
          <CardContent className="py-4">
            <div className="flex items-center gap-4">
              <Activity className="h-5 w-5 text-primary animate-pulse" />
              <div className="flex-1">
                <div className="flex justify-between text-sm mb-1">
                  <span>Exploring keyspace with QDASH algorithm...</span>
                  <span>{explorationProgress.toFixed(0)}%</span>
                </div>
                <div className="h-2 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-primary to-accent transition-all duration-300"
                    style={{ width: `${explorationProgress}%` }}
                  />
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Main Grid */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Infogenom Management */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Hexagon className="h-5 w-5" />
              Infogenoms
            </CardTitle>
            <CardDescription>Networks of coupled Gabriel Cells</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Create new */}
            <div className="space-y-2">
              <div className="flex gap-2">
                <Input
                  placeholder="Infogenom name..."
                  value={newInfogenomName}
                  onChange={(e) => setNewInfogenomName(e.target.value)}
                  className="flex-1"
                />
                <Input
                  type="number"
                  value={newInfogenomCells}
                  onChange={(e) => setNewInfogenomCells(parseInt(e.target.value) || 4)}
                  className="w-20"
                  min={2}
                  max={32}
                />
              </div>
              <Button
                onClick={handleCreateInfogenom}
                className="w-full"
                disabled={!newInfogenomName.trim()}
              >
                <Plus className="mr-2 h-4 w-4" />
                Create Infogenom ({newInfogenomCells} cells)
              </Button>
            </div>

            <Separator />

            {/* Infogenom list */}
            <ScrollArea className="h-[200px]">
              <div className="space-y-2">
                {infogenoms.length === 0 ? (
                  <p className="text-sm text-muted-foreground text-center py-4">
                    No Infogenoms created yet
                  </p>
                ) : (
                  infogenoms.map((infogenom) => (
                    <div
                      key={infogenom.id}
                      className={cn(
                        'flex items-center justify-between rounded-lg border p-3 cursor-pointer transition-colors',
                        activeInfogenomId === infogenom.id
                          ? 'border-primary bg-primary/5'
                          : 'hover:bg-muted/50'
                      )}
                      onClick={() => setActiveInfogenom(infogenom.id)}
                    >
                      <div>
                        <p className="font-medium">{infogenom.name}</p>
                        <p className="text-xs text-muted-foreground">
                          {infogenom.numCells} cells • κ={infogenom.couplingStrength}
                        </p>
                      </div>
                      <div className="flex items-center gap-2">
                        <Badge
                          variant={
                            infogenom.status === 'exploring'
                              ? 'default'
                              : infogenom.status === 'evaluating'
                              ? 'secondary'
                              : 'outline'
                          }
                        >
                          {infogenom.status}
                        </Badge>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-8 w-8"
                          onClick={(e) => {
                            e.stopPropagation()
                            deleteInfogenom(infogenom.id)
                          }}
                        >
                          <Trash2 className="h-4 w-4 text-destructive" />
                        </Button>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </ScrollArea>
          </CardContent>
        </Card>

        {/* Seed Phrase Embedding */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Sparkles className="h-5 w-5" />
              Seed Phrase Embedding
            </CardTitle>
            <CardDescription>Embed BIP39 mnemonic into 5D Metatron space</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <textarea
                placeholder="Enter BIP39 seed phrase (12 or 24 words)..."
                value={seedInput}
                onChange={(e) => setSeedInput(e.target.value)}
                className="w-full h-24 rounded-md border bg-background px-3 py-2 text-sm resize-none"
              />
              <Button onClick={handleParseSeedPhrase} className="w-full">
                <Layers className="mr-2 h-4 w-4" />
                Embed in 5D Space
              </Button>
            </div>

            {seedPhrase.length > 0 && (
              <>
                <Separator />
                <div>
                  <p className="text-sm font-medium mb-2">
                    Words: {seedPhrase.length}
                  </p>
                  <div className="flex flex-wrap gap-1">
                    {seedPhrase.map((word, i) => (
                      <Badge key={i} variant="secondary" className="text-xs">
                        {i + 1}. {word}
                      </Badge>
                    ))}
                  </div>
                </div>
              </>
            )}

            {embeddings.length > 0 && (
              <>
                <Separator />
                <div>
                  <p className="text-sm font-medium mb-2">Embeddings ({embeddings.length})</p>
                  <ScrollArea className="h-[120px]">
                    <div className="space-y-2">
                      {embeddings.slice(0, 5).map((emb, i) => (
                        <div key={i} className="rounded bg-muted/50 p-2">
                          <Point5DDisplay point={emb} label={`Word ${i + 1}: ${seedPhrase[i]}`} />
                        </div>
                      ))}
                      {embeddings.length > 5 && (
                        <p className="text-xs text-muted-foreground text-center">
                          +{embeddings.length - 5} more embeddings
                        </p>
                      )}
                    </div>
                  </ScrollArea>
                </div>
              </>
            )}
          </CardContent>
        </Card>

        {/* Operators & Controls */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Zap className="h-5 w-5" />
              Topological Operators
            </CardTitle>
            <CardDescription>Configure resonance calculation parameters</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-3 grid-cols-2">
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">WT (Wave)</label>
                <Input
                  type="number"
                  value={operators.waveTransform}
                  onChange={(e) =>
                    setOperators({ waveTransform: parseFloat(e.target.value) || 0 })
                  }
                  step="0.1"
                  className="h-8"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">SW (Spectral)</label>
                <Input
                  type="number"
                  value={operators.spectralWeight}
                  onChange={(e) =>
                    setOperators({ spectralWeight: parseFloat(e.target.value) || 0 })
                  }
                  step="0.1"
                  className="h-8"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">DK (Double-Kick)</label>
                <Input
                  type="number"
                  value={operators.doubleKick}
                  onChange={(e) =>
                    setOperators({ doubleKick: parseFloat(e.target.value) || 0 })
                  }
                  step="0.1"
                  className="h-8"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">PI (Phase)</label>
                <Input
                  type="number"
                  value={operators.phaseIntegral}
                  onChange={(e) =>
                    setOperators({ phaseIntegral: parseFloat(e.target.value) || 0 })
                  }
                  step="0.1"
                  className="h-8"
                />
              </div>
            </div>

            <Separator />

            <div className="space-y-1.5">
              <label className="text-xs font-medium text-muted-foreground">Max Steps</label>
              <Input
                type="number"
                value={maxSteps}
                onChange={(e) => setMaxSteps(parseInt(e.target.value) || 100)}
                min={100}
                max={10000}
                step={100}
                className="h-8"
              />
            </div>

            <div className="flex gap-2 pt-2">
              <Button variant="outline" size="sm" className="flex-1" onClick={resetOperators}>
                <RotateCcw className="mr-2 h-4 w-4" />
                Reset
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Results Section */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Active Infogenom Details */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Target className="h-5 w-5" />
              Active Infogenom
            </CardTitle>
          </CardHeader>
          <CardContent>
            {activeInfogenom ? (
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="text-lg font-semibold">{activeInfogenom.name}</h3>
                    <p className="text-sm text-muted-foreground">
                      {activeInfogenom.numCells} Gabriel Cells
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-xs text-muted-foreground">Total Resonance</p>
                    <p
                      className={cn(
                        'text-2xl font-bold font-mono',
                        getResonanceColor(activeInfogenom.totalResonance)
                      )}
                    >
                      {formatResonance(activeInfogenom.totalResonance)}
                    </p>
                  </div>
                </div>

                <Separator />

                {/* Gabriel Cells Visualization */}
                <div>
                  <p className="text-sm font-medium mb-2">Gabriel Cell Network</p>
                  <div className="flex flex-wrap gap-2">
                    {Array.from({ length: activeInfogenom.numCells }).map((_, i) => (
                      <div
                        key={i}
                        className={cn(
                          'w-10 h-10 rounded-lg flex items-center justify-center text-xs font-mono border',
                          activeInfogenom.status === 'exploring'
                            ? 'bg-primary/20 border-primary animate-pulse'
                            : 'bg-muted border-muted-foreground/20'
                        )}
                      >
                        G{i}
                      </div>
                    ))}
                  </div>
                  <p className="text-xs text-muted-foreground mt-2">
                    Coupling Strength: κ = {activeInfogenom.couplingStrength}
                  </p>
                </div>
              </div>
            ) : (
              <div className="py-8 text-center text-sm text-muted-foreground">
                Select or create an Infogenom to begin
              </div>
            )}
          </CardContent>
        </Card>

        {/* Exploration Results */}
        <Card>
          <CardHeader>
            <CardTitle>Latest Exploration Result</CardTitle>
            <CardDescription>QDASH Solve-Coagula algorithm output</CardDescription>
          </CardHeader>
          <CardContent>
            {latestResult ? (
              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="rounded-lg bg-muted p-3">
                    <p className="text-xs text-muted-foreground">Steps Completed</p>
                    <p className="text-xl font-bold">{latestResult.steps.toLocaleString()}</p>
                  </div>
                  <div className="rounded-lg bg-muted p-3">
                    <p className="text-xs text-muted-foreground">Best Resonance</p>
                    <p
                      className={cn(
                        'text-xl font-bold font-mono',
                        getResonanceColor(latestResult.bestResonance)
                      )}
                    >
                      {formatResonance(latestResult.bestResonance)}
                    </p>
                  </div>
                </div>

                <Separator />

                <div>
                  <p className="text-sm font-medium mb-2">Best Point in 5D Space</p>
                  <div className="rounded-lg bg-muted p-3">
                    <Point5DDisplay point={latestResult.bestPoint} />
                  </div>
                </div>

                <div>
                  <p className="text-sm font-medium mb-2">
                    Trajectory ({latestResult.trajectory.length} points)
                  </p>
                  <div className="h-20 rounded-lg bg-muted p-2 flex items-end gap-0.5">
                    {latestResult.trajectory.slice(-50).map((point, i) => {
                      const avgDim =
                        (point.x + point.y + point.z + point.w + point.v) / 5
                      return (
                        <div
                          key={i}
                          className="flex-1 bg-primary/60 rounded-t"
                          style={{ height: `${avgDim * 100}%` }}
                        />
                      )
                    })}
                  </div>
                </div>

                <p className="text-xs text-muted-foreground text-right">
                  Completed: {latestResult.completedAt.toLocaleString()}
                </p>
              </div>
            ) : (
              <div className="py-8 text-center text-sm text-muted-foreground">
                No exploration results yet. Start an exploration to see results.
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* History */}
      <Card>
        <CardHeader>
          <CardTitle>Exploration History</CardTitle>
          <CardDescription>Previous keyspace explorations</CardDescription>
        </CardHeader>
        <CardContent>
          {explorationResults.length === 0 ? (
            <p className="text-sm text-muted-foreground text-center py-4">
              No exploration history yet
            </p>
          ) : (
            <ScrollArea className="h-[200px]">
              <div className="space-y-2">
                {[...explorationResults]
                  .sort((a, b) => b.completedAt.getTime() - a.completedAt.getTime())
                  .map((result, i) => {
                    const infogenom = infogenoms.find((ig) => ig.id === result.infogenomId)
                    return (
                      <div
                        key={i}
                        className="flex items-center justify-between rounded-lg border p-3"
                      >
                        <div>
                          <p className="font-medium">{infogenom?.name || 'Unknown'}</p>
                          <p className="text-xs text-muted-foreground">
                            {result.steps.toLocaleString()} steps •{' '}
                            {result.completedAt.toLocaleString()}
                          </p>
                        </div>
                        <div
                          className={cn(
                            'text-lg font-bold font-mono',
                            getResonanceColor(result.bestResonance)
                          )}
                        >
                          {formatResonance(result.bestResonance)}
                        </div>
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
