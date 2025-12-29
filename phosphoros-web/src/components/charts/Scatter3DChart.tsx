import ReactECharts from 'echarts-for-react'
import { useMemo } from 'react'

interface DataPoint {
  x: number
  y: number
  z: number
  resonance: number
  label?: string
}

interface Scatter3DChartProps {
  data: DataPoint[]
  title?: string
  height?: number
}

export function Scatter3DChart({ data, title, height = 400 }: Scatter3DChartProps) {
  const option = useMemo(() => ({
    title: title ? {
      text: title,
      textStyle: {
        color: '#E5E7EB',
        fontSize: 14,
      },
    } : undefined,
    tooltip: {
      backgroundColor: 'rgba(17, 24, 39, 0.9)',
      borderColor: '#374151',
      textStyle: {
        color: '#E5E7EB',
      },
      formatter: (params: any) => {
        const d = params.data
        return `
          <div style="font-family: 'JetBrains Mono', monospace; font-size: 12px;">
            <div>X: ${d[0].toFixed(3)}</div>
            <div>Y: ${d[1].toFixed(3)}</div>
            <div>Z: ${d[2].toFixed(3)}</div>
            <div style="color: ${getResonanceColor(d[3])}">Resonance: ${(d[3] * 100).toFixed(1)}%</div>
          </div>
        `
      },
    },
    visualMap: {
      show: true,
      dimension: 3,
      min: 0,
      max: 1,
      inRange: {
        color: ['#738EF3', '#59BFD9', '#F3C01F'],
      },
      textStyle: {
        color: '#9CA3AF',
      },
    },
    xAxis3D: {
      type: 'value',
      name: 'X',
      nameTextStyle: { color: '#9CA3AF' },
      axisLine: { lineStyle: { color: '#374151' } },
      axisLabel: { color: '#6B7280' },
      splitLine: { lineStyle: { color: '#1F2937' } },
    },
    yAxis3D: {
      type: 'value',
      name: 'Y',
      nameTextStyle: { color: '#9CA3AF' },
      axisLine: { lineStyle: { color: '#374151' } },
      axisLabel: { color: '#6B7280' },
      splitLine: { lineStyle: { color: '#1F2937' } },
    },
    zAxis3D: {
      type: 'value',
      name: 'Z',
      nameTextStyle: { color: '#9CA3AF' },
      axisLine: { lineStyle: { color: '#374151' } },
      axisLabel: { color: '#6B7280' },
      splitLine: { lineStyle: { color: '#1F2937' } },
    },
    grid3D: {
      viewControl: {
        autoRotate: true,
        autoRotateSpeed: 5,
      },
      boxWidth: 100,
      boxHeight: 100,
      boxDepth: 100,
      light: {
        main: { intensity: 1.2 },
        ambient: { intensity: 0.3 },
      },
    },
    series: [{
      type: 'scatter3D',
      data: data.map(d => [d.x, d.y, d.z, d.resonance]),
      symbolSize: (val: number[]) => 5 + val[3] * 15,
      itemStyle: {
        opacity: 0.8,
      },
      emphasis: {
        itemStyle: {
          opacity: 1,
          borderColor: '#fff',
          borderWidth: 2,
        },
      },
    }],
  }), [data, title])

  return (
    <ReactECharts
      option={option}
      style={{ height }}
      opts={{ renderer: 'canvas' }}
    />
  )
}

function getResonanceColor(value: number): string {
  if (value >= 0.7) return '#F3C01F'
  if (value >= 0.4) return '#59BFD9'
  return '#738EF3'
}
