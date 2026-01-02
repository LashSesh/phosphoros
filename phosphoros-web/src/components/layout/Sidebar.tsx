import { NavLink, useLocation } from 'react-router-dom'
import {
  LayoutDashboard,
  Key,
  Activity,
  Network,
  Search,
  AlertTriangle,
  FileSearch,
  Settings,
  ChevronLeft,
  ChevronRight,
  Dna,
  Users,
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { useAppStore } from '@/stores/app'

const navigation = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard },
  { name: 'Wallet & Seeds', href: '/wallet', icon: Key },
  { name: 'Infogenetik', href: '/infogenetik', icon: Dna },
  { name: 'Resonance', href: '/resonance', icon: Activity },
  { name: 'Cluster', href: '/cluster', icon: Users },
  { name: 'Topology', href: '/topology', icon: Network },
  { name: 'Explorer', href: '/explorer', icon: Search },
  { name: 'Anomalies', href: '/anomalies', icon: AlertTriangle },
  { name: 'Forensics', href: '/forensics', icon: FileSearch },
]

const bottomNavigation = [
  { name: 'Settings', href: '/settings', icon: Settings },
]

export function Sidebar() {
  const location = useLocation()
  const { sidebarCollapsed, toggleSidebar } = useAppStore()

  const NavItem = ({ item }: { item: typeof navigation[0] }) => {
    const isActive = location.pathname === item.href
    const Icon = item.icon

    const content = (
      <NavLink
        to={item.href}
        className={cn(
          'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
          isActive
            ? 'bg-primary/10 text-primary'
            : 'text-muted-foreground hover:bg-muted hover:text-foreground'
        )}
      >
        <Icon className="h-5 w-5 shrink-0" />
        {!sidebarCollapsed && <span>{item.name}</span>}
      </NavLink>
    )

    if (sidebarCollapsed) {
      return (
        <Tooltip>
          <TooltipTrigger asChild>{content}</TooltipTrigger>
          <TooltipContent side="right">{item.name}</TooltipContent>
        </Tooltip>
      )
    }

    return content
  }

  return (
    <aside
      className={cn(
        'flex h-screen flex-col border-r border-sidebar-border bg-sidebar transition-all duration-300',
        sidebarCollapsed ? 'w-16' : 'w-64'
      )}
    >
      {/* Logo */}
      <div className="flex h-14 items-center border-b border-sidebar-border px-4">
        <div className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-accent">
            <Activity className="h-5 w-5 text-primary-foreground" />
          </div>
          {!sidebarCollapsed && (
            <span className="text-lg font-semibold text-gradient">
              PHOSPHOROS
            </span>
          )}
        </div>
      </div>

      {/* Navigation */}
      <ScrollArea className="flex-1 px-3 py-4">
        <nav className="flex flex-col gap-1">
          {navigation.map((item) => (
            <NavItem key={item.href} item={item} />
          ))}
        </nav>
      </ScrollArea>

      {/* Bottom section */}
      <div className="border-t border-sidebar-border px-3 py-4">
        <nav className="flex flex-col gap-1">
          {bottomNavigation.map((item) => (
            <NavItem key={item.href} item={item} />
          ))}
        </nav>
        <Separator className="my-3" />
        <Button
          variant="ghost"
          size="sm"
          className="w-full justify-start"
          onClick={toggleSidebar}
        >
          {sidebarCollapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <>
              <ChevronLeft className="h-4 w-4" />
              <span className="ml-2">Collapse</span>
            </>
          )}
        </Button>
      </div>
    </aside>
  )
}
