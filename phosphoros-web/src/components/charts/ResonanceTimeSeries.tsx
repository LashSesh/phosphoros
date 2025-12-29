import ReactECharts from 'echarts-for-react'
import { useMemo } from 'react'

interface TimeSeriesData {
  timestamp: Date | string
  psi: number
  rho: number
  omega: number
  resonance: number
}

interface ResonanceTimeSeriesProps {
  data: TimeSeriesData[]
  height?: number
}

export function ResonanceTimeSeries({ data, height = 300 }: ResonanceTimeSeriesProps) {
  const option = useMemo(() => {
    const timestamps = data.map(d =>
      typeof d.timestamp === 'string' ? d.timestamp : d.timestamp.toLocaleTimeString()
    )

    return {
      tooltip: {
        trigger: 'axis',
        backgroundColor: 'rgba(17, 24, 39, 0.95)',
        borderColor: '#374151',
        textStyle: {
          color: '#E5E7EB',
          fontSize: 12,
        },
        axisPointer: {
          type: 'cross',
          crossStyle: {
            color: '#6B7280',
          },
        },
      },
      legend: {
        data: ['ψ (Coherence)', 'ρ (Stability)', 'ω (Efficiency)', 'Resonance'],
        textStyle: {
          color: '#9CA3AF',
        },
        top: 0,
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        top: 40,
        containLabel: true,
      },
      xAxis: {
        type: 'category',
        data: timestamps,
        axisLine: { lineStyle: { color: '#374151' } },
        axisLabel: { color: '#6B7280', fontSize: 10 },
        splitLine: { show: false },
      },
      yAxis: {
        type: 'value',
        min: 0,
        max: 1,
        axisLine: { lineStyle: { color: '#374151' } },
        axisLabel: { color: '#6B7280', formatter: (val: number) => `${(val * 100).toFixed(0)}%` },
        splitLine: { lineStyle: { color: '#1F2937' } },
      },
      series: [
        {
          name: 'ψ (Coherence)',
          type: 'line',
          data: data.map(d => d.psi),
          smooth: true,
          lineStyle: { color: '#22D3EE', width: 2 },
          itemStyle: { color: '#22D3EE' },
          areaStyle: { color: 'rgba(34, 211, 238, 0.1)' },
        },
        {
          name: 'ρ (Stability)',
          type: 'line',
          data: data.map(d => d.rho),
          smooth: true,
          lineStyle: { color: '#3B82F6', width: 2 },
          itemStyle: { color: '#3B82F6' },
          areaStyle: { color: 'rgba(59, 130, 246, 0.1)' },
        },
        {
          name: 'ω (Efficiency)',
          type: 'line',
          data: data.map(d => d.omega),
          smooth: true,
          lineStyle: { color: '#FBBF24', width: 2 },
          itemStyle: { color: '#FBBF24' },
          areaStyle: { color: 'rgba(251, 191, 36, 0.1)' },
        },
        {
          name: 'Resonance',
          type: 'line',
          data: data.map(d => d.resonance),
          smooth: true,
          lineStyle: { color: '#10B981', width: 3 },
          itemStyle: { color: '#10B981' },
          emphasis: {
            lineStyle: { width: 4 },
          },
        },
      ],
    }
  }, [data])

  return (
    <ReactECharts
      option={option}
      style={{ height }}
      opts={{ renderer: 'canvas' }}
    />
  )
}
