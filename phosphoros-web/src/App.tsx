import { Routes, Route } from 'react-router-dom'
import { useEffect } from 'react'
import { AppShell } from '@/components/layout/AppShell'
import { Dashboard } from '@/features/dashboard/Dashboard'
import { WalletPage } from '@/features/wallet/WalletPage'
import { InfogenetikPage } from '@/features/infogenetik/InfogenetikPage'
import { ResonancePage } from '@/features/resonance/ResonancePage'
import { TopologyPage } from '@/features/topology/TopologyPage'
import { ExplorerPage } from '@/features/investigation/ExplorerPage'
import { AnomaliesPage } from '@/features/investigation/AnomaliesPage'
import { ForensicsPage } from '@/features/investigation/ForensicsPage'
import { SettingsPage } from '@/features/settings/SettingsPage'
import { useAppStore } from '@/stores/app'

function App() {
  const { theme } = useAppStore()

  // Apply theme class to document
  useEffect(() => {
    const root = document.documentElement
    root.classList.remove('light', 'dark')
    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light'
      root.classList.add(systemTheme)
    } else {
      root.classList.add(theme)
    }
  }, [theme])

  return (
    <Routes>
      <Route element={<AppShell />}>
        <Route path="/" element={<Dashboard />} />
        <Route path="/wallet" element={<WalletPage />} />
        <Route path="/infogenetik" element={<InfogenetikPage />} />
        <Route path="/resonance" element={<ResonancePage />} />
        <Route path="/topology" element={<TopologyPage />} />
        <Route path="/explorer" element={<ExplorerPage />} />
        <Route path="/anomalies" element={<AnomaliesPage />} />
        <Route path="/forensics" element={<ForensicsPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  )
}

export default App
