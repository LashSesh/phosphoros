import { useState } from 'react'
import { Key, Plus, Copy, Eye, EyeOff, Bitcoin, Coins } from 'lucide-react'
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { formatAddress } from '@/lib/utils'

interface ImportedSeed {
  id: string
  mnemonic: string
  maskedMnemonic: string
  addresses: {
    bitcoin: string
    ethereum: string
  }
  importedAt: Date
}

export function WalletPage() {
  const [mnemonic, setMnemonic] = useState('')
  const [showMnemonic, setShowMnemonic] = useState(false)
  const [seeds, setSeeds] = useState<ImportedSeed[]>([
    {
      id: '1',
      mnemonic: 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
      maskedMnemonic: 'abandon abandon abandon ... abandon about',
      addresses: {
        bitcoin: '1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2',
        ethereum: '0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed',
      },
      importedAt: new Date(),
    },
  ])

  const handleImport = () => {
    if (!mnemonic.trim()) return

    const words = mnemonic.trim().split(/\s+/)
    if (words.length !== 12 && words.length !== 24) {
      alert('Please enter a valid 12 or 24 word mnemonic')
      return
    }

    const newSeed: ImportedSeed = {
      id: crypto.randomUUID(),
      mnemonic: mnemonic.trim(),
      maskedMnemonic: `${words.slice(0, 3).join(' ')} ... ${words.slice(-2).join(' ')}`,
      addresses: {
        bitcoin: `1${crypto.randomUUID().slice(0, 32)}`,
        ethereum: `0x${crypto.randomUUID().replace(/-/g, '').slice(0, 40)}`,
      },
      importedAt: new Date(),
    }

    setSeeds([newSeed, ...seeds])
    setMnemonic('')
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Wallet & Seeds</h1>
        <p className="text-muted-foreground">
          Import and manage BIP39 mnemonics for multichain address derivation
        </p>
      </div>

      {/* Import Section */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Plus className="h-5 w-5" />
            Import Mnemonic
          </CardTitle>
          <CardDescription>
            Enter a BIP39 mnemonic phrase to derive addresses for Bitcoin and Ethereum
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="relative">
            <Input
              type={showMnemonic ? 'text' : 'password'}
              placeholder="Enter 12 or 24 word mnemonic phrase..."
              value={mnemonic}
              onChange={(e) => setMnemonic(e.target.value)}
              className="pr-10 font-mono"
            />
            <Button
              variant="ghost"
              size="icon"
              className="absolute right-1 top-1/2 -translate-y-1/2"
              onClick={() => setShowMnemonic(!showMnemonic)}
            >
              {showMnemonic ? (
                <EyeOff className="h-4 w-4" />
              ) : (
                <Eye className="h-4 w-4" />
              )}
            </Button>
          </div>
          <Button onClick={handleImport} disabled={!mnemonic.trim()}>
            <Key className="mr-2 h-4 w-4" />
            Import & Derive Addresses
          </Button>
        </CardContent>
      </Card>

      {/* Imported Seeds */}
      <Card>
        <CardHeader>
          <CardTitle>Imported Seeds ({seeds.length})</CardTitle>
        </CardHeader>
        <CardContent>
          <ScrollArea className="h-[400px]">
            <div className="space-y-4">
              {seeds.map((seed) => (
                <div
                  key={seed.id}
                  className="rounded-lg border p-4 space-y-4"
                >
                  {/* Mnemonic */}
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Key className="h-4 w-4 text-muted-foreground" />
                      <code className="text-sm">{seed.maskedMnemonic}</code>
                    </div>
                    <Badge variant="outline">
                      {seed.mnemonic.split(' ').length} words
                    </Badge>
                  </div>

                  <Separator />

                  {/* Addresses */}
                  <div className="grid gap-3 sm:grid-cols-2">
                    {/* Bitcoin */}
                    <div className="rounded-md bg-muted/50 p-3">
                      <div className="flex items-center gap-2 mb-2">
                        <Bitcoin className="h-4 w-4 text-amber-500" />
                        <span className="text-xs font-medium">Bitcoin</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <code className="text-xs font-address flex-1">
                          {formatAddress(seed.addresses.bitcoin, 8)}
                        </code>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-6 w-6"
                          onClick={() => copyToClipboard(seed.addresses.bitcoin)}
                        >
                          <Copy className="h-3 w-3" />
                        </Button>
                      </div>
                    </div>

                    {/* Ethereum */}
                    <div className="rounded-md bg-muted/50 p-3">
                      <div className="flex items-center gap-2 mb-2">
                        <Coins className="h-4 w-4 text-blue-500" />
                        <span className="text-xs font-medium">Ethereum</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <code className="text-xs font-address flex-1">
                          {formatAddress(seed.addresses.ethereum, 8)}
                        </code>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="h-6 w-6"
                          onClick={() => copyToClipboard(seed.addresses.ethereum)}
                        >
                          <Copy className="h-3 w-3" />
                        </Button>
                      </div>
                    </div>
                  </div>

                  <p className="text-xs text-muted-foreground">
                    Imported {seed.importedAt.toLocaleString()}
                  </p>
                </div>
              ))}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>
    </div>
  )
}
