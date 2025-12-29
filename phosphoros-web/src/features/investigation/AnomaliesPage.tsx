import { AlertTriangle, AlertCircle, Info, CheckCircle } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { formatAddress, formatRelativeTime } from '@/lib/utils'
import { cn } from '@/lib/utils'

interface Anomaly {
  id: string
  type: 'sybil' | 'laundering' | 'volume' | 'temporal' | 'structural'
  severity: 'low' | 'medium' | 'high' | 'critical'
  address: string
  description: string
  detectedAt: Date
  resolved: boolean
}

const mockAnomalies: Anomaly[] = [
  {
    id: '1',
    type: 'laundering',
    severity: 'critical',
    address: '0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D',
    description: 'Suspicious circular transaction pattern detected across 12 addresses',
    detectedAt: new Date(Date.now() - 1000 * 60 * 30),
    resolved: false,
  },
  {
    id: '2',
    type: 'sybil',
    severity: 'high',
    address: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
    description: 'Multiple wallets with identical behavioral patterns',
    detectedAt: new Date(Date.now() - 1000 * 60 * 60 * 2),
    resolved: false,
  },
  {
    id: '3',
    type: 'volume',
    severity: 'medium',
    address: '0xdAC17F958D2ee523a2206206994597C13D831ec7',
    description: 'Unusual transaction volume spike (10x average)',
    detectedAt: new Date(Date.now() - 1000 * 60 * 60 * 5),
    resolved: true,
  },
  {
    id: '4',
    type: 'temporal',
    severity: 'low',
    address: '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa',
    description: 'Activity outside normal operating hours',
    detectedAt: new Date(Date.now() - 1000 * 60 * 60 * 24),
    resolved: true,
  },
]

const severityConfig = {
  critical: { color: 'bg-destructive text-destructive-foreground', icon: AlertTriangle },
  high: { color: 'bg-warning text-warning-foreground', icon: AlertCircle },
  medium: { color: 'bg-primary text-primary-foreground', icon: Info },
  low: { color: 'bg-muted text-muted-foreground', icon: CheckCircle },
}

const typeLabels = {
  sybil: 'Sybil Attack',
  laundering: 'Money Laundering',
  volume: 'Volume Anomaly',
  temporal: 'Temporal Pattern',
  structural: 'Structural Anomaly',
}

export function AnomaliesPage() {
  const activeAnomalies = mockAnomalies.filter((a) => !a.resolved)
  const resolvedAnomalies = mockAnomalies.filter((a) => a.resolved)

  const AnomalyCard = ({ anomaly }: { anomaly: Anomaly }) => {
    const config = severityConfig[anomaly.severity]
    const Icon = config.icon

    return (
      <div
        className={cn(
          'rounded-lg border p-4 space-y-3',
          anomaly.resolved && 'opacity-60'
        )}
      >
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-3">
            <div className={cn('rounded-md p-2', config.color)}>
              <Icon className="h-4 w-4" />
            </div>
            <div>
              <p className="font-medium">{typeLabels[anomaly.type]}</p>
              <code className="text-xs font-address text-muted-foreground">
                {formatAddress(anomaly.address, 8)}
              </code>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Badge className={config.color}>{anomaly.severity}</Badge>
            {anomaly.resolved && <Badge variant="outline">Resolved</Badge>}
          </div>
        </div>
        <p className="text-sm text-muted-foreground">{anomaly.description}</p>
        <div className="flex items-center justify-between">
          <p className="text-xs text-muted-foreground">
            Detected {formatRelativeTime(anomaly.detectedAt)}
          </p>
          {!anomaly.resolved && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm">
                Investigate
              </Button>
              <Button variant="ghost" size="sm">
                Dismiss
              </Button>
            </div>
          )}
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Anomalies</h1>
          <p className="text-muted-foreground">
            Detected suspicious patterns and security alerts
          </p>
        </div>
        <div className="flex items-center gap-4">
          <div className="text-right">
            <p className="text-2xl font-bold text-destructive">{activeAnomalies.length}</p>
            <p className="text-xs text-muted-foreground">Active Alerts</p>
          </div>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-4 sm:grid-cols-4">
        {(['critical', 'high', 'medium', 'low'] as const).map((severity) => {
          const count = mockAnomalies.filter((a) => a.severity === severity && !a.resolved).length
          const config = severityConfig[severity]
          const Icon = config.icon
          return (
            <Card key={severity}>
              <CardContent className="pt-6">
                <div className="flex items-center gap-3">
                  <div className={cn('rounded-md p-2', config.color)}>
                    <Icon className="h-4 w-4" />
                  </div>
                  <div>
                    <p className="text-2xl font-bold">{count}</p>
                    <p className="text-sm text-muted-foreground capitalize">{severity}</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          )
        })}
      </div>

      {/* Active Anomalies */}
      <Card>
        <CardHeader>
          <CardTitle>Active Alerts ({activeAnomalies.length})</CardTitle>
          <CardDescription>Anomalies requiring investigation</CardDescription>
        </CardHeader>
        <CardContent>
          <ScrollArea className="h-[300px]">
            <div className="space-y-3">
              {activeAnomalies.map((anomaly) => (
                <AnomalyCard key={anomaly.id} anomaly={anomaly} />
              ))}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>

      {/* Resolved Anomalies */}
      <Card>
        <CardHeader>
          <CardTitle>Resolved ({resolvedAnomalies.length})</CardTitle>
          <CardDescription>Previously investigated anomalies</CardDescription>
        </CardHeader>
        <CardContent>
          <ScrollArea className="h-[200px]">
            <div className="space-y-3">
              {resolvedAnomalies.map((anomaly) => (
                <AnomalyCard key={anomaly.id} anomaly={anomaly} />
              ))}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>
    </div>
  )
}
