import { create } from 'zustand'

interface MetricsState {
  seeds: number
  clusters: number
  entities: number
  anomalies: number
  analyses: number
  hotspots: number

  // Recent activity
  recentActivity: Array<{
    id: string
    type: 'entity' | 'cluster' | 'anomaly' | 'analysis'
    message: string
    timestamp: Date
  }>

  // Actions
  setMetrics: (metrics: Partial<Omit<MetricsState, 'recentActivity' | 'setMetrics' | 'addActivity'>>) => void
  addActivity: (activity: Omit<MetricsState['recentActivity'][0], 'id' | 'timestamp'>) => void
}

export const useMetricsStore = create<MetricsState>((set) => ({
  seeds: 0,
  clusters: 0,
  entities: 0,
  anomalies: 0,
  analyses: 0,
  hotspots: 0,
  recentActivity: [],

  setMetrics: (metrics) => set((state) => ({ ...state, ...metrics })),

  addActivity: (activity) =>
    set((state) => ({
      recentActivity: [
        {
          ...activity,
          id: crypto.randomUUID(),
          timestamp: new Date(),
        },
        ...state.recentActivity,
      ].slice(0, 50), // Keep last 50 activities
    })),
}))
