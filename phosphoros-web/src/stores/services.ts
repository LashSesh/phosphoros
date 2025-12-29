import { create } from 'zustand'

export type ServiceStatus = 'online' | 'offline' | 'pending'

interface ServiceInfo {
  name: string
  status: ServiceStatus
  processed: number
  lastUpdate: Date | null
}

interface ServicesState {
  gateway: ServiceInfo
  scraper: ServiceInfo
  analyzer: ServiceInfo
  cluster: ServiceInfo
  websocket: 'connected' | 'disconnected' | 'connecting'

  // Actions
  updateService: (name: keyof Omit<ServicesState, 'websocket' | 'updateService' | 'setWebsocketStatus'>, update: Partial<ServiceInfo>) => void
  setWebsocketStatus: (status: 'connected' | 'disconnected' | 'connecting') => void
}

export const useServicesStore = create<ServicesState>((set) => ({
  gateway: { name: 'Gateway', status: 'pending', processed: 0, lastUpdate: null },
  scraper: { name: 'Scraper', status: 'offline', processed: 0, lastUpdate: null },
  analyzer: { name: 'Analyzer', status: 'offline', processed: 0, lastUpdate: null },
  cluster: { name: 'Cluster Engine', status: 'offline', processed: 0, lastUpdate: null },
  websocket: 'disconnected',

  updateService: (name, update) =>
    set((state) => ({
      [name]: { ...state[name], ...update },
    })),

  setWebsocketStatus: (status) => set({ websocket: status }),
}))
