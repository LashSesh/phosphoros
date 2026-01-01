import { useEffect, useRef, useCallback } from 'react'
import { useServicesStore } from '@/stores/services'
import { useMetricsStore } from '@/stores/metrics'

// ============================================================================
// Gateway Event Types (matching Rust backend)
// ============================================================================

export type GatewayEvent =
  | {
      type: 'Log'
      timestamp: string
      level: string
      message: string
      source?: string
    }
  | {
      type: 'AnalysisProgress'
      id: string
      progress: number
      step: string
    }
  | {
      type: 'AnalysisComplete'
      id: string
      success: boolean
      error?: string
    }
  | {
      type: 'ServiceStatus'
      service: string
      status: string
      message?: string
    }
  | {
      type: 'ResonanceEvaluated'
      timestamp: string
      score?: number
      gated: boolean
      label?: string
    }
  | {
      type: 'WalletDerived'
      timestamp: string
      blockchain: string
      count: number
    }
  | {
      type: 'ClusterComputed'
      timestamp: string
      snapshot_id: string
      num_clusters: number
    }
  | {
      type: 'Notification'
      level: string
      title: string
      message: string
    }

interface UseWebSocketOptions {
  url?: string
  reconnectInterval?: number
  maxRetries?: number
  onEvent?: (event: GatewayEvent) => void
}

export function useWebSocket(options: UseWebSocketOptions = {}) {
  const {
    url = `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws`,
    reconnectInterval = 3000,
    maxRetries = 5,
    onEvent,
  } = options

  const wsRef = useRef<WebSocket | null>(null)
  const retriesRef = useRef(0)
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>()

  const { setWebsocketStatus, updateService } = useServicesStore()
  const { setMetrics, addActivity } = useMetricsStore()

  const handleMessage = useCallback(
    (event: MessageEvent) => {
      try {
        const gatewayEvent: GatewayEvent = JSON.parse(event.data)

        // Call custom event handler if provided
        onEvent?.(gatewayEvent)

        // Handle event based on type
        switch (gatewayEvent.type) {
          case 'Log':
            // Add log to activity feed
            addActivity({
              type: 'log',
              message: `[${gatewayEvent.level.toUpperCase()}] ${gatewayEvent.message}`,
            })
            break

          case 'AnalysisProgress':
            // Update analysis progress in metrics
            addActivity({
              type: 'analysis',
              message: `${gatewayEvent.step}: ${gatewayEvent.progress}%`,
            })
            setMetrics({
              analysisProgress: gatewayEvent.progress,
            })
            break

          case 'AnalysisComplete':
            addActivity({
              type: gatewayEvent.success ? 'success' : 'error',
              message: gatewayEvent.success
                ? `Analysis ${gatewayEvent.id} completed`
                : `Analysis ${gatewayEvent.id} failed: ${gatewayEvent.error}`,
            })
            setMetrics({ analysisProgress: undefined })
            break

          case 'ServiceStatus':
            // Update service status in store
            updateService(gatewayEvent.service, {
              status: gatewayEvent.status as 'running' | 'stopped' | 'error',
              processed: 0, // Backend should provide this
            })
            addActivity({
              type: 'service',
              message: `${gatewayEvent.service}: ${gatewayEvent.status}${
                gatewayEvent.message ? ` - ${gatewayEvent.message}` : ''
              }`,
            })
            break

          case 'ResonanceEvaluated':
            // Add resonance evaluation to activity
            const resonanceMsg = gatewayEvent.gated
              ? `Resonance gated${gatewayEvent.label ? ` (${gatewayEvent.label})` : ''}`
              : `Resonance: ${gatewayEvent.score?.toFixed(4)}${
                  gatewayEvent.label ? ` (${gatewayEvent.label})` : ''
                }`
            addActivity({
              type: 'resonance',
              message: resonanceMsg,
            })
            break

          case 'WalletDerived':
            // Add wallet derivation to activity
            addActivity({
              type: 'wallet',
              message: `Derived ${gatewayEvent.count} ${gatewayEvent.blockchain} address${
                gatewayEvent.count > 1 ? 'es' : ''
              }`,
            })
            break

          case 'ClusterComputed':
            // Add cluster computation to activity
            addActivity({
              type: 'cluster',
              message: `Computed ${gatewayEvent.num_clusters} cluster${
                gatewayEvent.num_clusters !== 1 ? 's' : ''
              } for ${gatewayEvent.snapshot_id}`,
            })
            setMetrics({ clusters: gatewayEvent.num_clusters })
            break

          case 'Notification':
            // Show notification in activity feed
            addActivity({
              type: gatewayEvent.level === 'error' ? 'error' :
                   gatewayEvent.level === 'warning' ? 'warning' :
                   gatewayEvent.level === 'success' ? 'success' : 'info',
              message: `${gatewayEvent.title}: ${gatewayEvent.message}`,
            })
            break

          default:
            console.log('Unknown gateway event type:', gatewayEvent)
        }
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error)
      }
    },
    [addActivity, setMetrics, updateService, onEvent]
  )

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) return

    setWebsocketStatus('connecting')

    try {
      const ws = new WebSocket(url)
      wsRef.current = ws

      ws.onopen = () => {
        console.log('WebSocket connected to PHOSPHOROS Gateway')
        setWebsocketStatus('connected')
        retriesRef.current = 0
      }

      ws.onmessage = handleMessage

      ws.onerror = (error) => {
        console.error('WebSocket error:', error)
      }

      ws.onclose = () => {
        console.log('WebSocket disconnected from PHOSPHOROS Gateway')
        setWebsocketStatus('disconnected')

        // Attempt reconnection with exponential backoff
        if (retriesRef.current < maxRetries) {
          retriesRef.current += 1
          const delay = reconnectInterval * Math.pow(2, retriesRef.current - 1)
          console.log(`Reconnecting in ${delay}ms (attempt ${retriesRef.current}/${maxRetries})`)

          reconnectTimeoutRef.current = setTimeout(connect, delay)
        } else {
          console.log('Max WebSocket retries reached, giving up')
        }
      }
    } catch (error) {
      console.error('Failed to create WebSocket:', error)
      setWebsocketStatus('disconnected')
    }
  }, [url, reconnectInterval, maxRetries, handleMessage, setWebsocketStatus])

  const disconnect = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current)
    }
    if (wsRef.current) {
      wsRef.current.close()
      wsRef.current = null
    }
    setWebsocketStatus('disconnected')
  }, [setWebsocketStatus])

  const send = useCallback((data: unknown) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(data))
    } else {
      console.warn('WebSocket not connected, cannot send message')
    }
  }, [])

  useEffect(() => {
    connect()
    return () => disconnect()
  }, [connect, disconnect])

  return {
    connect,
    disconnect,
    send,
    isConnected: wsRef.current?.readyState === WebSocket.OPEN,
  }
}
