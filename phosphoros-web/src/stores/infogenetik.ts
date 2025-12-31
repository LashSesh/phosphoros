import { create } from 'zustand'
import { persist } from 'zustand/middleware'

export interface InfogenomData {
  id: string
  name: string
  numCells: number
  couplingStrength: number
  totalResonance: number
  status: 'idle' | 'evaluating' | 'exploring'
  createdAt: Date
}

export interface Point5D {
  x: number
  y: number
  z: number
  w: number
  v: number
}

export interface ExplorationResult {
  infogenomId: string
  steps: number
  bestResonance: number
  bestPoint: Point5D
  trajectory: Point5D[]
  completedAt: Date
}

interface InfogenetikState {
  // Infogenoms
  infogenoms: InfogenomData[]
  activeInfogenomId: string | null

  // Seed phrase embedding
  seedPhrase: string[]
  embeddings: Point5D[]

  // Exploration
  explorationResults: ExplorationResult[]
  isExploring: boolean
  explorationProgress: number

  // Operators (wave/spectral/etc)
  operators: {
    waveTransform: number
    spectralWeight: number
    doubleKick: number
    phaseIntegral: number
  }

  // Actions
  createInfogenom: (name: string, numCells: number) => void
  deleteInfogenom: (id: string) => void
  setActiveInfogenom: (id: string | null) => void
  setSeedPhrase: (words: string[]) => void
  embedSeedPhrase: () => void
  startExploration: (maxSteps: number) => void
  stopExploration: () => void
  setOperators: (operators: Partial<InfogenetikState['operators']>) => void
  resetOperators: () => void
  clearResults: () => void
}

const DEFAULT_OPERATORS = {
  waveTransform: 1.0,
  spectralWeight: 0.5,
  doubleKick: 0.3,
  phaseIntegral: 0.1,
}

// Simulate 5D embedding (this would call backend in production)
function simulateEmbed(word: string): Point5D {
  const hash = hashWord(word)
  const phi = (1 + Math.sqrt(5)) / 2 // Golden ratio
  return {
    x: (Math.sin(hash * phi) + 1) / 2,
    y: (Math.cos(hash * phi * 2) + 1) / 2,
    z: (Math.sin(hash * phi * 3) + 1) / 2,
    w: (Math.cos(hash * phi * 4) + 1) / 2,
    v: (Math.sin(hash * phi * 5) + 1) / 2,
  }
}

function hashWord(word: string): number {
  let hash = 0
  for (let i = 0; i < word.length; i++) {
    const char = word.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }
  return Math.abs(hash)
}

export const useInfogenetikStore = create<InfogenetikState>()(
  persist(
    (set, get) => ({
      infogenoms: [],
      activeInfogenomId: null,
      seedPhrase: [],
      embeddings: [],
      explorationResults: [],
      isExploring: false,
      explorationProgress: 0,
      operators: DEFAULT_OPERATORS,

      createInfogenom: (name, numCells) => {
        const id = crypto.randomUUID()
        const newInfogenom: InfogenomData = {
          id,
          name,
          numCells,
          couplingStrength: 0.5,
          totalResonance: 0,
          status: 'idle',
          createdAt: new Date(),
        }
        set((state) => ({
          infogenoms: [...state.infogenoms, newInfogenom],
          activeInfogenomId: id,
        }))
      },

      deleteInfogenom: (id) => {
        set((state) => ({
          infogenoms: state.infogenoms.filter((i) => i.id !== id),
          activeInfogenomId: state.activeInfogenomId === id ? null : state.activeInfogenomId,
        }))
      },

      setActiveInfogenom: (id) => set({ activeInfogenomId: id }),

      setSeedPhrase: (words) => set({ seedPhrase: words, embeddings: [] }),

      embedSeedPhrase: () => {
        const { seedPhrase } = get()
        if (seedPhrase.length === 0) return

        const embeddings = seedPhrase.map(simulateEmbed)
        set({ embeddings })
      },

      startExploration: (maxSteps) => {
        const { activeInfogenomId, infogenoms, embeddings, operators } = get()
        if (!activeInfogenomId || embeddings.length === 0) return

        set({ isExploring: true, explorationProgress: 0 })

        // Update infogenom status
        set({
          infogenoms: infogenoms.map((i) =>
            i.id === activeInfogenomId ? { ...i, status: 'exploring' as const } : i
          ),
        })

        // Simulate exploration with progress updates
        const totalSteps = maxSteps
        let currentStep = 0
        const trajectory: Point5D[] = []
        let bestResonance = 0
        let bestPoint: Point5D = embeddings[0]

        const interval = setInterval(() => {
          currentStep += Math.floor(Math.random() * 10) + 5
          if (currentStep >= totalSteps) {
            currentStep = totalSteps
          }

          // Simulate resonance calculation influenced by operators
          const operatorEffect =
            operators.waveTransform * 0.3 +
            operators.spectralWeight * 0.3 +
            operators.doubleKick * 0.2 +
            operators.phaseIntegral * 0.2

          const resonance = Math.random() * operatorEffect
          if (resonance > bestResonance) {
            bestResonance = resonance
            bestPoint = {
              x: Math.random(),
              y: Math.random(),
              z: Math.random(),
              w: Math.random(),
              v: Math.random(),
            }
          }

          trajectory.push({ ...bestPoint })

          set({ explorationProgress: (currentStep / totalSteps) * 100 })

          if (currentStep >= totalSteps) {
            clearInterval(interval)

            const result: ExplorationResult = {
              infogenomId: activeInfogenomId,
              steps: currentStep,
              bestResonance,
              bestPoint,
              trajectory,
              completedAt: new Date(),
            }

            set((state) => ({
              isExploring: false,
              explorationProgress: 100,
              explorationResults: [...state.explorationResults, result],
              infogenoms: state.infogenoms.map((i) =>
                i.id === activeInfogenomId
                  ? { ...i, status: 'idle' as const, totalResonance: bestResonance }
                  : i
              ),
            }))
          }
        }, 100)
      },

      stopExploration: () => {
        set((state) => ({
          isExploring: false,
          infogenoms: state.infogenoms.map((i) =>
            i.status === 'exploring' ? { ...i, status: 'idle' as const } : i
          ),
        }))
      },

      setOperators: (newOperators) => {
        set((state) => ({
          operators: { ...state.operators, ...newOperators },
        }))
      },

      resetOperators: () => set({ operators: DEFAULT_OPERATORS }),

      clearResults: () => set({ explorationResults: [] }),
    }),
    {
      name: 'phosphoros-infogenetik',
      partialize: (state) => ({
        infogenoms: state.infogenoms,
        operators: state.operators,
        explorationResults: state.explorationResults.slice(-10), // Keep last 10
      }),
    }
  )
)
