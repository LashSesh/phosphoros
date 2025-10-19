# PHOSPHOROS UI Design Guide

## Quick Reference for Developers

This guide provides quick references for maintaining the premium UI aesthetic across the PHOSPHOROS dashboard.

## Color Quick Reference

### When to Use Each Color

```rust
use crate::theme::colors;

// Backgrounds (from darkest to lightest)
colors::BACKGROUND        // #141519 - App background
colors::SIDEBAR_BG        // #0F1012 - Sidebar
colors::PANEL_BG          // #1C1D26 - Panels
colors::CARD_BG           // #21222B - Cards

// Text (from brightest to dimmest)
colors::TEXT              // #EBEDEF - Primary text
colors::TEXT_SECONDARY    // #A6A9B2 - Secondary text
colors::TEXT_TERTIARY     // #737680 - Tertiary text

// Interactive
colors::PRIMARY           // #4090F3 - Primary actions
colors::PRIMARY_BRIGHT    // #59A6FF - Hover states
colors::PRIMARY_DARK      // #2673D9 - Active states

// Status
colors::SUCCESS           // #2EB88E - Positive
colors::WARNING           // #F2A62E - Caution
colors::ERROR             // #ED4859 - Critical
colors::INFO              // #38ADE0 - Information

// Borders
colors::BORDER            // #2E2F38 - Standard
colors::BORDER_HOVER      // #474850 - Hover
colors::BORDER_ACTIVE     // #4090F3 - Active/Focus
```

## Component Quick Reference

### Buttons

```rust
use crate::widgets::{primary_button, secondary_button, success_button, danger_button};

// Main action (e.g., "Import", "Generate", "Start")
primary_button("Import Seed")
    .on_press(message)

// Secondary action (e.g., "Clear", "Cancel", "Export")
secondary_button("Clear")
    .on_press(message)

// Confirmation (e.g., "Confirm", "Accept")
success_button("Confirm")
    .on_press(message)

// Destructive (e.g., "Delete", "Remove")
danger_button("Delete")
    .on_press(message)
```

### Cards

```rust
use crate::widgets::{card, status_card, StatusType};

// Standard card
card(
    column![
        text("Title").size(16),
        text("Content").size(14),
    ]
)

// Status cards with colored borders
status_card(StatusType::Success, content)  // Green border
status_card(StatusType::Warning, content)  // Amber border
status_card(StatusType::Error, content)    // Red border
status_card(StatusType::Info, content)     // Cyan border
```

### Badges

```rust
use crate::widgets::{badge, BadgeType};

badge("ACTIVE", BadgeType::Success)   // Green badge
badge("PAUSED", BadgeType::Neutral)   // Gray badge
badge("HIGH", BadgeType::Warning)     // Amber badge
badge("CRITICAL", BadgeType::Error)   // Red badge
badge("INFO", BadgeType::Info)        // Cyan badge
badge("PRIMARY", BadgeType::Primary)  // Blue badge
```

### Headers

```rust
use crate::widgets::{header, section_header};

// Panel header with subtitle
header("Panel Title", Some("Description"))

// Section divider with title
section_header("Section Name")

// Or create custom headers
column![
    text("Title").size(36),
    vertical_space().height(4),
    text("Subtitle").size(14),
]
```

### Metric Cards

```rust
use crate::widgets::{metric_card, MetricTrend};

// Simple metric
metric_card("Label", "Value", "🔑", None)

// Metric with trend
metric_card(
    "Active Users",
    "1,234",
    "👥",
    Some(MetricTrend::Up("+12%".to_string()))
)

// Declining metric
metric_card(
    "Errors",
    "5",
    "⚠️",
    Some(MetricTrend::Down("-50%".to_string()))
)
```

## Typography Scale

