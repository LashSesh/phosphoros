# PHOSPHOROS Web GUI - Umfassende Funktionsdokumentation

<div align="center">

**Detaillierte Beschreibung aller GUI-Funktionen**

*React 18 Web Dashboard für Blockchain-Forensik*

[Übersicht](#übersicht) • [Hauptfunktionen](#hauptfunktionen) • [Navigation](#navigation) • [Seiten](#seiten) • [Komponenten](#komponenten)

</div>

---

## Übersicht

Die PHOSPHOROS Web-Oberfläche ist eine **moderne, React-basierte Single-Page-Application (SPA)**, die als primäre Benutzeroberfläche für alle Blockchain-Forensik- und Analyse-Funktionen dient. Sie bietet:

- ✅ **10 spezialisierte Seiten** für verschiedene Analyse-Workflows
- ✅ **Real-time Updates** via WebSocket (8 Event-Typen)
- ✅ **Interactive Visualizations** mit D3.js und ECharts
- ✅ **Type-safe Integration** mit TypeScript + Rust Backend
- ✅ **Responsive Design** optimiert für Desktop & Tablet
- ✅ **Dark/Light Theme** mit System-aware Auto-Switching

**Technologie-Stack:**
- React 18.3 + TypeScript 5.6
- TailwindCSS 3.4 + ShadcnUI Components
- Zustand 5.0 (State Management)
- React Query 5.x (Server State & Caching)
- D3.js 7.9 (Network Graphs)
- ECharts 5.5 (Charts & Gauges)
- React Router DOM 6.28 (Routing)

---

## Navigation & Layout

### App Shell (Hauptcontainer)

Die gesamte Anwendung ist in einem **4-Bereichs-Layout** organisiert:

```
┌─────────────────────────────────────────┐
│         Header (Kopfzeile)              │
├──────┬──────────────────────────────────┤
│      │                                  │
│  S   │      Content Area                │
│  i   │      (Hauptinhalt)               │
│  d   │                                  │
│  e   │                                  │
│  b   │                                  │
│  a   │                                  │
│  r   │                                  │
│      │                                  │
├──────┴──────────────────────────────────┤
│      Status Bar (Fußzeile)              │
└─────────────────────────────────────────┘
```

#### Header (Kopfzeile)

**Position:** Oben, fest fixiert

**Komponenten:**
- **Logo & Branding**: PHOSPHOROS-Logo mit Animation
- **Theme Toggle**: Wechsel zwischen Dark/Light Mode
  - Klick auf Mond/Sonne-Icon
  - Wird in localStorage gespeichert
  - System-Präferenz wird respektiert
- **Quick Actions**:
  - Benachrichtigungs-Icon (zukünftig)
  - User-Profil (zukünftig)
- **Command Palette Trigger**: ⌘+K / Ctrl+K Shortcut-Anzeige

#### Sidebar (Seitenleiste)

**Position:** Links, kollapsierbar

**Navigation:**
- **Dashboard** (Home-Icon): Übersicht & Metriken
- **Wallet** (Key-Icon): Seed & Adress-Verwaltung
- **Resonance** (Zap-Icon): 5D Spektral-Analyse
- **Cluster** (Users-Icon): Entity-Clustering
- **Topology** (Network-Icon): Netzwerk-Visualisierung
- **Explorer** (Search-Icon): Entity-Suche
- **Anomalies** (Alert-Icon): Anomalie-Management
- **Forensics** (Shield-Icon): Forensische Workflows
- **Infogenetik** (Brain-Icon): Spezialisierte Analyse
- **Settings** (Gear-Icon): Konfiguration

**Funktionen:**
- **Collapse/Expand**: Sidebar ein-/ausklappen (Hamburger-Icon)
- **Active State**: Aktive Seite wird hervorgehoben
- **Tooltips**: Bei kollabierter Sidebar werden Tooltips angezeigt
- **Keyboard Navigation**: Tab-Navigation durch Menüpunkte

#### Status Bar (Fußzeile)

**Position:** Unten, fest fixiert

**Anzeigen:**
- **Service Status**: Scraper, Analyzer, Cluster Engine
  - Online (grün)
  - Pending (gelb)
  - Offline (grau)
- **Processed Items**: Anzahl verarbeiteter Einträge pro Service
- **WebSocket Status**: Verbindungsstatus (Connected/Disconnected)
- **Last Update**: Timestamp des letzten Updates

#### Command Palette (⌘+K)

**Aktivierung:**
- Tastenkombination: `⌘+K` (Mac) / `Ctrl+K` (Windows/Linux)
- Klick auf Search-Icon in Header

**Funktionen:**
- **Global Search**: Suche durch alle verfügbaren Aktionen
- **Quick Navigation**: Schnellzugriff auf alle Seiten
- **Recent Actions**: Historie der letzten Aktionen
- **Keyboard-First**: Vollständig über Tastatur bedienbar
  - `↑/↓`: Navigation
  - `Enter`: Auswahl
  - `Esc`: Schließen

**Verfügbare Kommandos:**
- `Go to Dashboard`
- `Go to Wallet`
- `Go to Resonance`
- `Import Mnemonic`
- `Run Analysis`
- `Export Report`
- `Toggle Theme`
- (weitere werden dynamisch hinzugefügt)

---

## Seiten (Detailliert)

### 1. Dashboard (Home)

**Route:** `/`

**Zweck:** Zentrale Übersicht aller System-Metriken und Aktivitäten

#### Komponenten

##### Metric Cards (4 Karten)

**Seeds Collected**
- **Anzeige**: Anzahl importierter Mnemonics
- **Icon**: Schlüssel (Key)
- **Trend**: Prozentuale Veränderung (↑ 12%)
- **Farbe**: Grün bei positiv, Rot bei negativ

**Clusters Discovered**
- **Anzeige**: Anzahl berechneter Cluster
- **Icon**: Blitz (Zap)
- **Trend**: Prozentuale Veränderung (↑ 8%)
- **Farbe**: Grün bei positiv, Rot bei negativ

**Entities Tracked**
- **Anzeige**: Gesamtanzahl verfolgter Entities
- **Icon**: Datenbank (Database)
- **Trend**: Optional
- **Farbe**: Neutral (blau)

**Anomalies Detected**
- **Anzeige**: Anzahl erkannter Anomalien
- **Icon**: Warnung (AlertTriangle)
- **Trend**: Prozentuale Veränderung (↓ 3%)
- **Farbe**: Rot bei positiv (mehr Anomalien), Grün bei negativ

##### Service Status

**Anzeige für jeden Service:**
- **Scraper**:
  - Status-Badge: online/pending/offline
  - Processed Count: Anzahl gescannter Transaktionen
  - Icon: Database

- **Analyzer**:
  - Status-Badge: online/pending/offline
  - Processed Count: Anzahl analysierter Snapshots
  - Icon: Activity

- **Cluster Engine**:
  - Status-Badge: online/pending/offline
  - Processed Count: Anzahl berechneter Cluster
  - Icon: Zap

**Interaktivität:**
- Klick auf Service öffnet Detail-Modal (zukünftig)
- Farbcodierung: Grün (online), Gelb (pending), Grau (offline)

##### Activity Feed

**Funktionen:**
- **Live-Updates**: WebSocket-basierte Echtzeit-Benachrichtigungen
- **Kategorien**:
  - 🔑 **Entity**: Neue Wallet-Entdeckungen
  - ⚡ **Cluster**: Cluster-Formationen
  - ⚠️ **Anomaly**: Anomalie-Detektionen
  - 📊 **Analysis**: Analyse-Fortschritt

**Anzeige:**
- **Icon**: Kategorie-spezifisch farbcodiert
- **Message**: Beschreibungstext
- **Timestamp**: Relative Zeit ("2 minutes ago")
- **Auto-Scroll**: Neueste Einträge oben
- **Limit**: Maximale 50 Einträge (älteste werden entfernt)

**Beispiel-Einträge:**
```
🔑 Discovered new wallet 0x7a25...3f21 (2 minutes ago)
⚡ Formed cluster with 12 members (5 minutes ago)
⚠️ High-value transfer detected (8 minutes ago)
📊 Analysis complete (10 minutes ago)
```

##### Quick Actions (4 Buttons)

**Import Mnemonic**
- **Icon**: Plus
- **Aktion**: Navigiert zu `/wallet`
- **Beschreibung**: Schnelles Importieren von BIP-39 Seeds

**Start Analysis**
- **Icon**: Activity
- **Aktion**: Navigiert zu `/resonance`
- **Beschreibung**: Starten einer Resonanz-Analyse

**View Clusters**
- **Icon**: Zap
- **Aktion**: Navigiert zu `/topology`
- **Beschreibung**: Cluster-Visualisierung öffnen

**Export Data**
- **Icon**: Download
- **Aktion**: Triggert JSON-Export
- **Beschreibung**: Exportiert vollständigen Report

##### Header-Aktionen

**Export Report**
- **Button**: "Export Report" (mit Download-Icon)
- **Funktion**: Erstellt JSON-Report
- **Inhalt**:
  - Timestamp
  - Alle Metriken (seeds, clusters, entities, anomalies, analyses, hotspots)
  - Service-Status (scraper, analyzer, cluster)
  - Letzte 20 Activity-Feed-Einträge
- **Download**: `phosphoros-report-YYYY-MM-DD.json`
- **Feedback**: Success-Nachricht im Activity Feed

**Run Analysis**
- **Button**: "Run Analysis" (mit Play-Icon)
- **Funktion**: Simuliert forensische Analyse
- **Ablauf**:
  1. Start-Nachricht im Activity Feed
  2. Progress-Simulation (0-100%)
  3. Scannt 1,247 Transaktionen (simuliert)
  4. Identifiziert 1-3 neue Cluster (zufällig)
  5. Erkennt ggf. Anomalie (60% Chance)
  6. Completion-Nachricht
- **Dauer**: ~3 Sekunden
- **Loading State**: Button zeigt Spinner während Ausführung

---

### 2. Wallet & Seeds

**Route:** `/wallet`

**Zweck:** BIP-39 Mnemonic Import und Multi-Chain Address Derivation

#### Hauptfunktionen

##### Derive Addresses from Mnemonic

**Mnemonic-Eingabe:**
- **Feld**: Password-Input (toggle-bar mit Eye-Icon)
- **Format**: 12 oder 24 Wörter (space-separated)
- **Validierung**:
  - Wortanzahl-Check (12 oder 24)
  - BIP-39 Wordlist-Validierung (backend)
- **Placeholder**: "abandon abandon abandon ..."
- **Font**: Monospace für bessere Lesbarkeit

**Label-Eingabe:**
- **Feld**: Text-Input
- **Optional**: Für "Import Only"
- **Required**: Für "Derive Addresses"
- **Placeholder**: "My Trading Wallet"
- **Verwendung**: Gruppierung von abgeleiteten Adressen

**Blockchain-Auswahl:**

Verfügbare Blockchains (7):

| Blockchain | Icon | Farbe | Status |
|------------|------|-------|--------|
| **Bitcoin** | Bitcoin | Amber | Production |
| **Ethereum** | Coins | Blau | Production |
| **Polkadot** | Network | Pink | Production |
| **Kusama** | Network | Lila | Production |
| **Cosmos** | Network | Indigo | Production |
| **Solana** | Network | Teal | Production |
| **Cardano** | Network | Cyan | Production |

**Auswahl-Mechanismus:**
- **Multi-Select**: Checkboxes für jede Blockchain
- **Visual Feedback**: Border + Background-Farbe bei Selektion
- **Default**: Bitcoin + Ethereum vorausgewählt
- **Minimum**: Mindestens 1 Blockchain erforderlich

**Address Range:**

- **Start Index**:
  - Input-Type: Number
  - Min: 0
  - Default: 0
  - Beschreibung: BIP-44 Address Index Start

- **End Index**:
  - Input-Type: Number
  - Min: 1
  - Default: 4
  - Beschreibung: BIP-44 Address Index End (exklusiv)

**Berechnungs-Anzeige:**
- "Will derive **X** address(es) per blockchain"
- Berechnung: `max(0, end - start)`
- Plural-Handling: "address" vs. "addresses"

**Aktions-Buttons:**

1. **Import Only**:
   - **Icon**: Key
   - **Funktion**: Importiert Mnemonic ohne Adress-Derivation
   - **API**: `POST /wallet/import`
   - **Erforderlich**: Mnemonic + Label
   - **Verwendung**: Für spätere Derivation

2. **Derive Addresses**:
   - **Icon**: Network
   - **Funktion**: Leitet Adressen ab für alle selektierten Blockchains
   - **API**: `POST /wallet/derive`
   - **Erforderlich**: Mnemonic + mindestens 1 Blockchain + gültiger Range
   - **Parameter**:
     - `phrase`: Mnemonic
     - `blockchains`: Array von Blockchain-IDs
     - `account`: 0 (BIP-44 Account)
     - `address_range`: { start, end }
     - `label`: Optional

**Loading States:**
- **Während Import**: Button zeigt Spinner + "Importing..."
- **Während Derivation**: Button zeigt Spinner + "Deriving..."
- **Disabled**: Buttons deaktiviert wenn Validierung fehlschlägt

**Fehlerbehandlung:**

Anzeige in roter Alert-Box:
- **Import-Fehler**: z.B. "Invalid mnemonic phrase"
- **Derivation-Fehler**: z.B. "Blockchain not supported"
- **Netzwerk-Fehler**: z.B. "Connection to gateway failed"

**WebSocket Real-time Notification:**

Bei erfolgreicher Derivation erscheint grüne Notification-Box:
- **Icon**: CheckCircle
- **Message**: "Derived X bitcoin address(es)"
- **Auto-Dismiss**: Nach 5 Sekunden
- **Trigger**: WebSocket Event `WalletDerived`

##### Stored Wallets

**Liste aller importierten Wallets:**

**Header:**
- **Title**: "Stored Wallets (X)" mit Anzahl
- **Description**: "Addresses derived from imported mnemonics (stored in Gateway memory)"
- **Loading Indicator**: Spinner während API-Call

**Empty State:**
- **Icon**: Großes Key-Icon (opacity 50%)
- **Message**: "No wallets imported yet"
- **Hint**: "Use the form above to import a mnemonic and derive addresses"

**Wallet-Karten:**

Jede Wallet-Karte zeigt:

**Header:**
- **Icon**: Blockchain-spezifisch farbcodiert
- **Label**: Wallet-Name oder "Unlabeled Wallet"
- **Metadata**: "Bitcoin • 4 addresses"
- **Delete Button**: Trash-Icon (rot) mit Bestätigungs-Dialog

**Address-Liste:**
- **Badge**: Index-Nummer (#0, #1, #2, ...)
- **Address**: Monospace-Font, truncated
- **Copy Button**: Kopiert Adresse in Clipboard
- **Layout**: Scrollbar bei vielen Adressen

**Footer:**
- **Import Timestamp**: "Imported 2026-01-03 14:23:45"

**Beispiel-Anzeige:**
```
┌─────────────────────────────────────────┐
│ 🟡 Bitcoin  My Trading Wallet    🗑️    │
│ Bitcoin • 4 addresses                   │
├─────────────────────────────────────────┤
│ #0 bc1q...xyz  📋                      │
│ #1 bc1q...abc  📋                      │
│ #2 bc1q...def  📋                      │
│ #3 bc1q...ghi  📋                      │
├─────────────────────────────────────────┤
│ Imported 2026-01-03 14:23:45            │
└─────────────────────────────────────────┘
```

**Interaktionen:**
- **Copy Address**: Klick auf Kopier-Button → Clipboard
- **Delete Wallet**: Klick auf Trash → Bestätigungs-Dialog → API-Call → Refresh
- **Scroll**: Scrollarea mit max-height 500px

---

### 3. Resonance Analysis

**Route:** `/resonance`

**Zweck:** 5D Spektral-Analyse und Resonanz-Pattern-Detection

#### Hauptkomponenten

##### Page Header

**Titel**: "Resonance Analysis"
**Beschreibung**: "5D spectral analysis and resonance pattern detection"

**Aktions-Buttons:**

1. **Start/Stop Analysis**:
   - **Icon**: Play (gestartet) / Pause (gestoppt)
   - **Label**: "Start Analysis" / "Stop Analysis"
   - **Funktion**: Toggle Live-Analyse-Modus
   - **Effekt**: Startet/stoppt Echtzeit-Updates der Spektral-Werte

2. **Compute Spectral**:
   - **Icon**: Zap
   - **Funktion**: API-Call zur Berechnung spektraler Metriken
   - **API**: `POST /resonance/spectral`
   - **Parameter**: Current `psi`, `rho`, `omega`
   - **Response**: Validierte Spektral-Signatur

##### Progress & Status

**Wird angezeigt während Live-Analyse:**

- **Icon**: Activity (animiert, pulsierend)
- **Message**: "Analyzing spectral patterns..."
- **Progress Bar**: 0-100% mit Smooth Transition
- **Farbe**: Primary (blau)
- **Auto-Stop**: Bei 100% wird Analyse automatisch gestoppt

##### Resonance Gauge (Gauge-Chart)

**Visualisierung:**
- **Typ**: ECharts Gauge
- **Wert**: Kombinierte Resonanz D = ψ·ρ·ω
- **Range**: 0.0 - 1.0
- **Farb-Gradient**:
  - 0.0-0.3: Rot (Low Resonance)
  - 0.3-0.6: Gelb (Medium Resonance)
  - 0.6-0.8: Grün (Good Resonance)
  - 0.8-1.0: Blau (Excellent Resonance)
- **Animationen**: Smooth Value Transitions (500ms)

##### Spectral Signature (ψ, ρ, ω)

**Anzeige für jedes Triplett-Element:**

**ψ (Psi) - Coherence**
- **Label**: "ψ (Coherence)"
- **Wert**: Dezimal 4 Stellen (z.B. 0.8500)
- **Farbe**: Cyan (#00D9FF)
- **Progress Bar**: Horizontal, gefüllt entsprechend Wert
- **Update**: Real-time während Analyse

**ρ (Rho) - Stability**
- **Label**: "ρ (Stability)"
- **Wert**: Dezimal 4 Stellen (z.B. 0.9200)
- **Farbe**: Blau (#0099FF)
- **Progress Bar**: Horizontal, gefüllt entsprechend Wert
- **Update**: Real-time während Analyse

**ω (Omega) - Efficiency**
- **Label**: "ω (Efficiency)"
- **Wert**: Dezimal 4 Stellen (z.B. 0.7800)
- **Farbe**: Amber (#FFAA00)
- **Progress Bar**: Horizontal, gefüllt entsprechend Wert
- **Update**: Real-time während Analyse

**Kombinierte Resonanz:**
- **Label**: "ψ × ρ × ω"
- **Berechnung**: Multiplikation der drei Werte
- **Anzeige**: Groß, 2xl Font, Monospace
- **Farbe**: Dynamisch basierend auf Wert:
  - < 0.3: Rot
  - 0.3-0.6: Gelb
  - 0.6-0.8: Grün
  - > 0.8: Blau

**Evaluate Resonance Button:**
- **Icon**: Zap
- **Funktion**: Sendet aktuelle Spektral-Signatur an Holistic Matrix
- **API**: `POST /resonance/analyze`
- **Parameter**:
  - `t`: Timestamp (seconds)
  - `perception`: 5D Vector aus Signatur + Operators
  - `intention`: 5D Vector aus Operators + Signatur
  - `gradient`: 5D Vector aus Differenzen
  - `theta`: Phase Angle (resonance × π)
  - `label`: Timestamp-basiert
- **Response**: `Output` (Score + Vector) oder `Gated` (Reason)

##### Operator Configuration

**4 Topologische Operatoren:**

**WT (Wave/Wormhole Funnel)**
- **Label**: "WT (Wave)"
- **Input**: Number (0-2, Step 0.1)
- **Default**: 1.0
- **Effekt**: Beeinflusst ψ-Variationen (× 0.05)

**SW (Spectral/Threshold Sweep)**
- **Label**: "SW (Spectral)"
- **Input**: Number (0-2, Step 0.1)
- **Default**: 0.5
- **Effekt**: Beeinflusst ρ-Variationen (× 0.03)

**DK (Double-Kick)**
- **Label**: "DK (Double-Kick)"
- **Input**: Number (0-2, Step 0.1)
- **Default**: 0.3
- **Effekt**: Zufällige Spikes in ψ (Probability DK × 0.3)

**PI (Phase/Path Invariance)**
- **Label**: "PI (Phase)"
- **Input**: Number (0-2, Step 0.1)
- **Default**: 0.1
- **Effekt**: Glättet ω-Variationen (× 0.02)

**Operator-Kontrollen:**

**Reset Button:**
- **Icon**: RotateCcw
- **Funktion**: Setzt alle Operatoren auf Defaults zurück
- **Werte**: WT=1.0, SW=0.5, DK=0.3, PI=0.1

**Apply Button:**
- **Funktion**: Aktiviert pending Operator-Änderungen
- **Effekt**: Neue Werte werden in Live-Analyse verwendet

**Active Display:**
- **Text**: "Active: WT=1.0 SW=0.5 DK=0.3 PI=0.1"
- **Position**: Unter Kontrollen
- **Farbe**: Muted Foreground

##### Last Evaluation Result

**Wird angezeigt nach "Evaluate Resonance"-Call:**

**Bei Output (Erfolg):**
- **Status**: "Output Produced" (grün)
- **Resonance Score**: Großer Dezimalwert
- **Action Vector**: 5D Array angezeigt als:
  ```
  [0.1234, 0.5678, 0.9012, 0.3456, 0.7890]
  ```
- **Formatting**: Monospace, 4 Dezimalstellen

**Bei Gated (Blockiert):**
- **Status**: "Gated" (rot)
- **Reason**: Textuelle Begründung (z.B. "Coherence below threshold")
- **Keine Vector-Anzeige**

##### Resonance History (Time-Series Chart)

**Visualisierung:**
- **Typ**: ECharts Line Chart
- **X-Achse**: Zeit (HH:mm:ss)
- **Y-Achse**: Wert (0-1)
- **4 Linien**:
  - ψ (Cyan)
  - ρ (Blau)
  - ω (Amber)
  - D (Grün, Combined Resonance)
- **Datenpunkte**: Bis zu 30 letzte Einträge
- **Update**: Neue Punkte werden hinten angehängt
- **Smooth**: Kurven mit Smooth-Parameter

**Interaktionen:**
- **Hover**: Tooltip mit genauen Werten
- **Zoom**: Mausrad zum Zoomen
- **Pan**: Drag zum Verschieben (bei vielen Datenpunkten)

**Clear History Button:**
- **Position**: Top-Right in Card Header
- **Funktion**: Löscht lokale History
- **API**: `DELETE /resonance/history` (optional)
- **Confirmation**: Keine (direktes Löschen)

##### Gateway History

**Backend-persistierte Evaluations:**

**Anzeige:**
- **Titel**: "Gateway History (X records)"
- **Limit**: Letzte 10 Einträge
- **Sortierung**: Neueste zuerst (reverse)
- **Scroll**: Max-height 256px mit Scrollbar

**Eintrags-Format:**
- **Timestamp**: HH:mm:ss
- **Type**:
  - "Score: 0.6234" (grün) bei Output
  - "Gated" (rot) bei Gated
- **Label**: Falls vorhanden (z.B. "Eval-1735912345")
- **Layout**: Flex-Row mit justify-between

---

### 4. Cluster Analysis

**Route:** `/cluster`

**Zweck:** Entity-Clustering mit KNN, DBSCAN und Hierarchical Algorithmen

#### Compute Clusters Form

##### Snapshot ID

**Eingabe:**
- **Feld**: Text-Input
- **Placeholder**: "my-cluster-snapshot"
- **Beschreibung**: Eindeutiger Identifier für Cluster-Berechnung
- **Verwendung**: Gruppierung von Entities und Clustering-Ergebnissen
- **Validierung**: Nicht-leer erforderlich

##### Add Entities

**Entity-Eingabe (2 Felder + Button):**

**Address Field:**
- **Placeholder**: "Address (e.g., 0x1234...)"
- **Format**: Beliebiger String (wird als Entity-ID verwendet)
- **Beispiele**:
  - Ethereum: `0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb`
  - Bitcoin: `bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh`

**Features Field:**
- **Placeholder**: "Features (e.g., 0.5, 1.2, 0.8)"
- **Format**: Comma-separated Numbers
- **Parsing**: Split by `,`, trim, parseFloat
- **Validierung**: Alle Werte müssen gültige Zahlen sein
- **Dimensionen**: Beliebig (wird zu n-dimensionalem Feature-Vector)

**Add Button:**
- **Icon**: Plus
- **Funktion**: Fügt Entity zu Liste hinzu
- **Generiert**: Unique ID via `crypto.randomUUID()`
- **Reset**: Leert beide Felder nach Hinzufügen

**Beispiel-Eingabe:**
```
Address: 0x1234...5678
Features: 0.5, 1.2, 0.8, 0.3, 1.5
```

**Validierung:**
- Beide Felder müssen ausgefüllt sein
- Features müssen gültige Zahlen sein
- Bei ungültigen Features: Alert-Dialog

##### Entity List

**Anzeige:**
- **Header**: "Entities (X)" mit Anzahl
- **Scroll Area**: Max-height 200px
- **Empty State**: Keine Anzeige wenn Liste leer

**Entity-Karten:**

Jede Entity zeigt:
- **Address**: Truncated, Monospace-Font
- **Features**: Array-Anzeige `[0.5, 1.2, 0.8]`
- **Delete Button**: Trash-Icon (rot)
  - Entfernt Entity aus Liste
  - Keine Confirmation (direktes Löschen)

**Format:**
```
0x1234...5678 → [0.5, 1.2, 0.8, 0.3, 1.5]  🗑️
```

##### Algorithm Selection

**3 Algorithmen:**

**KNN (K-Nearest Neighbors)**
- **Beschreibung**: Graph-based Clustering
- **Parameter**: k (Anzahl Nachbarn)
- **Verwendung**: Für dichte, verbundene Cluster

**DBSCAN (Density-Based Spatial Clustering)**
- **Beschreibung**: Density-based Clustering
- **Parameter**: Threshold (Distanz-Schwellenwert)
- **Verwendung**: Für Cluster mit variabler Form

**Hierarchical**
- **Beschreibung**: Hierarchical Agglomerative Clustering
- **Parameter**: Threshold (Merge-Schwellenwert)
- **Verwendung**: Für nested Cluster-Strukturen

**Auswahl-UI:**
- **Layout**: 3 Buttons nebeneinander (Grid)
- **Visual Feedback**: Border + Background bei Selektion
- **Default**: KNN

##### Algorithm Parameters

**k (for KNN):**
- **Label**: "k (for KNN)"
- **Input**: Number, Min 1
- **Default**: 3
- **Enabled**: Nur bei KNN-Algorithmus
- **Beschreibung**: Anzahl der nächsten Nachbarn

**Threshold (for DBSCAN/Hierarchical):**
- **Label**: "Threshold (for DBSCAN/Hierarchical)"
- **Input**: Number, Min 0.01, Step 0.01
- **Default**: 0.5
- **Enabled**: Nur bei DBSCAN oder Hierarchical
- **Beschreibung**:
  - DBSCAN: Epsilon-Radius
  - Hierarchical: Merge-Distanz-Schwellenwert

##### Compute Button

**Funktion:**
- **Icon**: Play
- **Label**: "Compute Clusters"
- **API**: `POST /cluster/compute`
- **Parameter**:
  - `snapshot_id`: Snapshot-ID
  - `entities`: Array von `{ address, features }` (ohne ID)
  - `algorithm`: "knn" | "dbscan" | "hierarchical"
  - `k`: Nur bei KNN
  - `threshold`: Nur bei DBSCAN/Hierarchical

**Validierung:**
- Snapshot-ID nicht leer
- Mindestens 1 Entity in Liste
- Bei KNN: k ≥ 1
- Bei DBSCAN/Hierarchical: threshold > 0

**Loading State:**
- **Während Berechnung**: Spinner + "Computing Clusters..."
- **Disabled**: Button deaktiviert während API-Call

**Success:**
- **Setzt selectedSnapshot**: Für automatische Anzeige der Ergebnisse
- **Trigger**: WebSocket Event `ClusterComputed`
- **Notification**: Grüne Success-Box (5 Sekunden)

**Error:**
- **Alert-Box**: Roter Fehler-Banner mit Message
- **Mögliche Fehler**:
  - "Insufficient entities"
  - "Invalid feature dimensions"
  - "Clustering algorithm failed"

##### WebSocket Notification

**Bei erfolgreicher Berechnung:**

Grüne Notification-Box:
- **Icon**: CheckCircle
- **Message**: "Computed X cluster(s) for snapshot Y"
- **Auto-Dismiss**: Nach 5 Sekunden
- **Trigger**: WebSocket Event Type `ClusterComputed`
- **Payload**:
  - `snapshot_id`: ID des Snapshots
  - `num_clusters`: Anzahl berechneter Cluster
  - `timestamp`: ISO-8601 Timestamp

#### Cluster Results

##### Snapshot List

**Anzeige:**
- **Header**: "Cluster Snapshots (X)"
- **Loading**: Spinner während API-Call
- **Scroll Area**: Max-height 500px

**Empty State:**
- **Icon**: Großes Network-Icon (opacity 50%)
- **Message**: "No clusters computed yet"
- **Hint**: "Use the form above to compute clusters for entities"

##### Snapshot Cards

**Header:**
- **Icon**: Box (blau)
- **Name**: Snapshot-ID
- **Metadata**: "X cluster(s)"
- **View Button**: "View" / "Hide" Toggle

**Expanded View:**

**Cluster Items:**

Jedes Cluster zeigt:
- **Header (Collapsible)**:
  - **Chevron**: Down (expanded) / Right (collapsed)
  - **Icon**: BarChart3 (lila)
  - **Label**: Cluster-Label oder "Cluster {ID}"
  - **Badges**:
    - Size: Anzahl Members (mit Users-Icon)
    - Cohesion: Score als Dezimal (3 Stellen)

**Expanded Cluster:**

**Member List:**
- **Loading**: "Loading members..." mit Spinner
- **API**: `GET /cluster/:snapshot_id/:cluster_id` (lazy loading)
- **Anzeige**: Monospace-Font, graue Background-Boxes
- **Empty**: "No members found"

**Beispiel-Anzeige:**
```
> my-cluster-snapshot (3 clusters)

  > 📊 Cluster A  [👥 5] [Cohesion: 0.823]
      0x1234...5678
      0xabcd...ef01
      0x9876...5432
      0xfedc...ba98
      0x1111...2222

  > 📊 Cluster B  [👥 3] [Cohesion: 0.645]
      0x3333...4444
      0x5555...6666
      0x7777...8888
```

**Interaktionen:**
- **Click Snapshot Header**: Expand/Collapse gesamter Snapshot
- **Click Cluster Header**: Expand/Collapse einzelner Cluster
- **Lazy Loading**: Members werden erst bei Expansion geladen

---

### 5. Topology Viewer

**Route:** `/topology`

**Zweck:** Interaktive D3.js Netzwerk-Visualisierung

#### Force-Directed Graph

**Visualisierung:**
- **Library**: D3.js Force Simulation
- **Layout**: Force-Directed mit mehreren Kräften:
  - **Charge**: Abstoßung zwischen Knoten (-300)
  - **Link**: Anziehung zwischen verbundenen Knoten
  - **Center**: Zentrierung im Viewport
  - **Collision**: Verhindert Überlappung

**Nodes (Knoten):**
- **Darstellung**: Kreise
- **Größe**: Basierend auf Degree (Anzahl Verbindungen)
- **Farbe**: Kategorie-basiert oder Cluster-ID
- **Label**: Entity-Address (truncated)
- **Hover**: Tooltip mit vollständiger Info

**Edges (Kanten):**
- **Darstellung**: Linien zwischen Knoten
- **Dicke**: Basierend auf Edge Weight
- **Farbe**: Grau (standardmäßig)
- **Opacity**: 0.6

**Interaktionen:**

**Drag & Drop:**
- **Knoten ziehen**: Klicken + Ziehen verschiebt Knoten
- **Simulation Update**: Andere Knoten passen sich an
- **Fix on Drag**: Gezogener Knoten wird fixiert
- **Release**: Knoten wird wieder frei

**Zoom:**
- **Mausrad**: Zoom In/Out
- **Range**: 0.1x - 10x
- **Center**: Zoom zentriert auf Mausposition

**Pan:**
- **Leere Fläche ziehen**: Verschiebt gesamten Graph
- **Touch**: Unterstützt Touch-Gesten

**Click:**
- **Node Click**: Selektiert Knoten (Highlight)
- **Background Click**: Deselektiert alle

**Controls:**
- **Reset View Button**: Zentriert und reset Zoom
- **Play/Pause Button**: Startet/Stoppt Force-Simulation
- **Layout Button**: Wechselt zwischen Layout-Algorithmen (zukünftig)

#### Graph Metrics

**Anzeige (Card):**
- **Nodes**: Gesamtanzahl Knoten
- **Edges**: Gesamtanzahl Kanten
- **Components**: Anzahl disconnected Components
- **Density**: Graph-Dichte (edges / possible_edges)
- **Average Degree**: Durchschnittliche Knoten-Verbindungen

#### Entity Details Panel

**Bei Node-Selektion:**
- **Address**: Vollständige Adresse
- **Degree**: Anzahl Verbindungen
- **Neighbors**: Liste verbundener Nodes
- **Features**: Feature-Vector (falls vorhanden)
- **Cluster**: Zugehöriger Cluster (falls vorhanden)

---

### 6. Explorer

**Route:** `/explorer`

**Zweck:** Entity-Suche und Investigation

#### Search Interface

**Search Bar:**
- **Input**: Text-Feld mit Search-Icon
- **Placeholder**: "Search by address, transaction hash, or entity ID"
- **Autocomplete**: Vorschläge während Tippen (zukünftig)
- **Submit**: Enter-Taste oder Search-Button

**Filters:**
- **Blockchain**: Dropdown (All, Bitcoin, Ethereum, ...)
- **Entity Type**: Dropdown (All, Wallet, Contract, Exchange, ...)
- **Date Range**: Start/End Date Picker

#### Results

**Result Cards:**
- **Address**: Große Monospace-Anzeige
- **Blockchain**: Badge mit Icon
- **Balance**: Aktueller Saldo (falls verfügbar)
- **Transaction Count**: Anzahl Transaktionen
- **First Seen**: Datum der ersten Aktivität
- **Last Seen**: Datum der letzten Aktivität

**Actions:**
- **View Details**: Öffnet Detail-Modal
- **Add to Watchlist**: Fügt zu Überwachungsliste hinzu
- **Export**: CSV-Export der Entity-Daten

---

### 7. Anomalies

**Route:** `/anomalies`

**Zweck:** Anomalie-Detection und Alert-Management

#### Alert Dashboard

**Alert Cards:**
- **Severity**: Critical / High / Medium / Low
- **Type**: Z-Score Outlier / Hotspot / Pattern
- **Timestamp**: Wann erkannt
- **Entity**: Betroffene Entity
- **Description**: Textuelle Beschreibung

**Filters:**
- **Severity**: Alle / Critical / High / Medium / Low
- **Status**: Alle / Open / Investigating / Resolved
- **Date Range**: Zeitraum-Filter

**Actions:**
- **Mark as Reviewed**: Markiert Alert als überprüft
- **Investigate**: Öffnet Forensics-Workflow
- **Dismiss**: Verwirft Alert

#### Anomaly Visualization

**Chart:**
- **Typ**: Scatter Plot
- **X-Achse**: Zeit
- **Y-Achse**: Z-Score
- **Threshold Lines**: Bei ±3 (typisch für Anomalien)
- **Points**: Entities als Punkte, Anomalien hervorgehoben

---

### 8. Forensics

**Route:** `/forensics`

**Zweck:** Guided Forensic Analysis Workflows

#### Investigation Workflows

**Templates:**
- **Money Laundering**: Multi-Hop Transaction Tracing
- **Mixer Analysis**: Mixing Service Detection
- **Exchange Clustering**: Exchange Wallet Grouping
- **DeFi Flow**: DeFi Protocol Interaction Analysis

**Workflow Steps:**
1. **Entity Selection**: Auswahl Start-Entity
2. **Configuration**: Parameter für Analysis
3. **Execution**: Automated Analysis
4. **Results**: Visualization + Report
5. **Export**: PDF/JSON Export

#### Report Generation

**Components:**
- **Executive Summary**: Zusammenfassung
- **Timeline**: Chronologische Events
- **Graph Visualization**: Entity-Beziehungen
- **Anomalies**: Erkannte Auffälligkeiten
- **Recommendations**: Nächste Schritte

**Export-Formate:**
- **PDF**: Druckbarer Report
- **JSON**: Maschinenlesbare Daten
- **CSV**: Tabellen-Export

---

### 9. Infogenetik

**Route:** `/infogenetik`

**Zweck:** Specialized Infogenetic Analysis

#### Analysis Interface

**Input:**
- **Pattern**: Text-Input für Suchmuster
- **Operators**: WT, SW, DK, PI Konfiguration
- **Cycles**: Anzahl Iterationen

**Visualization:**
- **Search Space**: 2D/3D Visualisierung
- **Evolution**: Animation der Pattern-Evolution
- **Convergence**: Chart zeigt Konvergenz

---

### 10. Settings

**Route:** `/settings`

**Zweck:** System-Konfiguration

#### Appearance

**Theme:**
- **Light**: Heller Modus
- **Dark**: Dunkler Modus
- **System**: Folgt System-Präferenz
- **Auto**: Automatischer Wechsel (Tageszeit)

#### API Configuration

**Gateway URL:**
- **Input**: Text-Feld
- **Default**: `http://localhost:8080`
- **Validation**: URL-Format-Check
- **Test Connection**: Button zum Testen

#### Services

**Service Control:**
- **Start/Stop**: Buttons für Services
- **Auto-Start**: Checkbox für automatischen Start
- **Logs**: Link zu Service-Logs

---

## Komponenten-Bibliothek

### UI Components (ShadcnUI)

**Button:**
- **Variants**: default, destructive, outline, secondary, ghost, link
- **Sizes**: default, sm, lg, icon
- **Icons**: Mit lucide-react Icons

**Card:**
- **Components**: Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter
- **Styling**: Border, Radius, Shadow

**Input:**
- **Types**: text, password, number, email
- **States**: default, disabled, error
- **Icons**: Prefix/Suffix Icons möglich

**Badge:**
- **Variants**: default, secondary, destructive, outline, success, warning
- **Verwendung**: Status, Kategorien, Counts

**Dialog:**
- **Trigger**: Button zum Öffnen
- **Content**: Modal Content
- **Footer**: Action Buttons
- **Overlay**: Darkened Background

**Dropdown:**
- **Trigger**: Button/Input
- **Menu**: Liste von Items
- **Sections**: Gruppierung mit Separators

**Tabs:**
- **TabsList**: Horizontale Tab-Leiste
- **TabsTrigger**: Einzelne Tabs
- **TabsContent**: Content-Bereiche

**Tooltip:**
- **Trigger**: Hover-Element
- **Content**: Tooltip-Text
- **Position**: top, bottom, left, right

**ScrollArea:**
- **Scrollbar**: Custom Styled
- **Viewport**: Scrollbarer Content
- **Height**: Max-height konfigurierbar

**Separator:**
- **Orientation**: horizontal, vertical
- **Styling**: Thin Line, Muted Color

**Label:**
- **For**: Verknüpfung mit Input
- **Styling**: Font-weight, Color

**Checkbox:**
- **Checked**: Controlled State
- **Indeterminate**: Partial Selection
- **Disabled**: Non-interactive

**Switch:**
- **Toggle**: On/Off State
- **Label**: Optional Label-Text
- **Disabled**: Non-interactive

### Chart Components

**SpectralGauge (ECharts):**
- **Props**: `psi`, `rho`, `omega`, `height`
- **Rendering**: Canvas-based
- **Theme**: Dark/Light Adaptive

**ResonanceTimeSeries (ECharts):**
- **Props**: `data`, `height`
- **Lines**: 4 (ψ, ρ, ω, D)
- **Tooltip**: Interactive

**Scatter3DChart (ECharts):**
- **Props**: `data`, `height`
- **Interaction**: Rotate, Zoom
- **Axes**: X, Y, Z mit Labels

### Graph Components

**ForceGraph (D3.js):**
- **Props**: `nodes`, `links`, `width`, `height`
- **Simulation**: Force-Directed
- **Interaktionen**: Drag, Zoom, Click

### Common Components

**MetricCard:**
- **Props**: `title`, `value`, `icon`, `trend?`
- **Layout**: Icon + Value + Trend
- **Styling**: Card-based

**ActivityFeed:**
- **Props**: Keine (nutzt Store)
- **Data Source**: Zustand Store
- **Update**: Real-time via WebSocket
- **Limit**: 50 Einträge

**CommandPalette:**
- **Trigger**: ⌘+K / Ctrl+K
- **Search**: Fuzzy Search
- **Categories**: Navigation, Actions
- **Keyboard**: Full Keyboard Navigation

---

## State Management

### Zustand Stores

**app.ts (App Store):**
```typescript
interface AppStore {
  theme: 'light' | 'dark' | 'system'
  sidebarCollapsed: boolean
  commandPaletteOpen: boolean

  setTheme: (theme) => void
  toggleSidebar: () => void
  toggleCommandPalette: () => void
}
```

**metrics.ts (Metrics Store):**
```typescript
interface MetricsStore {
  seeds: number
  clusters: number
  entities: number
  anomalies: number
  analyses: number
  hotspots: number
  recentActivity: Activity[]

  setMetrics: (partial) => void
  addActivity: (activity) => void
}
```

**services.ts (Services Store):**
```typescript
interface ServicesStore {
  scraper: ServiceInfo
  analyzer: ServiceInfo
  cluster: ServiceInfo

  updateService: (name, info) => void
}

interface ServiceInfo {
  status: 'online' | 'pending' | 'offline'
  processed: number
  lastUpdate: string
}
```

**infogenetik.ts (Infogenetik Store):**
```typescript
interface InfogenetikStore {
  pattern: string
  operators: Operators
  results: SearchResult[]

  setPattern: (pattern) => void
  setOperators: (operators) => void
  addResult: (result) => void
}
```

---

## API Integration

### React Query (useApi)

**15 Custom Hooks:**

#### Resonance Hooks

```typescript
useAnalyzeResonance()      // POST /resonance/analyze
useComputeSpectral()       // POST /resonance/spectral
useResonanceHistory()      // GET /resonance/history
useClearResonanceHistory() // DELETE /resonance/history
```

#### Wallet Hooks

```typescript
useImportMnemonic()        // POST /wallet/import
useDeriveAddresses()       // POST /wallet/derive
useWalletList()            // GET /wallet/list
useRemoveWallet()          // DELETE /wallet/:label
```

#### Cluster Hooks

```typescript
useComputeClusters()       // POST /cluster/compute
useAllClusters()           // GET /cluster/all
useClustersForSnapshot()   // GET /cluster/:snapshot_id
useClusterMembers()        // GET /cluster/:snapshot_id/:id
```

#### Satellite Hooks (zukünftig)

```typescript
useIngestSnapshot()        // POST /satellite/v1/snapshots
useAnalyzeSnapshot()       // POST /satellite/v1/analyze/:id
useGetReport()             // GET /satellite/v1/reports/:id
```

**Hook-Pattern:**

```typescript
const {
  mutate,           // Trigger function
  isPending,        // Loading state
  error,            // Error object
  data,             // Response data
  isSuccess         // Success state
} = useApiHook()

// Usage
mutate(params, {
  onSuccess: (data) => { /* ... */ },
  onError: (error) => { /* ... */ }
})
```

### WebSocket (useWebSocket)

**Hook:**

```typescript
useWebSocket({
  onEvent: (event: WebSocketEvent) => void
  onConnect?: () => void
  onDisconnect?: () => void
})
```

**Event Types:**

```typescript
type WebSocketEvent =
  | LogEvent              // Server logs
  | AnalysisProgressEvent // 0-100% progress
  | AnalysisCompleteEvent // Analysis done
  | ServiceStatusEvent    // Service state changes
  | ResonanceEvaluatedEvent // Resonance result
  | WalletDerivedEvent    // Addresses derived
  | ClusterComputedEvent  // Clusters computed
  | NotificationEvent     // Generic notifications
```

**Connection Management:**
- **Auto-Reconnect**: Exponential Backoff (1s, 2s, 4s, 8s, 16s)
- **Heartbeat**: Ping alle 30 Sekunden
- **Status Display**: In StatusBar

---

## Keyboard Shortcuts

| Shortcut | Aktion |
|----------|--------|
| `⌘+K` / `Ctrl+K` | Open Command Palette |
| `⌘+B` / `Ctrl+B` | Toggle Sidebar |
| `⌘+/` / `Ctrl+/` | Open Shortcuts Help |
| `Esc` | Close Dialogs/Modals |
| `Tab` | Navigate Forward |
| `Shift+Tab` | Navigate Backward |
| `Enter` | Submit/Confirm |
| `Space` | Toggle Checkboxes |
| `↑/↓` | Navigate Lists |
| `←/→` | Navigate Tabs |

---

## Responsive Design

### Breakpoints

```css
sm: 640px   /* Small tablets */
md: 768px   /* Tablets */
lg: 1024px  /* Laptops */
xl: 1280px  /* Desktops */
2xl: 1536px /* Large desktops */
```

### Adaptive Layouts

**Mobile (< 640px):**
- Sidebar hidden by default
- Stacked cards
- Single column grids
- Simplified charts

**Tablet (640px - 1024px):**
- Collapsible sidebar
- 2-column grids
- Full charts
- Touch-optimized controls

**Desktop (> 1024px):**
- Persistent sidebar
- 3-4 column grids
- Advanced visualizations
- Keyboard shortcuts active

---

## Accessibility

### ARIA Labels

Alle interaktiven Elemente haben:
- `aria-label` für Screen Readers
- `role` Attribute (button, dialog, navigation, etc.)
- `aria-expanded` für expandierbare Elemente
- `aria-selected` für ausgewählte Items

### Keyboard Navigation

- **Tab Order**: Logische Reihenfolge
- **Focus Indicators**: Sichtbare Focus-Ringe
- **Skip Links**: "Skip to content" Link
- **Esc Handling**: Schließt Modals/Dialogs

### Screen Reader Support

- **Semantic HTML**: Korrekte HTML5 Elemente
- **Alt Text**: Alle Bilder/Icons haben Alt-Text
- **Live Regions**: ARIA Live für dynamische Updates
- **Form Labels**: Alle Inputs haben Labels

---

## Performance

### Optimierungen

**Code Splitting:**
- Route-based Splitting
- Lazy Loading für große Components
- Dynamic Imports

**Memoization:**
- React.memo für teure Components
- useMemo für teure Berechnungen
- useCallback für Event Handlers

**Virtualization:**
- Lange Listen virtualisiert (react-window)
- Nur sichtbare Items gerendert
- Smooth Scrolling

**Caching:**
- React Query Cache (5 Minuten default)
- LocalStorage für Theme/Preferences
- Service Worker (zukünftig)

**Bundle Size:**
- Tree Shaking aktiv
- Code Minification
- Gzip/Brotli Compression

---

## Fehlerbehandlung

### Error Boundaries

**Komponenten-Level:**
- Error Boundary umschließt Feature-Components
- Fallback-UI bei Crashes
- Error Logging an Backend

**API-Level:**
- React Query Error Handling
- Retry-Logic (3 Versuche)
- User-friendly Error Messages

### Loading States

**Skeleton Screens:**
- Während Initial Load
- Content-Placeholder
- Smooth Transitions

**Spinners:**
- Für Button-Actions
- API-Calls
- Background Tasks

**Progress Bars:**
- Für lange Operations
- Prozentuale Anzeige
- Estimated Time (optional)

---

## Deployment

### Build

```bash
cd phosphoros-web
npm run build
```

**Output:**
- `dist/` Ordner mit statischen Files
- Optimiert und Minified
- Gzip/Brotli komprimiert

### Environment Variables

**Development:**
```env
VITE_API_URL=http://localhost:8080
```

**Production:**
```env
VITE_API_URL=https://api.phosphoros.io
```

### Docker

```dockerfile
FROM node:18-alpine as build
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

---

## Troubleshooting

### Häufige Probleme

**WebSocket Connection Failed:**
- **Symptom**: "Disconnected" in StatusBar
- **Lösung**: Gateway läuft nicht → `cargo run -p phosphoros-gateway`

**API Calls Fail:**
- **Symptom**: "Network Error" in Alerts
- **Lösung**: CORS-Settings prüfen, Gateway-URL korrekt?

**Blank Page:**
- **Symptom**: Weiße Seite nach Build
- **Lösung**: Base URL in `vite.config.ts` prüfen

**Slow Performance:**
- **Symptom**: Laggy UI
- **Lösung**: React DevTools Profiler nutzen, Memoization prüfen

---

## Zusammenfassung

Die PHOSPHOROS Web-GUI ist eine **vollständig integrierte, moderne Single-Page-Application** für Blockchain-Forensik mit folgenden Highlights:

✅ **10 spezialisierte Seiten** für alle Analyse-Workflows
✅ **Real-time Updates** via WebSocket-Integration
✅ **Interaktive Visualisierungen** mit D3.js und ECharts
✅ **Type-safe End-to-End** mit TypeScript + Rust
✅ **Production-Ready** mit Docker-Deployment
✅ **Responsive** für Desktop & Tablet
✅ **Accessible** mit ARIA-Support
✅ **Performant** mit Code Splitting & Caching

**Nächste Schritte:**
1. Siehe [README.md](README.md) für Quick Start
2. Siehe [DEPLOYMENT.md](DEPLOYMENT.md) für Production Deployment
3. Siehe [docs/api.md](docs/api.md) für API-Details

---

<div align="center">

**PHOSPHOROS Web GUI**

*Modern React Dashboard für 5D Blockchain-Forensik*

Made with ⚡ by the PHOSPHOROS Team

</div>
