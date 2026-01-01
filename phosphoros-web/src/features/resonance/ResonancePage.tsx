import { useState, useEffect, useCallback } from 'react'
import { Activity, Play, Pause, Settings2, RotateCcw, Zap } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import { SpectralGauge } from '@/components/charts/SpectralGauge'
import { ResonanceTimeSeries } from '@/components/charts/ResonanceTimeSeries'
import { cn, formatResonance, getResonanceColor } from '@/lib/utils'
import {
  useComputeSpectral,
  useAnalyzeResonance,
  useResonanceHistory,
  useClearResonanceHistory,
  type SpectralResponse,
  type AnalyzeResonanceResponse,
} from '@/hooks/useApi'
import { useWebSocket } from '@/hooks/useWebSocket'

interface TimeSeriesPoint {
  timestamp: Date
  psi: number
  rho: number
  omega: number
  resonance: number
}

interface Operators {
  wt: number
  sw: number
  dk: number
  pi: number
}

const DEFAULT_OPERATORS: Operators = {
  wt: 1.0,
  sw: 0.5,
  dk: 0.3,
  pi: 0.1,
}

export function ResonancePage() {
  const [isRunning, setIsRunning] = useState(false)
  const [progress, setProgress] = useState(0)
  const [spectralSignature, setSpectralSignature] = useState({
    psi: 0.85,
    rho: 0.92,
    omega: 0.78,
  })
  const [history, setHistory] = useState<TimeSeriesPoint[]>([])
  const [operators, setOperators] = useState<Operators>(DEFAULT_OPERATORS)
  const [pendingOperators, setPendingOperators] = useState<Operators>(DEFAULT_OPERATORS)
  const [lastEvaluation, setLastEvaluation] = useState<AnalyzeResonanceResponse | null>(null)

  // API Hooks
  const computeSpectral = useComputeSpectral()
  const analyzeResonance = useAnalyzeResonance()
  const { data: resonanceHistory, refetch: refetchHistory } = useResonanceHistory()
  const clearHistory = useClearResonanceHistory()

  // WebSocket for real-time updates
  useWebSocket({
    onEvent: (event) => {
      if (event.type === 'ResonanceEvaluated') {
        // Add to history on real evaluation
        const newPoint: TimeSeriesPoint = {
          timestamp: new Date(),
          psi: spectralSignature.psi,
          rho: spectralSignature.rho,
          omega: spectralSignature.omega,
          resonance: event.score || 0,
        }
        setHistory((prev) => [...prev.slice(-30), newPoint])
      }
    },
  })

  // Generate initial history
  useEffect(() => {
    const initialHistory: TimeSeriesPoint[] = []
    const now = Date.now()
    for (let i = 20; i >= 0; i--) {
      const psi = 0.6 + Math.random() * 0.35
      const rho = 0.7 + Math.random() * 0.25
      const omega = 0.5 + Math.random() * 0.4
      initialHistory.push({
        timestamp: new Date(now - i * 5000),
        psi,
        rho,
        omega,
        resonance: psi * rho * omega,
      })
    }
    setHistory(initialHistory)
  }, [])

  // Simulate real-time updates when running, influenced by operators
  useEffect(() => {
    if (!isRunning) return

    const interval = setInterval(() => {
      // Update spectral signature with operator-influenced variations
      const wtEffect = operators.wt * 0.05
      const swEffect = operators.sw * 0.03
      const dkSpike = Math.random() > 1 - operators.dk * 0.3 ? 0.1 : 0
      const piSmooth = operators.pi * 0.02

      setSpectralSignature((prev) => ({
        psi: Math.min(1, Math.max(0, prev.psi + (Math.random() - 0.5) * wtEffect + dkSpike)),
        rho: Math.min(1, Math.max(0, prev.rho + (Math.random() - 0.5) * swEffect)),
        omega: Math.min(1, Math.max(0, prev.omega + (Math.random() - 0.5) * (0.04 - piSmooth))),
      }))

      // Update progress
      setProgress((prev) => {
        const next = prev + Math.random() * 3
        if (next >= 100) {
          setIsRunning(false)
          return 100
        }
        return next
      })
    }, 500)

    return () => clearInterval(interval)
  }, [isRunning, operators])

  const resonance = spectralSignature.psi * spectralSignature.rho * spectralSignature.omega

  const toggleAnalysis = () => {
    if (!isRunning) {
      setProgress(0)
    }
    setIsRunning(!isRunning)
  }

  // Apply pending operators
  const handleApplyOperators = useCallback(() => {
    setOperators(pendingOperators)
  }, [pendingOperators])

  // Reset operators to defaults
  const handleResetOperators = useCallback(() => {
    setPendingOperators(DEFAULT_OPERATORS)
    setOperators(DEFAULT_OPERATORS)
  }, [])

  // Update pending operator value
  const updatePendingOperator = useCallback((key: keyof Operators, value: string) => {
    const numValue = parseFloat(value) || 0
    setPendingOperators((prev) => ({
      ...prev,
      [key]: Math.min(2, Math.max(0, numValue)),
    }))
  }, [])

  // Compute spectral metrics via API
  const handleComputeSpectral = useCallback(() => {
    computeSpectral.mutate(
      {
        psi: spectralSignature.psi,
        rho: spectralSignature.rho,
        omega: spectralSignature.omega,
      },
      {
        onSuccess: (data: SpectralResponse) => {
          console.log('Spectral computation result:', data)
          // Update signature with server-validated values
          setSpectralSignature({
            psi: data.psi,
            rho: data.rho,
            omega: data.omega,
          })

          // Add to history
          const newPoint: TimeSeriesPoint = {
            timestamp: new Date(),
            psi: data.psi,
            rho: data.rho,
            omega: data.omega,
            resonance: data.resonance,
          }
          setHistory((prev) => [...prev.slice(-30), newPoint])
        },
        onError: (error) => {
          console.error('Spectral computation failed:', error)
        },
      }
    )
  }, [spectralSignature, computeSpectral])

  // Evaluate resonance via API
  const handleEvaluateResonance = useCallback(() => {
    // Create 5D vectors from current state and operators
    const perception: [number, number, number, number, number] = [
      spectralSignature.psi,
      spectralSignature.rho,
      spectralSignature.omega,
      operators.wt,
      operators.sw,
    ]

    const intention: [number, number, number, number, number] = [
      operators.dk,
      operators.pi,
      spectralSignature.psi * 0.8,
      spectralSignature.rho * 0.9,
      spectralSignature.omega * 0.7,
    ]

    const gradient: [number, number, number, number, number] = [
      spectralSignature.psi - 0.5,
      spectralSignature.rho - 0.5,
      spectralSignature.omega - 0.5,
      operators.wt - 0.5,
      operators.sw - 0.5,
    ]

    analyzeResonance.mutate(
      {
        t: Date.now() / 1000, // Convert to seconds
        perception,
        intention,
        gradient,
        theta: resonance * Math.PI, // Convert resonance to phase angle
        label: `Eval-${Date.now()}`,
      },
      {
        onSuccess: (response: AnalyzeResonanceResponse) => {
          console.log('Resonance evaluation result:', response)
          setLastEvaluation(response)
          refetchHistory()
        },
        onError: (error) => {
          console.error('Resonance evaluation failed:', error)
        },
      }
    )
  }, [spectralSignature, operators, resonance, analyzeResonance, refetchHistory])

  // Clear history
  const handleClearHistory = useCallback(() => {
    clearHistory.mutate(undefined, {
      onSuccess: () => {
        setHistory([])
        refetchHistory()
      },
    })
  }, [clearHistory, refetchHistory])

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Resonance Analysis</h1>
          <p className="text-muted-foreground">
            5D spectral analysis and resonance pattern detection
          </p>
        </div>
        <div className="flex gap-2">
          <Button onClick={toggleAnalysis} variant="default">
            {isRunning ? (
              <>
                <Pause className="mr-2 h-4 w-4" />
                Stop Analysis
              </>
            ) : (
              <>
                <Play className="mr-2 h-4 w-4" />
                Start Analysis
              </>
            )}
          </Button>
          <Button onClick={handleComputeSpectral} variant="outline" disabled={computeSpectral.isPending}>
            <Zap className="mr-2 h-4 w-4" />
            Compute Spectral
          </Button>
        </div>
      </div>

      {/* Progress & Status */}
      {isRunning && (
        <Card>
          <CardContent className="py-4">
            <div className="flex items-center gap-4">
              <Activity className="h-5 w-5 text-primary animate-pulse" />
              <div className="flex-1">
                <div className="flex justify-between text-sm mb-1">
                  <span>Analyzing spectral patterns...</span>
                  <span>{progress.toFixed(0)}%</span>
                </div>
                <div className="h-2 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-primary transition-all duration-300"
                    style={{ width: `${progress}%` }}
                  />
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Spectral Signature & Gauge */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Gauge */}
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-lg">Resonance Gauge</CardTitle>
          </CardHeader>
          <CardContent>
            <SpectralGauge
              psi={spectralSignature.psi}
              rho={spectralSignature.rho}
              omega={spectralSignature.omega}
              height={220}
            />
          </CardContent>
        </Card>

        {/* Spectral Values */}
        <Card>
          <CardHeader>
            <CardTitle>Spectral Signature (ψ, ρ, ω)</CardTitle>
            <CardDescription>Current resonance triplet values</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-5">
              {/* Psi */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ψ (Coherence)</span>
                  <span className="text-cyan-400 font-mono">
                    {formatResonance(spectralSignature.psi)}
                  </span>
                </div>
                <div className="h-3 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-cyan-400 transition-all duration-500"
                    style={{ width: `${spectralSignature.psi * 100}%` }}
                  />
                </div>
              </div>

              {/* Rho */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ρ (Stability)</span>
                  <span className="text-blue-400 font-mono">
                    {formatResonance(spectralSignature.rho)}
                  </span>
                </div>
                <div className="h-3 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-blue-400 transition-all duration-500"
                    style={{ width: `${spectralSignature.rho * 100}%` }}
                  />
                </div>
              </div>

              {/* Omega */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ω (Efficiency)</span>
                  <span className="text-amber-400 font-mono">
                    {formatResonance(spectralSignature.omega)}
                  </span>
                </div>
                <div className="h-3 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-amber-400 transition-all duration-500"
                    style={{ width: `${spectralSignature.omega * 100}%` }}
                  />
                </div>
              </div>

              <Separator />

              {/* Combined */}
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">ψ × ρ × ω</span>
                <span className={cn('text-2xl font-bold font-mono', getResonanceColor(resonance))}>
                  {formatResonance(resonance)}
                </span>
              </div>

              {/* Evaluate Button */}
              <Button
                className="w-full mt-2"
                onClick={handleEvaluateResonance}
                disabled={analyzeResonance.isPending}
              >
                <Zap className="mr-2 h-4 w-4" />
                Evaluate Resonance
              </Button>
            </div>
          </CardContent>
        </Card>

        {/* Operator Configuration */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Settings2 className="h-5 w-5" />
              Operators
            </CardTitle>
            <CardDescription>Topological operators (affect spectral dynamics)</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-3 grid-cols-2">
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">WT (Wave)</label>
                <Input
                  type="number"
                  value={pendingOperators.wt}
                  onChange={(e) => updatePendingOperator('wt', e.target.value)}
                  step="0.1"
                  min="0"
                  max="2"
                  className="h-8"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">SW (Spectral)</label>
                <Input
                  type="number"
                  value={pendingOperators.sw}
                  onChange={(e) => updatePendingOperator('sw', e.target.value)}
                  step="0.1"
                  min="0"
                  max="2"
                  className="h-8"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">DK (Double-Kick)</label>
                <Input
                  type="number"
                  value={pendingOperators.dk}
                  onChange={(e) => updatePendingOperator('dk', e.target.value)}
                  step="0.1"
                  min="0"
                  max="2"
                  className="h-8"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-muted-foreground">PI (Phase)</label>
                <Input
                  type="number"
                  value={pendingOperators.pi}
                  onChange={(e) => updatePendingOperator('pi', e.target.value)}
                  step="0.1"
                  min="0"
                  max="2"
                  className="h-8"
                />
              </div>
            </div>

            <div className="flex gap-2 pt-2">
              <Button variant="outline" size="sm" className="flex-1" onClick={handleResetOperators}>
                <RotateCcw className="mr-2 h-3 w-3" />
                Reset
              </Button>
              <Button size="sm" className="flex-1" onClick={handleApplyOperators}>
                Apply
              </Button>
            </div>

            <div className="text-xs text-muted-foreground pt-2 border-t">
              <p className="font-medium mb-1">
                Active: WT={operators.wt} SW={operators.sw} DK={operators.dk} PI={operators.pi}
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Last Evaluation Result */}
      {lastEvaluation && (
        <Card>
          <CardHeader>
            <CardTitle>Last Evaluation Result</CardTitle>
            <CardDescription>Holistic matrix evaluation output</CardDescription>
          </CardHeader>
          <CardContent>
            {lastEvaluation.type === 'Output' ? (
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Status</span>
                  <span className="text-green-400">Output Produced</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Resonance Score</span>
                  <span className="text-lg font-mono">{formatResonance(lastEvaluation.score)}</span>
                </div>
                <div className="mt-2">
                  <span className="text-sm font-medium">Action Vector (5D):</span>
                  <div className="font-mono text-sm mt-1 p-2 bg-muted rounded">
                    [{lastEvaluation.vector.map((v) => v.toFixed(4)).join(', ')}]
                  </div>
                </div>
              </div>
            ) : (
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Status</span>
                  <span className="text-red-400">Gated</span>
                </div>
                <div className="mt-2">
                  <span className="text-sm font-medium">Reason:</span>
                  <div className="text-sm mt-1 p-2 bg-muted rounded">{lastEvaluation.reason}</div>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Resonance History Chart */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Resonance History</CardTitle>
              <CardDescription>Time-series visualization of spectral components</CardDescription>
            </div>
            <Button variant="outline" size="sm" onClick={handleClearHistory}>
              Clear History
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <ResonanceTimeSeries data={history} height={300} />
        </CardContent>
      </Card>

      {/* Gateway History (from backend) */}
      {resonanceHistory && resonanceHistory.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Gateway History ({resonanceHistory.length} records)</CardTitle>
            <CardDescription>Resonance evaluations stored in gateway</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {resonanceHistory.slice(-10).reverse().map((record, idx) => (
                <div key={idx} className="flex items-center justify-between p-2 bg-muted rounded text-sm">
                  <span className="text-muted-foreground">
                    {new Date(record.timestamp).toLocaleTimeString()}
                  </span>
                  <span>
                    {record.response.type === 'Output' ? (
                      <span className="text-green-400">
                        Score: {formatResonance(record.response.score)}
                      </span>
                    ) : (
                      <span className="text-red-400">Gated</span>
                    )}
                  </span>
                  {record.request.label && (
                    <span className="text-xs text-muted-foreground">{record.request.label}</span>
                  )}
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
