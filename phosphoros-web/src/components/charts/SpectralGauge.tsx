import ReactECharts from 'echarts-for-react'
import { useMemo } from 'react'

interface GaugeData {
  value: number
  name: string
}

interface SpectralGaugeProps {
  psi: number
  rho: number
  omega: number
  height?: number
}

export function SpectralGauge({ psi, rho, omega, height = 200 }: SpectralGaugeProps) {
  const resonance = psi * rho * omega

  const option = useMemo(() => ({
    series: [
      {
        type: 'gauge',
        startAngle: 180,
        endAngle: 0,
        min: 0,
        max: 1,
        splitNumber: 10,
        radius: '90%',
        center: ['50%', '75%'],
        axisLine: {
          lineStyle: {
            width: 20,
            color: [
              [0.3, '#738EF3'],
              [0.7, '#59BFD9'],
              [1, '#F3C01F'],
            ],
          },
        },
        pointer: {
          icon: 'path://M12.8,0.7l12,40.1H0.7L12.8,0.7z',
          length: '60%',
          width: 12,
          offsetCenter: [0, '-10%'],
          itemStyle: {
            color: 'auto',
          },
        },
        axisTick: {
          length: 8,
          lineStyle: {
            color: 'auto',
            width: 2,
          },
        },
        splitLine: {
          length: 15,
          lineStyle: {
            color: 'auto',
            width: 3,
          },
        },
        axisLabel: {
          color: '#6B7280',
          fontSize: 10,
          distance: -40,
          formatter: (value: number) => {
            if (value === 0) return '0%'
            if (value === 0.5) return '50%'
            if (value === 1) return '100%'
            return ''
          },
        },
        title: {
          offsetCenter: [0, '20%'],
          fontSize: 12,
          color: '#9CA3AF',
        },
        detail: {
          fontSize: 28,
          offsetCenter: [0, '-20%'],
          valueAnimation: true,
          formatter: (value: number) => `${(value * 100).toFixed(1)}%`,
          color: resonance >= 0.7 ? '#F3C01F' : resonance >= 0.4 ? '#59BFD9' : '#738EF3',
        },
        data: [
          {
            value: resonance,
            name: 'Resonance',
          },
        ],
      },
    ],
  }), [psi, rho, omega, resonance])

  return (
    <ReactECharts
      option={option}
      style={{ height }}
      opts={{ renderer: 'canvas' }}
    />
  )
}
