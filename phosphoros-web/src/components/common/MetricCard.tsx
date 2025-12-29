import { LucideIcon } from 'lucide-react'
import { Card } from '@/components/ui/card'
import { cn, formatNumber, getResonanceColor } from '@/lib/utils'

interface MetricCardProps {
  title: string
  value: number | string
  icon: LucideIcon
  trend?: {
    value: number
    isPositive: boolean
  }
  variant?: 'default' | 'resonance'
  className?: string
}

export function MetricCard({
  title,
  value,
  icon: Icon,
  trend,
  variant = 'default',
  className,
}: MetricCardProps) {
  const displayValue = typeof value === 'number' ? formatNumber(value) : value
  const resonanceValue = typeof value === 'number' && variant === 'resonance' ? value : null

  return (
    <Card className={cn('metric-glow p-6', className)}>
      <div className="flex items-start justify-between">
        <div className="space-y-2">
          <p className="text-sm font-medium text-muted-foreground">{title}</p>
          <p
            className={cn(
              'text-3xl font-bold',
              resonanceValue !== null && getResonanceColor(resonanceValue)
            )}
          >
            {displayValue}
          </p>
          {trend && (
            <p
              className={cn(
                'text-xs',
                trend.isPositive ? 'text-success' : 'text-destructive'
              )}
            >
              {trend.isPositive ? '↑' : '↓'} {Math.abs(trend.value)}%
            </p>
          )}
        </div>
        <div className="rounded-lg bg-primary/10 p-3">
          <Icon className="h-6 w-6 text-primary" />
        </div>
      </div>
    </Card>
  )
}
