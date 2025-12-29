import { Command, Bell, Moon, Sun } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { useAppStore } from '@/stores/app'
import { useServicesStore } from '@/stores/services'
import { cn } from '@/lib/utils'

export function Header() {
  const { theme, setTheme, setCommandPaletteOpen } = useAppStore()
  const { gateway, websocket } = useServicesStore()

  return (
    <header className="flex h-14 items-center justify-between border-b border-border bg-background/95 px-6 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      {/* Left: Command Palette Trigger */}
      <div className="flex items-center gap-4">
        <Button
          variant="outline"
          className="relative h-9 w-64 justify-start text-sm text-muted-foreground"
          onClick={() => setCommandPaletteOpen(true)}
        >
          <Command className="mr-2 h-4 w-4" />
          <span>Search...</span>
          <kbd className="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 select-none">
            <span className="kbd">⌘K</span>
          </kbd>
        </Button>
      </div>

      {/* Right: Status & Actions */}
      <div className="flex items-center gap-4">
        {/* Service Status */}
        <div className="flex items-center gap-2">
          <Tooltip>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-1.5">
                <div
                  className={cn(
                    'status-dot',
                    gateway.status === 'online' && 'online',
                    gateway.status === 'offline' && 'offline',
                    gateway.status === 'pending' && 'pending'
                  )}
                />
                <span className="text-xs text-muted-foreground">Gateway</span>
              </div>
            </TooltipTrigger>
            <TooltipContent>
              Gateway: {gateway.status}
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-1.5">
                <div
                  className={cn(
                    'status-dot',
                    websocket === 'connected' && 'online',
                    websocket === 'disconnected' && 'offline',
                    websocket === 'connecting' && 'pending'
                  )}
                />
                <span className="text-xs text-muted-foreground">WS</span>
              </div>
            </TooltipTrigger>
            <TooltipContent>
              WebSocket: {websocket}
            </TooltipContent>
          </Tooltip>
        </div>

        {/* Notifications */}
        <Button variant="ghost" size="icon" className="relative">
          <Bell className="h-5 w-5" />
          <Badge
            variant="destructive"
            className="absolute -right-1 -top-1 h-5 w-5 rounded-full p-0 text-xs"
          >
            3
          </Badge>
        </Button>

        {/* Theme Toggle */}
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
            >
              {theme === 'dark' ? (
                <Sun className="h-5 w-5" />
              ) : (
                <Moon className="h-5 w-5" />
              )}
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            {theme === 'dark' ? 'Light mode' : 'Dark mode'}
          </TooltipContent>
        </Tooltip>
      </div>
    </header>
  )
}
