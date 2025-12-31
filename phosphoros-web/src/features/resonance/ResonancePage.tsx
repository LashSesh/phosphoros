import { useState, useEffect, useCallback } from 'react'
import { Activity, Play, Pause, Settings2, RotateCcw } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import { SpectralGauge } from '@/components/charts/SpectralGauge'
import { ResonanceTimeSeries } from '@/components/charts/ResonanceTimeSeries'
import { cn, formatResonance, getResonanceColor } from '@/lib/utils'

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
      // WT (Wave Transform) affects psi amplitude
      // SW (Spectral Weight) affects rho stability
      // DK (Double-Kick) adds periodic spikes
      // PI (Phase Integral) affects omega smoothness
      const wtEffect = operators.wt * 0.05
      const swEffect = operators.sw * 0.03
      const dkSpike = Math.random() > (1 - operators.dk * 0.3) ? 0.1 : 0
      const piSmooth = operators.pi * 0.02

      setSpectralSignature(prev => ({
        psi: Math.min(1, Math.max(0, prev.psi + (Math.random() - 0.5) * wtEffect + dkSpike)),
        rho: Math.min(1, Math.max(0, prev.rho + (Math.random() - 0.5) * swEffect)),
        omega: Math.min(1, Math.max(0, prev.omega + (Math.random() - 0.5) * (0.04 - piSmooth))),
      }))

      // Add to history
      setHistory(prev => {
        const newPoint: TimeSeriesPoint = {
          timestamp: new Date(),
          psi: spectralSignature.psi,
          rho: spectralSignature.rho,
          omega: spectralSignature.omega,
          resonance: spectralSignature.psi * spectralSignature.rho * spectralSignature.omega,
        }
        return [...prev.slice(-30), newPoint]
      })

      // Update progress
      setProgress(prev => {
        const next = prev + Math.random() * 3
        if (next >= 100) {
          setIsRunning(false)
          return 100
        }
        return next
      })
    }, 500)

    return () => clearInterval(interval)
  }, [isRunning, spectralSignature, operators])

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
    setPendingOperators(prev => ({
      ...prev,
      [key]: Math.min(2, Math.max(0, numValue)),
    }))
  }, [])

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
        <Button onClick={toggleAnalysis}>
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
            <CardDescription>
              Current resonance triplet values
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-5">
              {/* Psi */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ψ (Coherence)</span>
                  <span className="text-cyan-400 font-mono">{formatResonance(spectralSignature.psi)}</span>
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
                  <span className="text-blue-400 font-mono">{formatResonance(spectralSignature.rho)}</span>
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
                  <span className="text-amber-400 font-mono">{formatResonance(spectralSignature.omega)}</span>
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
            <CardDescription>
              Topological operators (affect spectral dynamics)
            </CardDescription>
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

            {/* Show active operators */}
            <div className="text-xs text-muted-foreground pt-2 border-t">
              <p className="font-medium mb-1">Active: WT={operators.wt} SW={operators.sw} DK={operators.dk} PI={operators.pi}</p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Resonance History Chart */}
      <Card>
        <CardHeader>
          <CardTitle>Resonance History</CardTitle>
          <CardDescription>
            Time-series visualization of spectral components
          </CardDescription>
        </CardHeader>
        <CardContent>
          <ResonanceTimeSeries data={history} height={300} />
        </CardContent>
      </Card>
    </div>
  )
}
