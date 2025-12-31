import { useState, useCallback, useMemo } from 'react'
import { useNavigate } from 'react-router-dom'
import { AlertTriangle, AlertCircle, Info, CheckCircle, Search, XCircle, RefreshCw } from 'lucide-react'
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
  investigatedAt?: Date
  resolution?: string
}

const initialAnomalies: Anomaly[] = [
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
    investigatedAt: new Date(Date.now() - 1000 * 60 * 60 * 4),
    resolution: 'False positive - legitimate exchange activity',
  },
  {
    id: '4',
    type: 'temporal',
    severity: 'low',
    address: '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa',
    description: 'Activity outside normal operating hours',
    detectedAt: new Date(Date.now() - 1000 * 60 * 60 * 24),
    resolved: true,
    investigatedAt: new Date(Date.now() - 1000 * 60 * 60 * 20),
    resolution: 'Normal activity from different timezone',
  },
  {
    id: '5',
    type: 'structural',
    severity: 'high',
    address: '0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984',
    description: 'Unusual contract interaction pattern suggesting potential exploit',
    detectedAt: new Date(Date.now() - 1000 * 60 * 45),
    resolved: false,
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
  const navigate = useNavigate()
  const [anomalies, setAnomalies] = useState<Anomaly[]>(initialAnomalies)
  const [selectedAnomaly, setSelectedAnomaly] = useState<Anomaly | null>(null)

  const activeAnomalies = useMemo(() => anomalies.filter((a) => !a.resolved), [anomalies])
  const resolvedAnomalies = useMemo(() => anomalies.filter((a) => a.resolved), [anomalies])

  // Investigate an anomaly - navigate to explorer with the address
  const handleInvestigate = useCallback((anomaly: Anomaly) => {
    navigate(`/explorer?address=${encodeURIComponent(anomaly.address)}`)
  }, [navigate])

  // Dismiss an anomaly - mark as resolved
  const handleDismiss = useCallback((anomalyId: string) => {
    setAnomalies(prev => prev.map(a =>
      a.id === anomalyId
        ? {
            ...a,
            resolved: true,
            investigatedAt: new Date(),
            resolution: 'Dismissed by analyst',
          }
        : a
    ))
  }, [])

  // Reopen an anomaly
  const handleReopen = useCallback((anomalyId: string) => {
    setAnomalies(prev => prev.map(a =>
      a.id === anomalyId
        ? {
            ...a,
            resolved: false,
            investigatedAt: undefined,
            resolution: undefined,
          }
        : a
    ))
  }, [])

  // Refresh anomalies (simulate new detection)
  const handleRefresh = useCallback(() => {
    const newAnomaly: Anomaly = {
      id: crypto.randomUUID(),
      type: ['sybil', 'laundering', 'volume', 'temporal', 'structural'][Math.floor(Math.random() * 5)] as Anomaly['type'],
      severity: ['low', 'medium', 'high', 'critical'][Math.floor(Math.random() * 4)] as Anomaly['severity'],
      address: `0x${Math.random().toString(16).slice(2, 42)}`,
      description: 'New suspicious activity detected during routine scan',
      detectedAt: new Date(),
      resolved: false,
    }
    setAnomalies(prev => [newAnomaly, ...prev])
  }, [])

  const severityCounts = useMemo(() => {
    return {
      critical: activeAnomalies.filter(a => a.severity === 'critical').length,
      high: activeAnomalies.filter(a => a.severity === 'high').length,
      medium: activeAnomalies.filter(a => a.severity === 'medium').length,
      low: activeAnomalies.filter(a => a.severity === 'low').length,
    }
  }, [activeAnomalies])

  const AnomalyCard = ({ anomaly }: { anomaly: Anomaly }) => {
    const config = severityConfig[anomaly.severity]
    const Icon = config.icon

    return (
      <div
        className={cn(
          'rounded-lg border p-4 space-y-3 transition-all',
          anomaly.resolved && 'opacity-60',
          selectedAnomaly?.id === anomaly.id && 'border-primary ring-1 ring-primary'
        )}
        onClick={() => setSelectedAnomaly(anomaly)}
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
          {!anomaly.resolved ? (
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={(e) => {
                  e.stopPropagation()
                  handleInvestigate(anomaly)
                }}
              >
                <Search className="mr-1 h-3 w-3" />
                Investigate
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={(e) => {
                  e.stopPropagation()
                  handleDismiss(anomaly.id)
                }}
              >
                <XCircle className="mr-1 h-3 w-3" />
                Dismiss
              </Button>
            </div>
          ) : (
            <Button
              variant="ghost"
              size="sm"
              onClick={(e) => {
                e.stopPropagation()
                handleReopen(anomaly.id)
              }}
            >
              <RefreshCw className="mr-1 h-3 w-3" />
              Reopen
            </Button>
          )}
        </div>
        {anomaly.resolved && anomaly.resolution && (
          <div className="pt-2 border-t">
            <p className="text-xs text-muted-foreground">
              <span className="font-medium">Resolution:</span> {anomaly.resolution}
            </p>
            {anomaly.investigatedAt && (
              <p className="text-xs text-muted-foreground">
                Resolved {formatRelativeTime(anomaly.investigatedAt)}
              </p>
            )}
          </div>
        )}
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
          <Button variant="outline" onClick={handleRefresh}>
            <RefreshCw className="mr-2 h-4 w-4" />
            Refresh
          </Button>
          <div className="text-right">
            <p className="text-2xl font-bold text-destructive">{activeAnomalies.length}</p>
            <p className="text-xs text-muted-foreground">Active Alerts</p>
          </div>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid gap-4 sm:grid-cols-4">
        {(['critical', 'high', 'medium', 'low'] as const).map((severity) => {
          const count = severityCounts[severity]
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
          {activeAnomalies.length === 0 ? (
            <div className="py-8 text-center text-muted-foreground">
              <CheckCircle className="h-12 w-12 mx-auto mb-2 opacity-50" />
              <p>No active anomalies detected</p>
            </div>
          ) : (
            <ScrollArea className="h-[300px]">
              <div className="space-y-3">
                {activeAnomalies.map((anomaly) => (
                  <AnomalyCard key={anomaly.id} anomaly={anomaly} />
                ))}
              </div>
            </ScrollArea>
          )}
        </CardContent>
      </Card>

      {/* Resolved Anomalies */}
      <Card>
        <CardHeader>
          <CardTitle>Resolved ({resolvedAnomalies.length})</CardTitle>
          <CardDescription>Previously investigated anomalies</CardDescription>
        </CardHeader>
        <CardContent>
          {resolvedAnomalies.length === 0 ? (
            <div className="py-8 text-center text-muted-foreground">
              <Info className="h-12 w-12 mx-auto mb-2 opacity-50" />
              <p>No resolved anomalies yet</p>
            </div>
          ) : (
            <ScrollArea className="h-[200px]">
              <div className="space-y-3">
                {resolvedAnomalies.map((anomaly) => (
                  <AnomalyCard key={anomaly.id} anomaly={anomaly} />
                ))}
              </div>
            </ScrollArea>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
