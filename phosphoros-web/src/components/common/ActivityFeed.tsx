import { Database, Zap, AlertTriangle, Activity } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { useMetricsStore } from '@/stores/metrics'
import { formatRelativeTime } from '@/lib/utils'
import { cn } from '@/lib/utils'

const typeIcons = {
  entity: Database,
  cluster: Zap,
  anomaly: AlertTriangle,
  analysis: Activity,
}

const typeColors = {
  entity: 'bg-primary/10 text-primary',
  cluster: 'bg-accent/10 text-accent',
  anomaly: 'bg-warning/10 text-warning',
  analysis: 'bg-success/10 text-success',
}

export function ActivityFeed() {
  const { recentActivity } = useMetricsStore()

  return (
    <Card className="h-full">
      <CardHeader className="pb-3">
        <CardTitle className="text-lg">Recent Activity</CardTitle>
      </CardHeader>
      <CardContent className="p-0">
        <ScrollArea className="h-[300px] px-6">
          {recentActivity.length === 0 ? (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No recent activity
            </p>
          ) : (
            <div className="space-y-4 pb-4">
              {recentActivity.map((activity) => {
                const Icon = typeIcons[activity.type]
                return (
                  <div
                    key={activity.id}
                    className="flex items-start gap-3 animate-in"
                  >
                    <div
                      className={cn(
                        'mt-0.5 rounded-full p-1.5',
                        typeColors[activity.type]
                      )}
                    >
                      <Icon className="h-3.5 w-3.5" />
                    </div>
                    <div className="flex-1 space-y-1">
                      <p className="text-sm">{activity.message}</p>
                      <p className="text-xs text-muted-foreground">
                        {formatRelativeTime(activity.timestamp)}
                      </p>
                    </div>
                    <Badge variant="outline" className="text-xs">
                      {activity.type}
                    </Badge>
                  </div>
                )
              })}
            </div>
          )}
        </ScrollArea>
      </CardContent>
    </Card>
  )
}
