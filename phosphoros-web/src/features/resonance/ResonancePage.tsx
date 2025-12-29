import { useState } from 'react'
import { Activity, Play, Pause, Settings2 } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import { cn, formatResonance, getResonanceColor } from '@/lib/utils'

export function ResonancePage() {
  const [isRunning, setIsRunning] = useState(false)
  const [progress, setProgress] = useState(0)
  const [bestResonance, setBestResonance] = useState(0.73)

  const spectralSignature = {
    psi: 0.85,
    rho: 0.92,
    omega: 0.78,
  }

  const toggleAnalysis = () => {
    setIsRunning(!isRunning)
    if (!isRunning) {
      // Simulate progress
      let p = 0
      const interval = setInterval(() => {
        p += Math.random() * 5
        if (p >= 100) {
          p = 100
          clearInterval(interval)
          setIsRunning(false)
        }
        setProgress(p)
      }, 200)
    }
  }

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

      {/* Spectral Signature */}
      <div className="grid gap-6 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Spectral Signature (ψ, ρ, ω)</CardTitle>
            <CardDescription>
              Current resonance triplet values
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-6">
              {/* Psi */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ψ (Coherence)</span>
                  <span className="text-cyan-400">{formatResonance(spectralSignature.psi)}</span>
                </div>
                <div className="h-3 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-cyan-400 transition-all"
                    style={{ width: `${spectralSignature.psi * 100}%` }}
                  />
                </div>
              </div>

              {/* Rho */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ρ (Stability)</span>
                  <span className="text-blue-400">{formatResonance(spectralSignature.rho)}</span>
                </div>
                <div className="h-3 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-blue-400 transition-all"
                    style={{ width: `${spectralSignature.rho * 100}%` }}
                  />
                </div>
              </div>

              {/* Omega */}
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="font-medium">ω (Efficiency)</span>
                  <span className="text-amber-400">{formatResonance(spectralSignature.omega)}</span>
                </div>
                <div className="h-3 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-amber-400 transition-all"
                    style={{ width: `${spectralSignature.omega * 100}%` }}
                  />
                </div>
              </div>

              <Separator />

              {/* Combined Resonance */}
              <div className="text-center py-4">
                <p className="text-sm text-muted-foreground mb-2">Combined Resonance</p>
                <p className={cn('text-5xl font-bold', getResonanceColor(bestResonance))}>
                  {formatResonance(bestResonance)}
                </p>
                <p className="text-xs text-muted-foreground mt-2">
                  ψ × ρ × ω = {formatResonance(spectralSignature.psi * spectralSignature.rho * spectralSignature.omega)}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Operator Configuration */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Settings2 className="h-5 w-5" />
              Operator Configuration
            </CardTitle>
            <CardDescription>
              Topological operators for spectral analysis
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div className="space-y-2">
                <label className="text-sm font-medium">WT (Wave Transform)</label>
                <Input type="number" defaultValue="1.0" step="0.1" />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">SW (Spectral Width)</label>
                <Input type="number" defaultValue="0.5" step="0.1" />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">DK (Double-Kick)</label>
                <Input type="number" defaultValue="0.3" step="0.1" />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">PI (Phase Injection)</label>
                <Input type="number" defaultValue="0.1" step="0.1" />
              </div>
            </div>

            <Separator />

            <div className="flex gap-2">
              <Button variant="outline" className="flex-1">
                Reset Defaults
              </Button>
              <Button className="flex-1">Apply Configuration</Button>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Resonance History Placeholder */}
      <Card>
        <CardHeader>
          <CardTitle>Resonance History</CardTitle>
          <CardDescription>
            Time-series visualization of resonance patterns
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="h-64 rounded-lg border-2 border-dashed border-muted flex items-center justify-center">
            <p className="text-muted-foreground">ECharts visualization will render here</p>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
