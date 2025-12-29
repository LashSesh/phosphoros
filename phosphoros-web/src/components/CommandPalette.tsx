import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Command } from 'cmdk'
import {
  LayoutDashboard,
  Key,
  Activity,
  Network,
  Search,
  AlertTriangle,
  FileSearch,
  Settings,
  Plus,
  Download,
  Play,
  Moon,
  Sun,
} from 'lucide-react'
import { useAppStore } from '@/stores/app'
import { cn } from '@/lib/utils'

const pages = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard, keywords: 'home overview' },
  { name: 'Wallet & Seeds', href: '/wallet', icon: Key, keywords: 'mnemonic import' },
  { name: 'Resonance', href: '/resonance', icon: Activity, keywords: 'spectral analysis' },
  { name: 'Topology', href: '/topology', icon: Network, keywords: 'graph network' },
  { name: 'Explorer', href: '/explorer', icon: Search, keywords: 'search find' },
  { name: 'Anomalies', href: '/anomalies', icon: AlertTriangle, keywords: 'alerts suspicious' },
  { name: 'Forensics', href: '/forensics', icon: FileSearch, keywords: 'investigation workflow' },
  { name: 'Settings', href: '/settings', icon: Settings, keywords: 'config preferences' },
]

const actions = [
  { name: 'Import Mnemonic', icon: Plus, action: 'import-mnemonic', keywords: 'add seed wallet' },
  { name: 'Export Report', icon: Download, action: 'export-report', keywords: 'download save' },
  { name: 'Run Analysis', icon: Play, action: 'run-analysis', keywords: 'start analyze' },
]

export function CommandPalette() {
  const navigate = useNavigate()
  const { commandPaletteOpen, setCommandPaletteOpen, theme, setTheme } = useAppStore()
  const [search, setSearch] = useState('')

  // Keyboard shortcut
  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === 'k' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setCommandPaletteOpen(!commandPaletteOpen)
      }
      if (e.key === 'Escape') {
        setCommandPaletteOpen(false)
      }
    }

    document.addEventListener('keydown', down)
    return () => document.removeEventListener('keydown', down)
  }, [commandPaletteOpen, setCommandPaletteOpen])

  const handleSelect = (value: string) => {
    setCommandPaletteOpen(false)
    setSearch('')

    if (value.startsWith('/')) {
      navigate(value)
    } else if (value === 'toggle-theme') {
      setTheme(theme === 'dark' ? 'light' : 'dark')
    } else {
      // Handle other actions
      console.log('Action:', value)
    }
  }

  if (!commandPaletteOpen) return null

  return (
    <div className="fixed inset-0 z-50">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-background/80 backdrop-blur-sm"
        onClick={() => setCommandPaletteOpen(false)}
      />

      {/* Dialog */}
      <div className="absolute left-1/2 top-[20%] w-full max-w-lg -translate-x-1/2">
        <Command
          className="rounded-lg border bg-popover shadow-lg"
          loop
        >
          <Command.Input
            value={search}
            onValueChange={setSearch}
            placeholder="Type a command or search..."
            className="h-12 w-full border-b bg-transparent px-4 text-sm outline-none placeholder:text-muted-foreground"
          />
          <Command.List className="max-h-80 overflow-y-auto p-2">
            <Command.Empty className="py-6 text-center text-sm text-muted-foreground">
              No results found.
            </Command.Empty>

            <Command.Group heading="Pages" className="text-xs text-muted-foreground px-2 py-1.5">
              {pages.map((page) => (
                <Command.Item
                  key={page.href}
                  value={page.href + ' ' + page.name + ' ' + page.keywords}
                  onSelect={() => handleSelect(page.href)}
                  className={cn(
                    'flex cursor-pointer items-center gap-3 rounded-md px-3 py-2 text-sm',
                    'aria-selected:bg-accent aria-selected:text-accent-foreground'
                  )}
                >
                  <page.icon className="h-4 w-4 text-muted-foreground" />
                  <span>{page.name}</span>
                </Command.Item>
              ))}
            </Command.Group>

            <Command.Separator className="my-2 h-px bg-border" />

            <Command.Group heading="Actions" className="text-xs text-muted-foreground px-2 py-1.5">
              {actions.map((action) => (
                <Command.Item
                  key={action.action}
                  value={action.action + ' ' + action.name + ' ' + action.keywords}
                  onSelect={() => handleSelect(action.action)}
                  className={cn(
                    'flex cursor-pointer items-center gap-3 rounded-md px-3 py-2 text-sm',
                    'aria-selected:bg-accent aria-selected:text-accent-foreground'
                  )}
                >
                  <action.icon className="h-4 w-4 text-muted-foreground" />
                  <span>{action.name}</span>
                </Command.Item>
              ))}
              <Command.Item
                value="toggle-theme dark light mode"
                onSelect={() => handleSelect('toggle-theme')}
                className={cn(
                  'flex cursor-pointer items-center gap-3 rounded-md px-3 py-2 text-sm',
                  'aria-selected:bg-accent aria-selected:text-accent-foreground'
                )}
              >
                {theme === 'dark' ? (
                  <Sun className="h-4 w-4 text-muted-foreground" />
                ) : (
                  <Moon className="h-4 w-4 text-muted-foreground" />
                )}
                <span>{theme === 'dark' ? 'Light Mode' : 'Dark Mode'}</span>
              </Command.Item>
            </Command.Group>
          </Command.List>
        </Command>
      </div>
    </div>
  )
}
