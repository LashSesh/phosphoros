# PHOSPHOROS Web Dashboard

Modern enterprise web dashboard for PHOSPHOROS blockchain forensics platform.

## Features

- **Command Palette** (⌘+K): Quick access to any action
- **Real-time Updates**: WebSocket-powered live data
- **Interactive Visualizations**: D3.js network graphs, ECharts charts
- **Dark/Light Theme**: System-aware theming
- **Responsive Design**: Works on desktop and tablet

## Technology Stack

- **React 18** with TypeScript
- **TailwindCSS** + ShadcnUI components
- **Zustand** for state management
- **React Query** for server state
- **D3.js** for network topology
- **ECharts** for data visualization
- **Vite** for fast development

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Project Structure

```
src/
├── components/
│   ├── ui/              # ShadcnUI primitives
│   ├── layout/          # Shell, Sidebar, Header
│   ├── charts/          # ECharts wrappers
│   ├── graphs/          # D3 force graph
│   └── common/          # Shared components
├── features/
│   ├── dashboard/       # Home overview
│   ├── wallet/          # Seed management
│   ├── resonance/       # Spectral analysis
│   ├── topology/        # Network graphs
│   ├── investigation/   # Forensics
│   └── settings/        # Configuration
├── hooks/               # Custom React hooks
├── stores/              # Zustand stores
├── lib/                 # Utilities
└── types/               # TypeScript types
```

## Pages

| Route | Description |
|-------|-------------|
| `/` | Dashboard home with metrics |
| `/wallet` | Seed/mnemonic management |
| `/resonance` | Spectral analysis |
| `/topology` | Network graph visualization |
| `/explorer` | Entity search |
| `/anomalies` | Alert management |
| `/forensics` | Investigation workflows |
| `/settings` | Configuration |

## Docker

```bash
# Build image
docker build -t phosphoros-web .

# Run container
docker run -p 3000:80 phosphoros-web
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `VITE_API_URL` | Backend API URL | `http://localhost:8080` |

## License

MIT OR Apache-2.0
