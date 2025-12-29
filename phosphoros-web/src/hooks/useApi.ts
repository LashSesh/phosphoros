import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

const API_BASE = '/api'

// Types
export interface GatewayInfo {
  service: string
  version: string
  endpoints: Record<string, string>
}

export interface HealthResponse {
  status: string
  version: string
}

export interface EntityObservation {
  id: string
  address: string
  features: number[]
  connections: string[]
  metadata: Record<string, unknown>
}

export interface SnapshotIngest {
  label: string
  observations: EntityObservation[]
  context?: Record<string, unknown>
}

export interface SnapshotRecord {
  id: string
  label: string
  captured_at: string
  observations: EntityObservation[]
}

export interface AnalysisRequest {
  knn_k?: number
  entropy_bins?: number
  resonance_threshold?: number
}

export interface ResonanceHotspot {
  origin: string
  magnitude: number
  neighbours: string[]
}

export interface AnomalyScore {
  entity: string
  z_score: number
}

export interface AnalysisReport {
  snapshot_id: string
  label: string
  captured_at: string
  resonance_hotspots: ResonanceHotspot[]
  anomaly_scores: AnomalyScore[]
  topology: {
    components: number
    articulation_points: number
    betti_estimate: number[]
  }
  entropy: {
    spectral: number
    distribution: number[]
  }
}

// API functions
async function fetchJson<T>(path: string): Promise<T> {
  const response = await fetch(`${API_BASE}${path}`)
  if (!response.ok) {
    throw new Error(`API error: ${response.status} ${response.statusText}`)
  }
  return response.json()
}

async function postJson<T, B>(path: string, body: B): Promise<T> {
  const response = await fetch(`${API_BASE}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!response.ok) {
    throw new Error(`API error: ${response.status} ${response.statusText}`)
  }
  return response.json()
}

// Hooks

export function useGatewayInfo() {
  return useQuery({
    queryKey: ['gateway', 'info'],
    queryFn: () => fetchJson<GatewayInfo>('/'),
    staleTime: 60 * 1000,
  })
}

export function useHealth() {
  return useQuery({
    queryKey: ['gateway', 'health'],
    queryFn: () => fetchJson<HealthResponse>('/health'),
    refetchInterval: 30 * 1000,
  })
}

export function useSnapshots() {
  return useQuery({
    queryKey: ['satellite', 'snapshots'],
    queryFn: () => fetchJson<SnapshotRecord[]>('/satellite/v1/snapshots'),
  })
}

export function useIngestSnapshot() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (snapshot: SnapshotIngest) =>
      postJson<{ id: string; captured_at: string }, SnapshotIngest>(
        '/satellite/v1/snapshots',
        snapshot
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['satellite', 'snapshots'] })
    },
  })
}

export function useRunAnalysis() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ snapshotId, request }: { snapshotId: string; request: AnalysisRequest }) =>
      postJson<AnalysisReport, AnalysisRequest>(
        `/satellite/v1/analyze/${snapshotId}`,
        request
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['satellite', 'reports'] })
    },
  })
}

export function useLatestReport() {
  return useQuery({
    queryKey: ['satellite', 'reports', 'latest'],
    queryFn: () => fetchJson<AnalysisReport>('/satellite/v1/reports/latest'),
    retry: false,
  })
}
