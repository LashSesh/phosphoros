import { useState, useEffect } from 'react'
import {
  Key, Plus, Copy, Eye, EyeOff, Bitcoin, Coins, Trash2,
  Loader2, AlertCircle, CheckCircle2, Network
} from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { Label } from '@/components/ui/label'
import { Checkbox } from '@/components/ui/checkbox'
import { formatAddress } from '@/lib/utils'
import {
  useImportMnemonic,
  useDeriveAddresses,
  useWalletList,
  useRemoveWallet,
} from '@/hooks/useApi'
import { useWebSocket } from '@/hooks/useWebSocket'
import type { WalletInfo } from '@/hooks/useApi'

// Supported blockchains with display info
const BLOCKCHAINS = [
  { id: 'bitcoin', name: 'Bitcoin', icon: Bitcoin, color: 'text-amber-500' },
  { id: 'ethereum', name: 'Ethereum', icon: Coins, color: 'text-blue-500' },
  { id: 'polkadot', name: 'Polkadot', icon: Network, color: 'text-pink-500' },
  { id: 'kusama', name: 'Kusama', icon: Network, color: 'text-purple-500' },
  { id: 'cosmos', name: 'Cosmos', icon: Network, color: 'text-indigo-500' },
  { id: 'solana', name: 'Solana', icon: Network, color: 'text-teal-500' },
  { id: 'cardano', name: 'Cardano', icon: Network, color: 'text-cyan-500' },
] as const

