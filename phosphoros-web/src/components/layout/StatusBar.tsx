import { Activity, Database, Zap, AlertTriangle } from 'lucide-react'
import { useServicesStore } from '@/stores/services'
import { useMetricsStore } from '@/stores/metrics'
import { cn } from '@/lib/utils'
import { formatNumber } from '@/lib/utils'

export function StatusBar() {
  const { scraper, analyzer, cluster, websocket } = useServicesStore()
  const { entities, clusters, anomalies } = useMetricsStore()

  const ServiceIndicator = ({
    name,
    status,
    processed,
  }: {
    name: string
    status: 'online' | 'offline' | 'pending'
    processed: number
  }) => (
    <div className="flex items-center gap-1.5 text-xs">
      <div
        className={cn(
          'h-1.5 w-1.5 rounded-full',
          status === 'online' && 'bg-success',
          status === 'offline' && 'bg-muted-foreground',
          status === 'pending' && 'bg-warning animate-pulse'
        )}
      />
      <span className="text-muted-foreground">{name}</span>
      {status === 'online' && processed > 0 && (
        <span className="text-foreground">{formatNumber(processed)}</span>
      )}
    </div>
  )

  return (
    <footer className="flex h-8 items-center justify-between border-t border-border bg-background/95 px-4 text-xs backdrop-blur">
      {/* Left: Services */}
      <div className="flex items-center gap-4">
        <span className="text-muted-foreground">Services:</span>
        <ServiceIndicator
          name="Scraper"
          status={scraper.status}
          processed={scraper.processed}
        />
        <ServiceIndicator
          name="Analyzer"
          status={analyzer.status}
          processed={analyzer.processed}
        />
        <ServiceIndicator
          name="Cluster"
          status={cluster.status}
          processed={cluster.processed}
        />
      </div>

      {/* Center: Quick Stats */}
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-1.5">
          <Database className="h-3.5 w-3.5 text-muted-foreground" />
          <span className="text-muted-foreground">Entities:</span>
          <span className="font-medium">{formatNumber(entities)}</span>
        </div>
        <div className="flex items-center gap-1.5">
          <Zap className="h-3.5 w-3.5 text-muted-foreground" />
          <span className="text-muted-foreground">Clusters:</span>
          <span className="font-medium">{formatNumber(clusters)}</span>
        </div>
        <div className="flex items-center gap-1.5">
          <AlertTriangle className="h-3.5 w-3.5 text-muted-foreground" />
          <span className="text-muted-foreground">Anomalies:</span>
          <span className="font-medium text-warning">{formatNumber(anomalies)}</span>
        </div>
      </div>

      {/* Right: Connection Status */}
      <div className="flex items-center gap-2">
        <Activity
          className={cn(
            'h-3.5 w-3.5',
            websocket === 'connected' && 'text-success animate-pulse',
            websocket === 'disconnected' && 'text-muted-foreground',
            websocket === 'connecting' && 'text-warning animate-spin'
          )}
        />
        <span className="text-muted-foreground">
          {websocket === 'connected' && 'Live'}
          {websocket === 'disconnected' && 'Offline'}
          {websocket === 'connecting' && 'Connecting...'}
        </span>
      </div>
    </footer>
  )
}
