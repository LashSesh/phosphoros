import { Settings, Moon, Sun, Monitor, Bell, Shield, Database } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { useAppStore } from '@/stores/app'
import { cn } from '@/lib/utils'

export function SettingsPage() {
  const { theme, setTheme } = useAppStore()

  const themes = [
    { id: 'light', name: 'Light', icon: Sun },
    { id: 'dark', name: 'Dark', icon: Moon },
    { id: 'system', name: 'System', icon: Monitor },
  ] as const

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
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Anomaly Alerts</p>
                <p className="text-sm text-muted-foreground">
                  Get notified when anomalies are detected
                </p>
              </div>
              <Badge variant="success">Enabled</Badge>
            </div>
            <Separator />
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Analysis Complete</p>
                <p className="text-sm text-muted-foreground">
                  Notify when analysis jobs finish
                </p>
              </div>
              <Badge variant="success">Enabled</Badge>
            </div>
            <Separator />
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Service Status</p>
                <p className="text-sm text-muted-foreground">
                  Alert on service health changes
                </p>
              </div>
              <Badge variant="outline">Disabled</Badge>
            </div>
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
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Auto-mask Mnemonics</p>
                <p className="text-sm text-muted-foreground">
                  Hide sensitive seed phrases by default
                </p>
              </div>
              <Badge variant="success">Enabled</Badge>
            </div>
            <Separator />
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Request Logging</p>
                <p className="text-sm text-muted-foreground">
                  Log all API requests for audit
                </p>
              </div>
              <Badge variant="success">Enabled</Badge>
            </div>
            <Separator />
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Session Timeout</p>
                <p className="text-sm text-muted-foreground">
                  Auto-logout after inactivity
                </p>
              </div>
              <Badge>30 minutes</Badge>
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
              <Badge>JSON</Badge>
            </div>
            <Separator />
            <div className="space-y-2">
              <Button variant="outline" className="w-full">
                Export All Data
              </Button>
              <Button variant="outline" className="w-full text-destructive">
                Clear Local Cache
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
                Enterprise Blockchain Forensics Platform
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