```rust
// Headings (semantic structure)
.size(36)  // H1 - Main panel titles
.size(28)  // H2 - Legacy compatibility
.size(24)  // H3 - Major sections
.size(18)  // H4 - Subsections
.size(16)  // H5 - Card headers

// Body (content)
.size(16)  // Large body text
.size(14)  // Standard body text
.size(13)  // Supporting text
.size(12)  // Small text, badges
.size(11)  // Fine print, metadata

// Special (non-semantic emphasis)
.size(48)  // Extra large icons (empty states)
.size(36)  // Large metrics/statistics (same size as H1, but for numbers)
.size(20)  // Sub-metrics
```

## Spacing Scale

```rust
// Vertical spacing
vertical_space().height(4)   // Micro - tight groups
vertical_space().height(8)   // Small - related items
vertical_space().height(12)  // Medium - list items
vertical_space().height(16)  // Large - section breaks
vertical_space().height(20)  // XLarge - major breaks
vertical_space().height(28)  // XXLarge - panel sections
vertical_space().height(32)  // XXXLarge - main sections

// Horizontal spacing
horizontal_space().width(8)   // Tight - button groups
horizontal_space().width(12)  // Medium - controls
horizontal_space().width(16)  // Wide - columns
horizontal_space().width(20)  // XWide - major divisions

// Padding
.padding(20)      // Cards
.padding([12, 24]) // Buttons (vertical, horizontal)
.padding(14)      // Inputs
.padding(16)      // Containers
```

## Common Patterns

### Panel Structure

```rust
fn my_panel_view(&self) -> Element<Message> {
    use crate::widgets::{section_header, primary_button};
    
    // 1. Premium header
    let title = column![
        text("Panel Name").size(36),
        vertical_space().height(4),
        text("Description of panel").size(14),
    ];
    
    // 2. Main content in cards
    let content = column![
        title,
        vertical_space().height(32),
        
        // Section 1
        card(
            column![
                section_header("Section Name"),
                vertical_space().height(16),
                // ... section content
            ]
        ),
        vertical_space().height(20),
        
        // Section 2
        card(
            column![
                section_header("Another Section"),
                vertical_space().height(16),
                // ... section content
            ]
        ),
    ];
    
    scrollable(content).into()
}
```

### Empty States

```rust
let empty_state = card(
    column![
        text("🔍").size(48),
        vertical_space().height(16),
        text("No items found").size(20),
        vertical_space().height(8),
        text("Description of what to do").size(14),
    ].align_x(iced::Alignment::Center)
);
```

### Action Rows

```rust
// With primary and secondary actions
let action_row = row![
    primary_button("Primary Action")
        .on_press(primary_msg),
    horizontal_space().width(12),
    secondary_button("Secondary")
        .on_press(secondary_msg),
];

// With export actions
let export_row = row![
    secondary_button("Export JSON")
        .on_press(export_json),
    horizontal_space().width(8),
    secondary_button("Export CSV")
        .on_press(export_csv),
    horizontal_space().width(8),
    secondary_button("Export Markdown")
        .on_press(export_md),
];
```

### Status Indicators

```rust
// Status with badge
row![
    text("Service Name").size(16),
    horizontal_space().width(12),
    badge(
        if running { "ACTIVE" } else { "PAUSED" },
        if running { BadgeType::Success } else { BadgeType::Neutral }
    ),
    horizontal_space().width(12),
    text(format!("Details: {}", details)).size(14),
].align_y(iced::Alignment::Center)
```

### Metric Displays

```rust
// Side-by-side metrics
row![
    column![
        text("Metric 1").size(11),
        vertical_space().height(4),
        text(value1).size(20),
    ],
    horizontal_space().width(32),
    column![
        text("Metric 2").size(11),
        vertical_space().height(4),
        text(value2).size(20),
    ],
]
```

### Settings Toggles

```rust
row![
    column![
        text("Setting Name").size(16),
        vertical_space().height(4),
        text("Description of setting").size(12),
    ],
    horizontal_space().width(Length::Fill),
    toggler(value)
        .on_toggle(message),
].align_y(iced::Alignment::Center)
```

## Do's and Don'ts

### ✅ DO

