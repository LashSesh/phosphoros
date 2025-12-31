import { useState, useCallback, useEffect, useMemo } from 'react'
import { useSearchParams, useNavigate } from 'react-router-dom'
import { Search, Database, Filter, ArrowRight, Loader2, X, Network, ExternalLink } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { formatAddress, formatResonance, getResonanceColor } from '@/lib/utils'
import { cn } from '@/lib/utils'

interface SearchResult {
  id: string
  address: string
  type: 'wallet' | 'contract' | 'exchange'
  resonance: number
  cluster?: string
  transactions?: number
  balance?: string
}

// Mock database for search
const mockDatabase: SearchResult[] = [
  { id: '1', address: '0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D', type: 'contract', resonance: 0.89, cluster: 'DEX', transactions: 15420, balance: '1,234 ETH' },
  { id: '2', address: '0xdAC17F958D2ee523a2206206994597C13D831ec7', type: 'contract', resonance: 0.76, cluster: 'Stablecoin', transactions: 892341, balance: '$2.1B USDT' },
  { id: '3', address: '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa', type: 'wallet', resonance: 0.95, transactions: 2, balance: '68.35 BTC' },
  { id: '4', address: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh', type: 'wallet', resonance: 0.42, transactions: 127, balance: '0.5 BTC' },
  { id: '5', address: '0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984', type: 'contract', resonance: 0.82, cluster: 'UNI Token', transactions: 423891, balance: '50M UNI' },
  { id: '6', address: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2', type: 'contract', resonance: 0.91, cluster: 'WETH', transactions: 1892341, balance: '3.2M WETH' },
  { id: '7', address: '0x6B175474E89094C44Da98b954EescdeCB5c33200', type: 'contract', resonance: 0.78, cluster: 'DAI', transactions: 234123, balance: '$890M DAI' },
  { id: '8', address: 'binance-hot-wallet-1', type: 'exchange', resonance: 0.65, cluster: 'Binance', transactions: 12341234, balance: '2.5M ETH' },
  { id: '9', address: 'coinbase-custody-2', type: 'exchange', resonance: 0.71, cluster: 'Coinbase', transactions: 892341, balance: '120K BTC' },
  { id: '10', address: '0x28C6c06298d514Db089934071355E5743bf21d60', type: 'exchange', resonance: 0.68, cluster: 'Binance', transactions: 2341234, balance: '1.2M ETH' },
]

export function ExplorerPage() {
  const [searchParams] = useSearchParams()
  const navigate = useNavigate()
  const [query, setQuery] = useState(searchParams.get('address') || '')
  const [results, setResults] = useState<SearchResult[]>(mockDatabase)
  const [isSearching, setIsSearching] = useState(false)
  const [selectedResult, setSelectedResult] = useState<SearchResult | null>(null)
  const [typeFilter, setTypeFilter] = useState<Set<SearchResult['type']>>(
    new Set(['wallet', 'contract', 'exchange'])
  )
  const [showFilters, setShowFilters] = useState(false)

  // Handle URL parameter for address search
  useEffect(() => {
    const addressParam = searchParams.get('address')
    if (addressParam) {
      setQuery(addressParam)
      performSearch(addressParam)
    }
  }, [])

  const performSearch = useCallback(async (searchQuery: string) => {
    const q = searchQuery.toLowerCase().trim()
    if (!q) {
      setResults(mockDatabase)
      return
    }

    setIsSearching(true)
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 500))

    const filtered = mockDatabase.filter(item =>
      item.address.toLowerCase().includes(q) ||
      (item.cluster?.toLowerCase().includes(q)) ||
      item.type.toLowerCase().includes(q)
    )

    setResults(filtered)
    setIsSearching(false)
  }, [])

  const handleSearch = useCallback(() => {
    performSearch(query)
  }, [query, performSearch])

  // Filter results by type
  const filteredResults = useMemo(() => {
    return results.filter(r => typeFilter.has(r.type))
  }, [results, typeFilter])

  const toggleTypeFilter = useCallback((type: SearchResult['type']) => {
    setTypeFilter(prev => {
      const newSet = new Set(prev)
      if (newSet.has(type)) {
        newSet.delete(type)
      } else {
        newSet.add(type)
      }
      return newSet
    })
  }, [])

  const handleViewEntity = useCallback((result: SearchResult) => {
    setSelectedResult(result)
  }, [])

  const handleViewInTopology = useCallback((result: SearchResult) => {
    navigate('/topology')
  }, [navigate])

  const clearSearch = useCallback(() => {
    setQuery('')
    setResults(mockDatabase)
    setSelectedResult(null)
  }, [])

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
                placeholder="Search by address, cluster, or type..."
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                className="pl-10 pr-10"
                onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              />
              {query && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="absolute right-1 top-1/2 -translate-y-1/2 h-7 w-7"
                  onClick={clearSearch}
                >
                  <X className="h-4 w-4" />
                </Button>
              )}
            </div>
            <Button onClick={handleSearch} disabled={isSearching}>
              {isSearching ? (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              ) : (
                <Search className="mr-2 h-4 w-4" />
              )}
              Search
            </Button>
            <Button
              variant={showFilters ? 'default' : 'outline'}
              onClick={() => setShowFilters(!showFilters)}
            >
              <Filter className="mr-2 h-4 w-4" />
              Filters
            </Button>
          </div>

          {/* Filter Panel */}
          {showFilters && (
            <div className="mt-4 pt-4 border-t">
              <p className="text-sm font-medium mb-2">Filter by Type</p>
              <div className="flex gap-2">
                <Badge
                  variant={typeFilter.has('wallet') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleTypeFilter('wallet')}
                >
                  Wallets
                </Badge>
                <Badge
                  variant={typeFilter.has('contract') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleTypeFilter('contract')}
                >
                  Contracts
                </Badge>
                <Badge
                  variant={typeFilter.has('exchange') ? 'default' : 'outline'}
                  className="cursor-pointer"
                  onClick={() => toggleTypeFilter('exchange')}
                >
                  Exchanges
                </Badge>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Search Results */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              <span>Results</span>
              <Badge variant="outline">{filteredResults.length} entities found</Badge>
            </CardTitle>
          </CardHeader>
          <CardContent>
            <ScrollArea className="h-[400px]">
              <div className="space-y-3">
                {isSearching ? (
                  <div className="flex items-center justify-center py-8">
                    <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
                  </div>
                ) : filteredResults.length === 0 ? (
                  <div className="text-center py-8 text-muted-foreground">
                    <Database className="h-12 w-12 mx-auto mb-2 opacity-50" />
                    <p>No entities found matching your search</p>
                  </div>
                ) : (
                  filteredResults.map((result) => (
                    <div
                      key={result.id}
                      className={cn(
                        "flex items-center justify-between rounded-lg border p-4 hover:bg-muted/50 transition-colors cursor-pointer",
                        selectedResult?.id === result.id && "border-primary bg-primary/5"
                      )}
                      onClick={() => handleViewEntity(result)}
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
                        <ArrowRight className="h-4 w-4 text-muted-foreground" />
                      </div>
                    </div>
                  ))
                )}
              </div>
            </ScrollArea>
          </CardContent>
        </Card>

        {/* Entity Details Panel */}
        <Card>
          <CardHeader>
            <CardTitle>Entity Details</CardTitle>
            <CardDescription>
              {selectedResult ? 'Selected entity information' : 'Select an entity to view details'}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {selectedResult ? (
              <div className="space-y-4">
                <div>
                  <p className="text-xs text-muted-foreground">Address</p>
                  <code className="text-sm font-address break-all">{selectedResult.address}</code>
                </div>

                <Separator />

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-xs text-muted-foreground">Type</p>
                    <Badge className={cn('mt-1', getTypeColor(selectedResult.type))}>
                      {selectedResult.type}
                    </Badge>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">Resonance</p>
                    <p className={cn('text-xl font-bold', getResonanceColor(selectedResult.resonance))}>
                      {formatResonance(selectedResult.resonance)}
                    </p>
                  </div>
                </div>

                {selectedResult.cluster && (
                  <div>
                    <p className="text-xs text-muted-foreground">Cluster</p>
                    <Badge variant="secondary" className="mt-1">{selectedResult.cluster}</Badge>
                  </div>
                )}

                {selectedResult.transactions && (
                  <div>
                    <p className="text-xs text-muted-foreground">Transactions</p>
                    <p className="text-lg font-semibold">{selectedResult.transactions.toLocaleString()}</p>
                  </div>
                )}

                {selectedResult.balance && (
                  <div>
                    <p className="text-xs text-muted-foreground">Balance</p>
                    <p className="text-lg font-semibold">{selectedResult.balance}</p>
                  </div>
                )}

                <Separator />

                <div className="space-y-2">
                  <Button className="w-full" onClick={() => handleViewInTopology(selectedResult)}>
                    <Network className="mr-2 h-4 w-4" />
                    View in Topology
                  </Button>
                  <Button variant="outline" className="w-full">
                    <ExternalLink className="mr-2 h-4 w-4" />
                    Open in Etherscan
                  </Button>
                </div>
              </div>
            ) : (
              <div className="py-8 text-center text-sm text-muted-foreground">
                <Search className="h-12 w-12 mx-auto mb-2 opacity-50" />
                <p>Click on a result to view entity details</p>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
