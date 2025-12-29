import { useEffect, useRef, useCallback } from 'react'
import { useServicesStore } from '@/stores/services'
import { useMetricsStore } from '@/stores/metrics'

type MessageType =
  | 'entity:discovered'
  | 'cluster:formed'
  | 'anomaly:detected'
  | 'analysis:progress'
  | 'service:status'
  | 'metrics:update'

interface WSMessage {
  type: MessageType
  payload: unknown
  timestamp: string
}

interface UseWebSocketOptions {
  url?: string
  reconnectInterval?: number
  maxRetries?: number
}

export function useWebSocket(options: UseWebSocketOptions = {}) {
  const {
    url = `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws`,
    reconnectInterval = 3000,
    maxRetries = 5,
  } = options

  const wsRef = useRef<WebSocket | null>(null)
  const retriesRef = useRef(0)
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>()

  const { setWebsocketStatus, updateService } = useServicesStore()
  const { setMetrics, addActivity } = useMetricsStore()

  const handleMessage = useCallback((event: MessageEvent) => {
    try {
      const message: WSMessage = JSON.parse(event.data)

      switch (message.type) {
        case 'entity:discovered':
          addActivity({
            type: 'entity',
            message: `Discovered entity ${(message.payload as any).address?.slice(0, 10)}...`,
          })
          setMetrics({ entities: (message.payload as any).totalEntities })
          break

        case 'cluster:formed':
          addActivity({
            type: 'cluster',
            message: `Formed cluster with ${(message.payload as any).memberCount} members`,
          })
          setMetrics({ clusters: (message.payload as any).totalClusters })
          break

        case 'anomaly:detected':
          addActivity({
            type: 'anomaly',
            message: `${(message.payload as any).severity} anomaly: ${(message.payload as any).description}`,
          })
          setMetrics({ anomalies: (message.payload as any).totalAnomalies })
          break

        case 'analysis:progress':
          addActivity({
            type: 'analysis',
            message: `Analysis ${(message.payload as any).progress}% complete`,
          })
          break

        case 'service:status':
          const { service, status, processed } = message.payload as any
          updateService(service, { status, processed })
          break

        case 'metrics:update':
          setMetrics(message.payload as any)
          break

        default:
          console.log('Unknown message type:', message.type)
      }
    } catch (error) {
      console.error('Failed to parse WebSocket message:', error)
    }
  }, [addActivity, setMetrics, updateService])

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) return

    setWebsocketStatus('connecting')

    try {
      const ws = new WebSocket(url)
      wsRef.current = ws

      ws.onopen = () => {
        console.log('WebSocket connected')
        setWebsocketStatus('connected')
        retriesRef.current = 0
      }

      ws.onmessage = handleMessage

      ws.onerror = (error) => {
        console.error('WebSocket error:', error)
      }

      ws.onclose = () => {
        console.log('WebSocket disconnected')
        setWebsocketStatus('disconnected')

        // Attempt reconnection
        if (retriesRef.current < maxRetries) {
          retriesRef.current += 1
          const delay = reconnectInterval * Math.pow(2, retriesRef.current - 1)
          console.log(`Reconnecting in ${delay}ms (attempt ${retriesRef.current}/${maxRetries})`)

          reconnectTimeoutRef.current = setTimeout(connect, delay)
        } else {
          console.log('Max retries reached, giving up')
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

  const send = useCallback((type: string, payload: unknown) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify({ type, payload }))
    }
  }, [])

  useEffect(() => {
    connect()
    return () => disconnect()
  }, [connect, disconnect])

  return { connect, disconnect, send }
}