- Use semantic color constants (`colors::PRIMARY` not hardcoded values)
- Use predefined widget functions (`card()`, `badge()`, etc.)
- Follow the 8px spacing grid (4, 8, 12, 16, 20, 24, 28, 32)
- Use the typography scale (11, 12, 13, 14, 16, 18, 20, 24, 28, 36, 48)
- Apply shadows to elevated elements
- Add section headers to organize content
- Use badges for status indicators
- Maintain consistent padding across similar elements
- Group related navigation items
- Show empty states with helpful messages

### ❌ DON'T

- Hardcode RGB values (`Color::from_rgb(0.5, 0.5, 0.5)`)
- Create one-off custom styles inline
- Use random spacing values (7px, 15px, 23px, etc.)
- Mix font sizes without following the scale
- Forget shadows on cards and buttons
- Use plain text for status (use badges)
- Leave panels without headers
- Show empty lists without explanation
- Mix interaction patterns within the same panel
- Create navigation without grouping

## Accessibility Checklist

- [ ] Text contrast ≥ 7:1 for primary text
- [ ] Interactive elements have visible states
- [ ] Touch targets ≥ 44px (buttons with padding)
- [ ] Related items are grouped visually
- [ ] Status conveyed beyond color alone (icons, text)
- [ ] Consistent interaction patterns
- [ ] Clear visual hierarchy

## Performance Tips

- Cards with shadows use GPU acceleration (no performance impact)
- Color constants are resolved at compile time
- Widget functions are zero-cost abstractions
- Shadow blur radius affects render performance:
  - 8px: Negligible impact
  - 12px: Light impact
  - 24px: Moderate impact (use sparingly)

## Testing Your Changes

```bash
# Build to check compilation
cargo build -p phosphoros-dashboard

# Run to see visual changes
cargo run -p phosphoros-dashboard --release

# Run tests to ensure nothing broke
cargo test --workspace
```

## Common Mistakes

### 1. Inconsistent Spacing
```rust
// ❌ Bad
vertical_space().height(15)
vertical_space().height(22)

// ✅ Good
vertical_space().height(16)
vertical_space().height(20)
```

### 2. Manual Styling
```rust
// ❌ Bad
container(content)
    .style(|_| {
        container::Style {
            background: Some(Color::from_rgb(0.2, 0.2, 0.25).into()),
            // ... manual style
        }
    })

// ✅ Good
card(content)
```

### 3. Missing Status Indicators
```rust
// ❌ Bad
text(format!("Status: {}", if active { "Active" } else { "Paused" }))

// ✅ Good
row![
    text("Status").size(16),
    badge(
        if active { "ACTIVE" } else { "PAUSED" },
        if active { BadgeType::Success } else { BadgeType::Neutral }
    ),
]
```

### 4. Flat Information
```rust
// ❌ Bad
column![
    text("Title"),
    text("Item 1"),
    text("Item 2"),
]

// ✅ Good
column![
    section_header("Title"),
    vertical_space().height(16),
    card(text("Item 1")),
    vertical_space().height(12),
    card(text("Item 2")),
]
```

## Quick Start Template

```rust
fn new_panel_view(&self) -> Element<Message> {
    use crate::widgets::{
        section_header, 
        badge, BadgeType,
        primary_button, secondary_button
    };
    
    let title = column![
        text("Panel Name").size(36),
        vertical_space().height(4),
        text("Panel description").size(14),
    ];
    
    let content = column![
        title,
        vertical_space().height(32),
        
        // Your content here
        card(
            column![
                section_header("Section"),
                vertical_space().height(16),
                // Add your widgets
            ]
        ),
    ];
    
    scrollable(content).into()
}
```

## Resources

- **Theme Module**: `crates/phosphoros-dashboard/src/theme/mod.rs`
- **Widgets Module**: `crates/phosphoros-dashboard/src/widgets/mod.rs`
- **Full Documentation**: `PREMIUM_UI_ENHANCEMENTS.md`
- **Examples**: View existing panels in `crates/phosphoros-dashboard/src/app.rs`

---

**Remember**: Consistency is key to a premium experience. When in doubt, look at existing implementations and follow the established patterns.
