import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

const API_BASE = '/api'

// ============================================================================
// Gateway Types
// ============================================================================

export interface GatewayInfo {
  service: string
  version: string
  endpoints: Record<string, string>
}

export interface HealthResponse {
  status: string
  version: string
}

// ============================================================================
// Satellite Types (existing)
// ============================================================================

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

// ============================================================================
// Resonance API Types (NEW)
// ============================================================================

export interface AnalyzeResonanceRequest {
  t: number
  perception: [number, number, number, number, number]
  intention: [number, number, number, number, number]
  gradient: [number, number, number, number, number]
  theta: number
  label?: string
}

export type AnalyzeResonanceResponse =
  | {
      type: 'Output'
      vector: [number, number, number, number, number]
      score: number
      label?: string
    }
  | {
      type: 'Gated'
      reason: string
      label?: string
    }

export interface SpectralRequest {
  psi: number
  rho: number
  omega: number
}

export interface SpectralResponse {
  psi: number
  rho: number
  omega: number
  resonance: number
  energy: number
}

export interface ResonanceRecord {
  timestamp: string
  request: AnalyzeResonanceRequest
  response: AnalyzeResonanceResponse
}

// ============================================================================
// Wallet API Types (NEW)
// ============================================================================

export interface ImportMnemonicRequest {
  phrase: string
  language?: string
  label: string
}

export interface ImportMnemonicResponse {
  success: boolean
  error?: string
  mnemonic?: string
}

export interface DeriveAddressesRequest {
  phrase: string
  blockchains: string[]
  account?: number
  address_range?: { start: number; end: number }
  passphrase?: string
  label?: string
}

export interface DeriveAddressesResponse {
  success: boolean
  error?: string
  addresses: Record<string, string[]>
}

export interface WalletInfo {
  label: string
  blockchain: string
  addresses: string[]
  imported_at: string
}

export interface ListWalletsResponse {
  wallets: WalletInfo[]
}

export interface RemoveWalletRequest {
  label: string
}

// ============================================================================
// Cluster API Types (NEW)
// ============================================================================

export interface Entity {
  address: string
  features: number[]
}

export interface ComputeClustersRequest {
  snapshot_id: string
  entities: Entity[]
  algorithm?: 'knn' | 'dbscan' | 'hierarchical'
  k?: number
  threshold?: number
}

export interface ClusterSummary {
  id: string
  label?: string
  size: number
  cohesion: number
}

export interface ComputeClustersResponse {
  success: boolean
  error?: string
  num_clusters: number
  clusters: ClusterSummary[]
}

export interface Cluster {
  id: string
  label?: string
  size: number
  centroid: number[]
  cohesion: number
  members: string[]
  metadata: Record<string, string>
}

export interface ListClustersResponse {
  snapshots: Record<string, ClusterSummary[]>
}

export interface ClusterMembersResponse {
  cluster_id: string
  members: string[]
  count: number
}

// ============================================================================
// API Helper Functions
// ============================================================================

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

// ============================================================================
// Gateway Hooks
// ============================================================================

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

// ============================================================================
// Satellite Hooks (existing)
// ============================================================================

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

// ============================================================================
// Resonance API Hooks (NEW)
// ============================================================================

export function useAnalyzeResonance() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (request: AnalyzeResonanceRequest) =>
      postJson<AnalyzeResonanceResponse, AnalyzeResonanceRequest>(
        '/v1/resonance/analyze',
        request
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['resonance', 'history'] })
    },
  })
}

export function useComputeSpectral() {
  return useMutation({
    mutationFn: (request: SpectralRequest) =>
      postJson<SpectralResponse, SpectralRequest>('/v1/resonance/spectral', request),
  })
}

export function useResonanceHistory() {
  return useQuery({
    queryKey: ['resonance', 'history'],
    queryFn: () => fetchJson<ResonanceRecord[]>('/v1/resonance/history'),
  })
}

export function useClearResonanceHistory() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: () =>
      fetch(`${API_BASE}/v1/resonance/history/clear`, { method: 'POST' }).then(() => undefined),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['resonance', 'history'] })
    },
  })
}

// ============================================================================
// Wallet API Hooks (NEW)
// ============================================================================

export function useImportMnemonic() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (request: ImportMnemonicRequest) =>
      postJson<ImportMnemonicResponse, ImportMnemonicRequest>('/v1/wallet/import', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['wallet', 'list'] })
    },
  })
}

export function useDeriveAddresses() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (request: DeriveAddressesRequest) =>
      postJson<DeriveAddressesResponse, DeriveAddressesRequest>('/v1/wallet/derive', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['wallet', 'list'] })
    },
  })
}

export function useWalletList() {
  return useQuery({
    queryKey: ['wallet', 'list'],
    queryFn: () => fetchJson<ListWalletsResponse>('/v1/wallet/list'),
  })
}

export function useRemoveWallet() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (request: RemoveWalletRequest) =>
      postJson<void, RemoveWalletRequest>('/v1/wallet/remove', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['wallet', 'list'] })
    },
  })
}

// ============================================================================
// Cluster API Hooks (NEW)
// ============================================================================

export function useComputeClusters() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (request: ComputeClustersRequest) =>
      postJson<ComputeClustersResponse, ComputeClustersRequest>('/v1/cluster/compute', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['cluster', 'list'] })
    },
  })
}

export function useAllClusters() {
  return useQuery({
    queryKey: ['cluster', 'list'],
    queryFn: () => fetchJson<ListClustersResponse>('/v1/cluster/list'),
  })
}

export function useClustersForSnapshot(snapshotId: string, enabled = true) {
  return useQuery({
    queryKey: ['cluster', 'snapshot', snapshotId],
    queryFn: () => fetchJson<ClusterSummary[]>(`/v1/cluster/${snapshotId}`),
    enabled,
  })
}

export function useClusterMembers(snapshotId: string, clusterId: string, enabled = true) {
  return useQuery({
    queryKey: ['cluster', 'members', snapshotId, clusterId],
    queryFn: () => fetchJson<ClusterMembersResponse>(`/v1/cluster/${snapshotId}/${clusterId}/members`),
    enabled,
  })
}
