import { useState } from 'react'
import { Search, Database, Filter, ArrowRight } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { formatAddress, formatResonance, getResonanceColor } from '@/lib/utils'
import { cn } from '@/lib/utils'

interface SearchResult {
  id: string
  address: string
  type: 'wallet' | 'contract' | 'exchange'
  resonance: number
  cluster?: string
}

export function ExplorerPage() {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResult[]>([
    { id: '1', address: '0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D', type: 'contract', resonance: 0.89, cluster: 'DEX' },
    { id: '2', address: '0xdAC17F958D2ee523a2206206994597C13D831ec7', type: 'contract', resonance: 0.76, cluster: 'Stablecoin' },
    { id: '3', address: '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa', type: 'wallet', resonance: 0.95 },
    { id: '4', address: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh', type: 'wallet', resonance: 0.42 },
  ])

  const handleSearch = () => {
    // Simulate search
    console.log('Searching for:', query)
  }

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'wallet': return 'bg-primary/10 text-primary'
      case 'contract': return 'bg-accent/10 text-accent'
      case 'exchange': return 'bg-warning/10 text-warning'
      default: return 'bg-muted text-muted-foreground'
    }
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Explorer</h1>
        <p className="text-muted-foreground">
          Search and explore entities in the forensic database
        </p>
      </div>

      {/* Search Bar */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex gap-4">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder="Search by address, cluster, or spectral signature..."
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                className="pl-10"
                onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              />
            </div>
            <Button onClick={handleSearch}>
              <Search className="mr-2 h-4 w-4" />
              Search
            </Button>
            <Button variant="outline">
              <Filter className="mr-2 h-4 w-4" />
              Filters
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Search Results */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Results ({results.length})</span>
            <Badge variant="outline">{results.length} entities found</Badge>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <ScrollArea className="h-[400px]">
            <div className="space-y-3">
              {results.map((result) => (
                <div
                  key={result.id}
                  className="flex items-center justify-between rounded-lg border p-4 hover:bg-muted/50 transition-colors cursor-pointer"
                >
                  <div className="flex items-center gap-4">
                    <div className={cn('rounded-md p-2', getTypeColor(result.type))}>
                      <Database className="h-4 w-4" />
                    </div>
                    <div>
                      <code className="text-sm font-address">
                        {formatAddress(result.address, 10)}
                      </code>
                      <div className="flex items-center gap-2 mt-1">
                        <Badge variant="outline" className="text-xs">
                          {result.type}
                        </Badge>
                        {result.cluster && (
                          <Badge variant="secondary" className="text-xs">
                            {result.cluster}
                          </Badge>
                        )}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <div className="text-right">
                      <p className="text-xs text-muted-foreground">Resonance</p>
                      <p className={cn('text-lg font-bold', getResonanceColor(result.resonance))}>
                        {formatResonance(result.resonance)}
                      </p>
                    </div>
                    <Button variant="ghost" size="icon">
                      <ArrowRight className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>
    </div>
  )
}
