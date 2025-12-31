import { useState, useCallback } from 'react'
import { Moon, Sun, Monitor, Bell, Shield, Database, Download, Trash2, Loader2, Check } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { Switch } from '@/components/ui/switch'
import { useAppStore } from '@/stores/app'
import { useMetricsStore } from '@/stores/metrics'
import { useInfogenetikStore } from '@/stores/infogenetik'
import { cn } from '@/lib/utils'

interface SettingsState {
  anomalyAlerts: boolean
  analysisComplete: boolean
  serviceStatus: boolean
  autoMaskMnemonics: boolean
  requestLogging: boolean
  sessionTimeout: number
  exportFormat: 'json' | 'csv' | 'xml'
}

export function SettingsPage() {
  const { theme, setTheme } = useAppStore()
  const metrics = useMetricsStore()
  const infogenetik = useInfogenetikStore()

  const [settings, setSettings] = useState<SettingsState>({
    anomalyAlerts: true,
    analysisComplete: true,
    serviceStatus: false,
    autoMaskMnemonics: true,
    requestLogging: true,
    sessionTimeout: 30,
    exportFormat: 'json',
  })

  const [isExporting, setIsExporting] = useState(false)
  const [isClearing, setIsClearing] = useState(false)
  const [exportSuccess, setExportSuccess] = useState(false)
  const [clearSuccess, setClearSuccess] = useState(false)

  const themes = [
    { id: 'light', name: 'Light', icon: Sun },
    { id: 'dark', name: 'Dark', icon: Moon },
    { id: 'system', name: 'System', icon: Monitor },
  ] as const

  const updateSetting = useCallback(<K extends keyof SettingsState>(
    key: K,
    value: SettingsState[K]
  ) => {
    setSettings(prev => ({ ...prev, [key]: value }))
  }, [])

  const handleExportAllData = useCallback(async () => {
    setIsExporting(true)
    setExportSuccess(false)

    try {
      // Gather all data from stores
      const exportData = {
        exportedAt: new Date().toISOString(),
        version: '3.0.0',
        settings: settings,
        metrics: {
          seeds: metrics.seeds,
          clusters: metrics.clusters,
          entities: metrics.entities,
          anomalies: metrics.anomalies,
          analyses: metrics.analyses,
          hotspots: metrics.hotspots,
          recentActivity: metrics.recentActivity,
        },
        infogenetik: {
          infogenoms: infogenetik.infogenoms,
          operators: infogenetik.operators,
          explorationResults: infogenetik.explorationResults,
        },
      }

      // Simulate export delay
      await new Promise(resolve => setTimeout(resolve, 1000))

      // Create and download file
      const blob = new Blob([JSON.stringify(exportData, null, 2)], {
        type: 'application/json',
      })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `phosphoros-full-export-${new Date().toISOString().split('T')[0]}.json`
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)

      setExportSuccess(true)
      setTimeout(() => setExportSuccess(false), 3000)
    } finally {
      setIsExporting(false)
    }
  }, [settings, metrics, infogenetik])

  const handleClearCache = useCallback(async () => {
    setIsClearing(true)
    setClearSuccess(false)

    try {
      // Simulate clearing delay
      await new Promise(resolve => setTimeout(resolve, 1000))

      // Clear localStorage items related to our app
      const keysToRemove = [
        'phosphoros-app-settings',
        'phosphoros-infogenetik',
      ]
      keysToRemove.forEach(key => localStorage.removeItem(key))

      // Reset metrics
      metrics.setMetrics({
        seeds: 0,
        clusters: 0,
        entities: 0,
        anomalies: 0,
        analyses: 0,
        hotspots: 0,
      })

      // Clear Infogenetik data
      infogenetik.clearResults()

      setClearSuccess(true)
      setTimeout(() => setClearSuccess(false), 3000)
    } finally {
      setIsClearing(false)
    }
  }, [metrics, infogenetik])

  const ToggleRow = ({
    title,
    description,
    enabled,
    onChange,
  }: {
    title: string
    description: string
    enabled: boolean
    onChange: (value: boolean) => void
  }) => (
    <div className="flex items-center justify-between">
      <div>
        <p className="font-medium">{title}</p>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>
      <Switch checked={enabled} onCheckedChange={onChange} />
    </div>
  )

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-muted-foreground">
          Configure your dashboard preferences
        </p>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        {/* Appearance */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Moon className="h-5 w-5" />
              Appearance
            </CardTitle>
            <CardDescription>
              Customize the look and feel of the dashboard
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label className="text-sm font-medium">Theme</label>
              <div className="flex gap-2 mt-2">
                {themes.map(({ id, name, icon: Icon }) => (
                  <Button
                    key={id}
                    variant={theme === id ? 'default' : 'outline'}
                    className="flex-1"
                    onClick={() => setTheme(id)}
                  >
                    <Icon className="mr-2 h-4 w-4" />
                    {name}
                  </Button>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Notifications */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Bell className="h-5 w-5" />
              Notifications
            </CardTitle>
            <CardDescription>
              Configure alert and notification preferences
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <ToggleRow
              title="Anomaly Alerts"
              description="Get notified when anomalies are detected"
              enabled={settings.anomalyAlerts}
              onChange={(value) => updateSetting('anomalyAlerts', value)}
            />
            <Separator />
            <ToggleRow
              title="Analysis Complete"
              description="Notify when analysis jobs finish"
              enabled={settings.analysisComplete}
              onChange={(value) => updateSetting('analysisComplete', value)}
            />
            <Separator />
            <ToggleRow
              title="Service Status"
              description="Alert on service health changes"
              enabled={settings.serviceStatus}
              onChange={(value) => updateSetting('serviceStatus', value)}
            />
          </CardContent>
        </Card>

        {/* Security */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="h-5 w-5" />
              Security
            </CardTitle>
            <CardDescription>
              Security and privacy settings
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <ToggleRow
              title="Auto-mask Mnemonics"
              description="Hide sensitive seed phrases by default"
              enabled={settings.autoMaskMnemonics}
              onChange={(value) => updateSetting('autoMaskMnemonics', value)}
            />
            <Separator />
            <ToggleRow
              title="Request Logging"
              description="Log all API requests for audit"
              enabled={settings.requestLogging}
              onChange={(value) => updateSetting('requestLogging', value)}
            />
            <Separator />
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Session Timeout</p>
                <p className="text-sm text-muted-foreground">
                  Auto-logout after inactivity
                </p>
              </div>
              <div className="flex gap-2">
                {[15, 30, 60].map((minutes) => (
                  <Button
                    key={minutes}
                    variant={settings.sessionTimeout === minutes ? 'default' : 'outline'}
                    size="sm"
                    onClick={() => updateSetting('sessionTimeout', minutes)}
                  >
                    {minutes}m
                  </Button>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Data */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Database className="h-5 w-5" />
              Data Management
            </CardTitle>
            <CardDescription>
              Manage your forensic data and exports
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Export Format</p>
                <p className="text-sm text-muted-foreground">
                  Default format for data exports
                </p>
              </div>
              <div className="flex gap-2">
                {(['json', 'csv', 'xml'] as const).map((format) => (
                  <Badge
                    key={format}
                    variant={settings.exportFormat === format ? 'default' : 'outline'}
                    className="cursor-pointer uppercase"
                    onClick={() => updateSetting('exportFormat', format)}
                  >
                    {format}
                  </Badge>
                ))}
              </div>
            </div>
            <Separator />
            <div className="space-y-2">
              <Button
                variant="outline"
                className="w-full"
                onClick={handleExportAllData}
                disabled={isExporting}
              >
                {isExporting ? (
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                ) : exportSuccess ? (
                  <Check className="mr-2 h-4 w-4 text-green-500" />
                ) : (
                  <Download className="mr-2 h-4 w-4" />
                )}
                {exportSuccess ? 'Exported Successfully!' : 'Export All Data'}
              </Button>
              <Button
                variant="outline"
                className={cn("w-full", !clearSuccess && "text-destructive hover:text-destructive")}
                onClick={handleClearCache}
                disabled={isClearing}
              >
                {isClearing ? (
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                ) : clearSuccess ? (
                  <Check className="mr-2 h-4 w-4 text-green-500" />
                ) : (
                  <Trash2 className="mr-2 h-4 w-4" />
                )}
                {clearSuccess ? 'Cache Cleared!' : 'Clear Local Cache'}
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* About */}
      <Card>
        <CardHeader>
          <CardTitle>About PHOSPHOROS</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium">PHOSPHOROS Dashboard</p>
              <p className="text-sm text-muted-foreground">
                Enterprise Blockchain Forensics Platform with 5D Infogenetik
              </p>
            </div>
            <div className="text-right">
              <Badge variant="outline">v3.0.0</Badge>
              <p className="text-xs text-muted-foreground mt-1">Web Edition</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
