import { useEffect, useState, useCallback } from 'react'
import { useNavigate } from 'react-router-dom'
import {
  Key,
  Zap,
  Database,
  AlertTriangle,
  Activity,
  Play,
  Download,
  Plus,
  Loader2,
} from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { MetricCard } from '@/components/common/MetricCard'
import { ActivityFeed } from '@/components/common/ActivityFeed'
import { useMetricsStore } from '@/stores/metrics'
import { useServicesStore } from '@/stores/services'
import { cn } from '@/lib/utils'

export function Dashboard() {
  const navigate = useNavigate()
  const metrics = useMetricsStore()
  const { scraper, analyzer, cluster } = useServicesStore()
  const [isAnalyzing, setIsAnalyzing] = useState(false)
  const [isExporting, setIsExporting] = useState(false)

  // Simulate some initial activity for demo
  useEffect(() => {
    const addActivity = metrics.addActivity
    const timeout = setTimeout(() => {
      addActivity({ type: 'entity', message: 'Discovered new wallet 0x7a25...3f21' })
    }, 1000)
    const timeout2 = setTimeout(() => {
      addActivity({ type: 'cluster', message: 'Formed cluster with 12 members' })
    }, 2000)
    const timeout3 = setTimeout(() => {
      addActivity({ type: 'anomaly', message: 'High-value transfer detected' })
    }, 3000)
    return () => {
      clearTimeout(timeout)
      clearTimeout(timeout2)
      clearTimeout(timeout3)
    }
  }, [])

  // Export report as JSON
  const handleExportReport = useCallback(async () => {
    setIsExporting(true)
    try {
      const report = {
        exportedAt: new Date().toISOString(),
        metrics: {
          seeds: metrics.seeds,
          clusters: metrics.clusters,
          entities: metrics.entities,
          anomalies: metrics.anomalies,
          analyses: metrics.analyses,
          hotspots: metrics.hotspots,
        },
        services: {
          scraper: scraper,
          analyzer: analyzer,
          cluster: cluster,
        },
        recentActivity: metrics.recentActivity.slice(0, 20),
      }
      const blob = new Blob([JSON.stringify(report, null, 2)], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `phosphoros-report-${new Date().toISOString().split('T')[0]}.json`
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
      metrics.addActivity({ type: 'analysis', message: 'Report exported successfully' })
    } finally {
      setIsExporting(false)
    }
  }, [metrics, scraper, analyzer, cluster])

  // Run analysis simulation
  const handleRunAnalysis = useCallback(async () => {
    setIsAnalyzing(true)
    metrics.addActivity({ type: 'analysis', message: 'Starting forensic analysis...' })

    // Simulate analysis progress
    await new Promise(resolve => setTimeout(resolve, 1500))
    metrics.setMetrics({ analyses: metrics.analyses + 1 })
    metrics.addActivity({ type: 'entity', message: 'Scanned 1,247 transactions' })

    await new Promise(resolve => setTimeout(resolve, 1000))
    const newClusters = Math.floor(Math.random() * 3) + 1
    metrics.setMetrics({ clusters: metrics.clusters + newClusters })
    metrics.addActivity({ type: 'cluster', message: `Identified ${newClusters} new clusters` })

    await new Promise(resolve => setTimeout(resolve, 800))
    const anomalyChance = Math.random()
    if (anomalyChance > 0.6) {
      metrics.setMetrics({ anomalies: metrics.anomalies + 1 })
      metrics.addActivity({ type: 'anomaly', message: 'Suspicious pattern detected in cluster' })
    }

    metrics.addActivity({ type: 'analysis', message: 'Analysis complete' })
    setIsAnalyzing(false)
  }, [metrics])

  // Quick action handlers
  const handleImportMnemonic = () => navigate('/wallet')
  const handleStartAnalysis = () => navigate('/resonance')
  const handleViewClusters = () => navigate('/topology')
  const handleExportData = () => handleExportReport()

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
          <p className="text-muted-foreground">
            Real-time blockchain forensics overview
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleExportReport} disabled={isExporting}>
            {isExporting ? (
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            ) : (
              <Download className="mr-2 h-4 w-4" />
            )}
            Export Report
          </Button>
          <Button size="sm" onClick={handleRunAnalysis} disabled={isAnalyzing}>
            {isAnalyzing ? (
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            ) : (
              <Play className="mr-2 h-4 w-4" />
            )}
            Run Analysis
          </Button>
        </div>
      </div>

      {/* Metric Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <MetricCard
          title="Seeds Collected"
          value={metrics.seeds}
          icon={Key}
          trend={{ value: 12, isPositive: true }}
        />
        <MetricCard
          title="Clusters Discovered"
          value={metrics.clusters}
          icon={Zap}
          trend={{ value: 8, isPositive: true }}
        />
        <MetricCard
          title="Entities Tracked"
          value={metrics.entities}
          icon={Database}
        />
        <MetricCard
          title="Anomalies Detected"
          value={metrics.anomalies}
          icon={AlertTriangle}
          trend={{ value: 3, isPositive: false }}
        />
      </div>

      {/* Main Content Grid */}
      <div className="grid gap-6 lg:grid-cols-3">
        {/* Services Status */}
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Services</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {[
              { name: 'Scraper', info: scraper, icon: Database },
              { name: 'Analyzer', info: analyzer, icon: Activity },
              { name: 'Cluster Engine', info: cluster, icon: Zap },
            ].map(({ name, info, icon: Icon }) => (
              <div
                key={name}
                className="flex items-center justify-between rounded-lg border p-3"
              >
                <div className="flex items-center gap-3">
                  <div className="rounded-md bg-muted p-2">
                    <Icon className="h-4 w-4" />
                  </div>
                  <div>
                    <p className="text-sm font-medium">{name}</p>
                    <p className="text-xs text-muted-foreground">
                      {info.processed > 0
                        ? `${info.processed} processed`
                        : 'Idle'}
                    </p>
                  </div>
                </div>
                <Badge
                  variant={
                    info.status === 'online'
                      ? 'success'
                      : info.status === 'pending'
                      ? 'warning'
                      : 'secondary'
                  }
                >
                  {info.status}
                </Badge>
              </div>
            ))}
          </CardContent>
        </Card>

        {/* Activity Feed */}
        <div className="lg:col-span-2">
          <ActivityFeed />
        </div>
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-lg">Quick Actions</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <Button
              variant="outline"
              className="h-auto flex-col gap-2 py-4"
              onClick={handleImportMnemonic}
            >
              <Plus className="h-6 w-6" />
              <span>Import Mnemonic</span>
            </Button>
            <Button
              variant="outline"
              className="h-auto flex-col gap-2 py-4"
              onClick={handleStartAnalysis}
            >
              <Activity className="h-6 w-6" />
              <span>Start Analysis</span>
            </Button>
            <Button
              variant="outline"
              className="h-auto flex-col gap-2 py-4"
              onClick={handleViewClusters}
            >
              <Zap className="h-6 w-6" />
              <span>View Clusters</span>
            </Button>
            <Button
              variant="outline"
              className="h-auto flex-col gap-2 py-4"
              onClick={handleExportData}
              disabled={isExporting}
            >
              {isExporting ? (
                <Loader2 className="h-6 w-6 animate-spin" />
              ) : (
                <Download className="h-6 w-6" />
              )}
              <span>Export Data</span>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
