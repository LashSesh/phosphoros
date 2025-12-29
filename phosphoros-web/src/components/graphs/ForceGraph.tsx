import { useEffect, useRef, useState } from 'react'
import * as d3 from 'd3'
import { cn } from '@/lib/utils'

export interface GraphNode {
  id: string
  label: string
  type: 'wallet' | 'contract' | 'exchange' | 'unknown'
  resonance: number
  cluster?: string
  x?: number
  y?: number
  fx?: number | null
  fy?: number | null
}

export interface GraphLink {
  source: string | GraphNode
  target: string | GraphNode
  weight: number
}

export interface ForceGraphProps {
  nodes: GraphNode[]
  links: GraphLink[]
  width?: number
  height?: number
  onNodeClick?: (node: GraphNode) => void
  onNodeHover?: (node: GraphNode | null) => void
  className?: string
}

const nodeColors: Record<GraphNode['type'], string> = {
  wallet: '#4088F2',
  contract: '#2EB894',
  exchange: '#F3A612',
  unknown: '#6B7280',
}

export function ForceGraph({
  nodes,
  links,
  width = 800,
  height = 600,
  onNodeClick,
  onNodeHover,
  className,
}: ForceGraphProps) {
  const svgRef = useRef<SVGSVGElement>(null)
  const [transform, setTransform] = useState(d3.zoomIdentity)

  useEffect(() => {
    if (!svgRef.current || nodes.length === 0) return

    const svg = d3.select(svgRef.current)
    svg.selectAll('*').remove()

    // Create container group for zoom
    const g = svg.append('g')

    // Create zoom behavior
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        g.attr('transform', event.transform)
        setTransform(event.transform)
      })

    svg.call(zoom)

    // Create arrow marker for directed edges
    svg.append('defs').append('marker')
      .attr('id', 'arrowhead')
      .attr('viewBox', '-0 -5 10 10')
      .attr('refX', 20)
      .attr('refY', 0)
      .attr('orient', 'auto')
      .attr('markerWidth', 6)
      .attr('markerHeight', 6)
      .append('path')
      .attr('d', 'M 0,-5 L 10,0 L 0,5')
      .attr('fill', '#6B7280')

    // Create force simulation
    const simulation = d3.forceSimulation(nodes as d3.SimulationNodeDatum[])
      .force('link', d3.forceLink(links)
        .id((d: any) => d.id)
        .distance(100)
        .strength((d: any) => d.weight * 0.5)
      )
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(30))

    // Create links
    const link = g.append('g')
      .attr('class', 'links')
      .selectAll('line')
      .data(links)
      .join('line')
      .attr('stroke', '#374151')
      .attr('stroke-opacity', 0.6)
      .attr('stroke-width', (d) => Math.sqrt(d.weight) * 2)
      .attr('marker-end', 'url(#arrowhead)')

    // Create node groups
    const node = g.append('g')
      .attr('class', 'nodes')
      .selectAll('g')
      .data(nodes)
      .join('g')
      .attr('cursor', 'pointer')
      .call(d3.drag<SVGGElement, GraphNode>()
        .on('start', (event, d) => {
          if (!event.active) simulation.alphaTarget(0.3).restart()
          d.fx = d.x
          d.fy = d.y
        })
        .on('drag', (event, d) => {
          d.fx = event.x
          d.fy = event.y
        })
        .on('end', (event, d) => {
          if (!event.active) simulation.alphaTarget(0)
          d.fx = null
          d.fy = null
        }) as any
      )

    // Add circles to nodes
    node.append('circle')
      .attr('r', (d) => 8 + d.resonance * 12)
      .attr('fill', (d) => nodeColors[d.type])
      .attr('stroke', '#1F2937')
      .attr('stroke-width', 2)
      .on('click', (event, d) => {
        event.stopPropagation()
        onNodeClick?.(d)
      })
      .on('mouseenter', (event, d) => {
        onNodeHover?.(d)
        d3.select(event.currentTarget)
          .transition()
          .duration(200)
          .attr('r', (d: any) => 10 + d.resonance * 14)
          .attr('stroke', '#fff')
      })
      .on('mouseleave', (event, d) => {
        onNodeHover?.(null)
        d3.select(event.currentTarget)
          .transition()
          .duration(200)
          .attr('r', (d: any) => 8 + d.resonance * 12)
          .attr('stroke', '#1F2937')
      })

    // Add labels to nodes
    node.append('text')
      .text((d) => d.label.slice(0, 8) + '...')
      .attr('x', 0)
      .attr('y', 25)
      .attr('text-anchor', 'middle')
      .attr('fill', '#9CA3AF')
      .attr('font-size', '10px')
      .attr('font-family', 'JetBrains Mono, monospace')
      .attr('pointer-events', 'none')

    // Add resonance indicator
    node.append('text')
      .text((d) => `${(d.resonance * 100).toFixed(0)}%`)
      .attr('x', 0)
      .attr('y', 4)
      .attr('text-anchor', 'middle')
      .attr('fill', '#fff')
      .attr('font-size', '8px')
      .attr('font-weight', 'bold')
      .attr('pointer-events', 'none')

    // Update positions on tick
    simulation.on('tick', () => {
      link
        .attr('x1', (d: any) => d.source.x)
        .attr('y1', (d: any) => d.source.y)
        .attr('x2', (d: any) => d.target.x)
        .attr('y2', (d: any) => d.target.y)

      node.attr('transform', (d: any) => `translate(${d.x},${d.y})`)
    })

    // Cleanup
    return () => {
      simulation.stop()
    }
  }, [nodes, links, width, height, onNodeClick, onNodeHover])

  return (
    <svg
      ref={svgRef}
      width={width}
      height={height}
      className={cn('bg-card rounded-lg', className)}
      style={{ maxWidth: '100%', height: 'auto' }}
    />
  )
}
