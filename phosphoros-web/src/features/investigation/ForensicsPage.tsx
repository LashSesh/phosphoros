import { FileSearch, Clock, CheckCircle2, ArrowRight, Shield } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'

interface Workflow {
  id: string
  name: string
  description: string
  duration: string
  steps: number
  category: 'investigation' | 'analysis' | 'export'
  compliance?: string[]
}

const workflows: Workflow[] = [
  {
    id: 'sybil',
    name: 'Sybil Attack Investigation',
    description: 'Identify and cluster wallets exhibiting coordinated behavior patterns',
    duration: '30-45 min',
    steps: 6,
    category: 'investigation',
  },
  {
    id: 'laundering',
    name: 'Money Laundering Trace',
    description: 'Follow transaction chains through mixing services and identify endpoints',
    duration: '45-60 min',
    steps: 8,
    category: 'investigation',
    compliance: ['AML', 'KYC'],
  },
  {
    id: 'profiling',
    name: 'Address Profiling',
    description: 'Comprehensive analysis of a single address including all related entities',
    duration: '20-30 min',
    steps: 5,
    category: 'analysis',
  },
  {
    id: 'cluster',
    name: 'Cluster Forensics',
    description: 'Deep analysis of entity clusters with topological metrics',
    duration: '30-40 min',
    steps: 7,
    category: 'analysis',
  },
  {
    id: 'monero',
    name: 'Monero Ring Signature Analysis',
    description: 'Quantum-enhanced QAOA analysis for ring signature decomposition',
    duration: '60-90 min',
    steps: 10,
    category: 'investigation',
    compliance: ['IRS'],
  },
  {
    id: 'evidence',
    name: 'Evidence Package Export',
    description: 'Generate court-admissible evidence packages with full audit trail',
    duration: '15-20 min',
    steps: 4,
    category: 'export',
    compliance: ['Legal'],
  },
]

const categoryColors = {
  investigation: 'bg-destructive/10 text-destructive',
  analysis: 'bg-primary/10 text-primary',
  export: 'bg-success/10 text-success',
}

export function ForensicsPage() {
  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Forensic Workflows</h1>
        <p className="text-muted-foreground">
          Guided investigation procedures for blockchain forensics
        </p>
      </div>

      {/* Workflow Categories */}
      <div className="grid gap-4 sm:grid-cols-3">
        <Card className="border-destructive/20">
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <div className="rounded-md bg-destructive/10 p-2">
                <FileSearch className="h-5 w-5 text-destructive" />
              </div>
              <div>
                <p className="font-medium">Investigation</p>
                <p className="text-sm text-muted-foreground">Active threat hunting</p>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card className="border-primary/20">
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <div className="rounded-md bg-primary/10 p-2">
                <Shield className="h-5 w-5 text-primary" />
              </div>
              <div>
                <p className="font-medium">Analysis</p>
                <p className="text-sm text-muted-foreground">Deep forensic analysis</p>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card className="border-success/20">
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <div className="rounded-md bg-success/10 p-2">
                <CheckCircle2 className="h-5 w-5 text-success" />
              </div>
              <div>
                <p className="font-medium">Export</p>
                <p className="text-sm text-muted-foreground">Evidence packaging</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Workflow Cards */}
      <div className="grid gap-4 lg:grid-cols-2">
        {workflows.map((workflow) => (
          <Card key={workflow.id} className="overflow-hidden">
            <CardHeader className="pb-3">
              <div className="flex items-start justify-between">
                <div className="space-y-1">
                  <CardTitle className="text-lg">{workflow.name}</CardTitle>
                  <CardDescription>{workflow.description}</CardDescription>
                </div>
                <Badge className={categoryColors[workflow.category]}>
                  {workflow.category}
                </Badge>
              </div>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4 text-sm text-muted-foreground">
                  <div className="flex items-center gap-1">
                    <Clock className="h-4 w-4" />
                    <span>{workflow.duration}</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <CheckCircle2 className="h-4 w-4" />
                    <span>{workflow.steps} steps</span>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {workflow.compliance?.map((tag) => (
                    <Badge key={tag} variant="outline" className="text-xs">
                      {tag}
                    </Badge>
                  ))}
                  <Button size="sm">
                    Start
                    <ArrowRight className="ml-2 h-4 w-4" />
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Recent Investigations */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Investigations</CardTitle>
          <CardDescription>Your recently completed forensic workflows</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground">
            <FileSearch className="h-12 w-12 mx-auto mb-4 opacity-50" />
            <p>No recent investigations</p>
            <p className="text-sm">Start a workflow above to begin</p>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