export function WalletPage() {
  // Form state
  const [mnemonic, setMnemonic] = useState('')
  const [showMnemonic, setShowMnemonic] = useState(false)
  const [label, setLabel] = useState('')
  const [selectedBlockchains, setSelectedBlockchains] = useState<string[]>(['bitcoin', 'ethereum'])
  const [addressStart, setAddressStart] = useState(0)
  const [addressEnd, setAddressEnd] = useState(4)

  // API hooks
  const { mutate: importMnemonic, isPending: isImporting, error: importError } = useImportMnemonic()
  const { mutate: deriveAddresses, isPending: isDeriving, error: deriveError } = useDeriveAddresses()
  const { data: walletListData, isLoading: isLoadingWallets, refetch: refetchWallets } = useWalletList()
  const { mutate: removeWallet, isPending: isRemoving } = useRemoveWallet()

  // WebSocket for real-time updates
  const [lastDerivedEvent, setLastDerivedEvent] = useState<{
    blockchain: string
    count: number
    timestamp: string
  } | null>(null)

  useWebSocket({
    onEvent: (event) => {
      if (event.type === 'WalletDerived') {
        setLastDerivedEvent({
          blockchain: event.blockchain,
          count: event.count,
          timestamp: event.timestamp,
        })
        // Refresh wallet list after derivation
        refetchWallets()
      }
    },
  })

  // Clear last derived event after 5 seconds
  useEffect(() => {
    if (lastDerivedEvent) {
      const timeout = setTimeout(() => setLastDerivedEvent(null), 5000)
      return () => clearTimeout(timeout)
    }
  }, [lastDerivedEvent])

  const handleToggleBlockchain = (blockchain: string) => {
    setSelectedBlockchains((prev) =>
      prev.includes(blockchain)
        ? prev.filter((b) => b !== blockchain)
        : [...prev, blockchain]
    )
  }

  const handleImport = () => {
    if (!mnemonic.trim() || !label.trim()) return

    const words = mnemonic.trim().split(/\s+/)
    if (words.length !== 12 && words.length !== 24) {
      alert('Please enter a valid 12 or 24 word mnemonic')
      return
    }

    importMnemonic(
      {
        phrase: mnemonic.trim(),
        language: 'english',
        label: label.trim(),
      },
      {
        onSuccess: (response) => {
          if (response.success) {
            setMnemonic('')
            setLabel('')
            refetchWallets()
          } else {
            alert(`Import failed: ${response.error}`)
          }
        },
        onError: (error) => {
          alert(`Import error: ${error.message}`)
        },
      }
    )
  }

  const handleDerive = () => {
    if (!mnemonic.trim() || selectedBlockchains.length === 0) return

    const words = mnemonic.trim().split(/\s+/)
    if (words.length !== 12 && words.length !== 24) {
      alert('Please enter a valid 12 or 24 word mnemonic')
      return
    }

    if (addressStart < 0 || addressEnd <= addressStart) {
      alert('Invalid address range')
      return
    }

    deriveAddresses(
      {
        phrase: mnemonic.trim(),
        blockchains: selectedBlockchains,
        account: 0,
        address_range: { start: addressStart, end: addressEnd },
        label: label.trim() || undefined,
      },
      {
        onSuccess: (response) => {
          if (response.success) {
            // Success notification handled via WebSocket
            refetchWallets()
          } else {
            alert(`Derivation failed: ${response.error}`)
          }
        },
        onError: (error) => {
          alert(`Derivation error: ${error.message}`)
        },
      }
    )
  }

  const handleRemoveWallet = (walletLabel: string) => {
    if (!confirm(`Delete wallet "${walletLabel}"?`)) return

    removeWallet(
      { label: walletLabel },
      {
        onSuccess: () => {
          refetchWallets()
        },
        onError: (error) => {
          alert(`Delete error: ${error.message}`)
        },
      }
    )
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
  }

  const wallets = walletListData?.wallets || []

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Wallet & Seeds</h1>
        <p className="text-muted-foreground">
          Import BIP39 mnemonics and derive addresses for 7 blockchains via PHOSPHOROS Gateway
        </p>
      </div>

      {/* WebSocket Event Notification */}
      {lastDerivedEvent && (
        <Card className="border-green-500 bg-green-50 dark:bg-green-950">
          <CardContent className="pt-6">
            <div className="flex items-center gap-2 text-green-700 dark:text-green-300">
              <CheckCircle2 className="h-5 w-5" />
              <span className="font-medium">
                Derived {lastDerivedEvent.count} {lastDerivedEvent.blockchain} address
                {lastDerivedEvent.count > 1 ? 'es' : ''}
              </span>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Import & Derive Section */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Plus className="h-5 w-5" />
            Derive Addresses from Mnemonic
          </CardTitle>
          <CardDescription>
            Enter a BIP39 mnemonic to derive addresses across multiple blockchains
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Mnemonic Input */}
          <div className="space-y-2">
            <Label htmlFor="mnemonic">Mnemonic Phrase (12 or 24 words)</Label>
            <div className="relative">
              <Input
                id="mnemonic"
                type={showMnemonic ? 'text' : 'password'}
                placeholder="abandon abandon abandon ..."
                value={mnemonic}
                onChange={(e) => setMnemonic(e.target.value)}
                className="pr-10 font-mono text-sm"
              />
              <Button
                variant="ghost"
                size="icon"
                className="absolute right-1 top-1/2 -translate-y-1/2"
                onClick={() => setShowMnemonic(!showMnemonic)}
              >
                {showMnemonic ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
              </Button>
            </div>
          </div>

          {/* Label Input */}
          <div className="space-y-2">
            <Label htmlFor="label">Wallet Label (optional)</Label>
            <Input
              id="label"
              type="text"
              placeholder="My Trading Wallet"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
            />
          </div>

          {/* Blockchain Selection */}
          <div className="space-y-3">
            <Label>Select Blockchains</Label>
            <div className="grid grid-cols-2 sm:grid-cols-3 gap-3">
              {BLOCKCHAINS.map((blockchain) => {
                const Icon = blockchain.icon
                const isSelected = selectedBlockchains.includes(blockchain.id)
                return (
                  <div
                    key={blockchain.id}
                    className={`flex items-center gap-2 rounded-md border p-3 cursor-pointer transition-colors ${
                      isSelected
                        ? 'border-primary bg-primary/5'
                        : 'border-muted hover:border-primary/50'
                    }`}
                    onClick={() => handleToggleBlockchain(blockchain.id)}
                  >
                    <Checkbox
                      checked={isSelected}
                      onCheckedChange={() => handleToggleBlockchain(blockchain.id)}
                    />
                    <Icon className={`h-4 w-4 ${blockchain.color}`} />
                    <span className="text-sm font-medium">{blockchain.name}</span>
                  </div>
                )
              })}
            </div>
          </div>

          {/* Address Range */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="start">Start Index</Label>
              <Input
                id="start"
                type="number"
                min="0"
                value={addressStart}
                onChange={(e) => setAddressStart(parseInt(e.target.value) || 0)}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="end">End Index</Label>
              <Input
                id="end"
                type="number"
                min="1"
                value={addressEnd}
                onChange={(e) => setAddressEnd(parseInt(e.target.value) || 1)}
              />
            </div>
          </div>

          <p className="text-xs text-muted-foreground">
            Will derive {Math.max(0, addressEnd - addressStart)} address
            {addressEnd - addressStart !== 1 ? 'es' : ''} per blockchain
          </p>

          {/* Error Display */}
          {(importError || deriveError) && (
            <div className="flex items-start gap-2 rounded-md border border-red-500 bg-red-50 dark:bg-red-950 p-3">
              <AlertCircle className="h-5 w-5 text-red-500 mt-0.5" />
              <div className="flex-1">
                <p className="text-sm font-medium text-red-700 dark:text-red-300">
                  {importError?.message || deriveError?.message}
                </p>
              </div>
            </div>
          )}

          {/* Action Buttons */}
          <div className="flex gap-3">
            <Button
              onClick={handleImport}
              disabled={!mnemonic.trim() || !label.trim() || isImporting}
              variant="outline"
            >
              {isImporting ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Importing...
                </>
              ) : (
                <>
                  <Key className="mr-2 h-4 w-4" />
                  Import Only
                </>
              )}
            </Button>

            <Button
              onClick={handleDerive}
              disabled={
                !mnemonic.trim() ||
                selectedBlockchains.length === 0 ||
                isDeriving ||
                addressEnd <= addressStart
              }
            >
              {isDeriving ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Deriving...
                </>
              ) : (
                <>
                  <Network className="mr-2 h-4 w-4" />
                  Derive Addresses
                </>
              )}
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Wallet List */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Stored Wallets ({wallets.length})</CardTitle>
            {isLoadingWallets && <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />}
          </div>
          <CardDescription>
            Addresses derived from imported mnemonics (stored in Gateway memory)
          </CardDescription>
        </CardHeader>
        <CardContent>
          {wallets.length === 0 ? (
            <div className="text-center py-12 text-muted-foreground">
              <Key className="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p>No wallets imported yet</p>
              <p className="text-sm mt-2">Use the form above to import a mnemonic and derive addresses</p>
            </div>
          ) : (
            <ScrollArea className="h-[500px]">
              <div className="space-y-4">
                {wallets.map((wallet, idx) => {
                  const blockchain = BLOCKCHAINS.find((b) => b.id === wallet.blockchain.toLowerCase())
                  const Icon = blockchain?.icon || Network
                  const color = blockchain?.color || 'text-gray-500'

                  return (
                    <div key={`${wallet.label}-${wallet.blockchain}-${idx}`} className="rounded-lg border p-4 space-y-4">
                      {/* Header */}
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <Icon className={`h-5 w-5 ${color}`} />
                          <div>
                            <h3 className="font-medium">{wallet.label || 'Unlabeled Wallet'}</h3>
                            <p className="text-xs text-muted-foreground">
                              {wallet.blockchain} • {wallet.addresses.length} address
                              {wallet.addresses.length !== 1 ? 'es' : ''}
                            </p>
                          </div>
                        </div>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => handleRemoveWallet(wallet.label)}
                          disabled={isRemoving}
                        >
                          <Trash2 className="h-4 w-4 text-red-500" />
                        </Button>
                      </div>

                      <Separator />

                      {/* Addresses */}
                      <div className="space-y-2">
                        {wallet.addresses.map((address, addrIdx) => (
                          <div
                            key={`${address}-${addrIdx}`}
                            className="flex items-center gap-2 rounded-md bg-muted/50 p-2"
                          >
                            <Badge variant="outline" className="text-xs">
                              #{addrIdx}
                            </Badge>
                            <code className="text-xs font-address flex-1 truncate">
                              {address}
                            </code>
                            <Button
                              variant="ghost"
                              size="icon"
                              className="h-6 w-6 flex-shrink-0"
                              onClick={() => copyToClipboard(address)}
                            >
                              <Copy className="h-3 w-3" />
                            </Button>
                          </div>
                        ))}
                      </div>

                      {/* Import timestamp */}
                      <p className="text-xs text-muted-foreground">
                        Imported {new Date(wallet.imported_at).toLocaleString()}
                      </p>
                    </div>
                  )
                })}
              </div>
            </ScrollArea>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
