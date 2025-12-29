import { Outlet } from 'react-router-dom'
import { TooltipProvider } from '@/components/ui/tooltip'
import { Sidebar } from './Sidebar'
import { Header } from './Header'
import { StatusBar } from './StatusBar'
import { CommandPalette } from '@/components/CommandPalette'
import { useAppStore } from '@/stores/app'
import { cn } from '@/lib/utils'

export function AppShell() {
  const { sidebarCollapsed } = useAppStore()

  return (
    <TooltipProvider delayDuration={0}>
      <div className="flex h-screen overflow-hidden bg-background">
        {/* Sidebar */}
        <Sidebar />

        {/* Main Content */}
        <div className="flex flex-1 flex-col overflow-hidden">
          {/* Header */}
          <Header />

          {/* Page Content */}
          <main
            className={cn(
              'flex-1 overflow-auto p-6',
              'transition-all duration-300'
            )}
          >
            <Outlet />
          </main>

          {/* Status Bar */}
          <StatusBar />
        </div>

        {/* Command Palette (Global) */}
        <CommandPalette />
      </div>
    </TooltipProvider>
  )
}
