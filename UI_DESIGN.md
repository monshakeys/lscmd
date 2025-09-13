# lscmd TUI Design Specification

## Overview
A terminal user interface with a 4-panel layout optimized for browsing and searching shell aliases and functions. Design inspired by GitUI's visual style with clear selection highlighting and responsive cursor movement feedback.

**Related Documentation**: See `UI_LOGIC.md` for detailed interaction logic and state management rules.

## Layout Structure

### Screen Dimensions & Margins
- **Margin**: 12px equivalent border around entire screen
- **Border**: Single white line around content area
- **Background**: Blue-gray base color
- **Content Area**: Screen minus margins, divided into 4 panels

### Panel Layout
```
┌─────────────────────────────────────────────────────────┐
│ ┌─────────────────────────────────────────────────────┐ │
│ │                TOP PANEL (1/5)                      │ │
│ │                Shortcuts & Help                     │ │
│ └─────────────────────────────────────────────────────┘ │
│ ┌═════════════════════════════┐ ┌─────────────────────┐ │
│ ║                             ║ │                     │ │
│ ║         LEFT PANEL          ║ │    RIGHT PANEL      │ │
│ ║        (2/3 width)          ║ │   (1/3 width)       │ │
│ ║    Aliases & Functions      ║ │  File List / Search │ │
│ ║         (3/5 height)        ║ │    (3/5 height)     │ │
│ ║                             ║ │                     │ │
│ └═════════════════════════════┘ └─────────────────────┘ │
│ ┌─────────────────────────────────────────────────────┐ │
│ │               BOTTOM PANEL (1/5)                    │ │
│ │              Code Display                          │ │
│ └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```
Note: ═ indicates focused panel with thick white border

### Panel Dimensions
- **Top Panel**: 100% width × 20% height
- **Left Panel**: 67% width × 60% height (focusable)
- **Right Panel**: 33% width × 60% height (focusable)
- **Bottom Panel**: 100% width × 20% height

## Panel Specifications

### 1. Top Panel - Shortcuts & Help
**Purpose**: Display keyboard shortcuts and usage instructions
**Height**: 1/5 of screen
**Width**: Full width

**Content**:
```
┌─ Shortcuts ────────────────────────────────────────────┐
│ ↑↓: Navigate  Enter: Select  Tab: Switch Panel        │
│ /: Search     q: Quit        Space: Preview           │
│ c: Copy       e: Edit file                            │
└───────────────────────────────────────────────────────┘
```

### 2. Left Panel - Aliases & Functions List
**Purpose**: Display aliases and functions from selected file or search results
**Height**: 3/5 of screen (60%)
**Width**: 2/3 of screen (67%)
**Focus**: Can receive focus (thick white border when focused)

**Content Structure**:
```
┌─ git.sh (12 items) ────────────────────────┐
│ 📁 ALIASES (8)                            │
│ > ga                    "git add"         │
│   gc                    "git commit"      │
│   gp                    "git push"        │
│                                           │
│ 🔧 FUNCTIONS (4)                          │
│ > glog                  Show git log      │
│   gbr                   Git branch util   │
│   gst                   Git status        │
└───────────────────────────────────────────┘
```

**Features**:
- Grouped by type (aliases vs functions)
- Item count display
- Scrollable list
- Type icons (📁 for alias, 🔧 for function)
- **GitUI-style highlighting**: Blue hover bar for cursor movement (no manual selection)
- **Dynamic text color**: White text on colored backgrounds, light gray on normal background
- **Immediate feedback**: Cursor movement directly updates bottom panel code display
- **Focus behavior**: Detailed in `UI_LOGIC.md` - Focus Management section

### 3. Right Panel - File List / Search Input
**Purpose**: Display available .sh files or search interface
**Height**: 3/5 of screen (60%) 
**Width**: 1/3 of screen (33%)
**Focus**: Can receive focus (thick white border when focused)

#### Normal Mode - File List
**Content Structure**:
```
┌─ Files (20) ─────────────┐
│ > git.sh        (12)    │
│   docker.sh     (8)     │
│   node.sh       (15)    │
│   system.sh     (6)     │
│   utils.sh      (23)    │
│   ⋮                     │
└─────────────────────────┘
```

#### Search Mode - Input Interface  
**Content Structure**:
```
┌─ Search ─────────────────┐
│ ┌─────────────────────┐  │
│ │ git_                │  │
│ └─────────────────────┘  │
│                         │
│ 12 matches found        │
│                         │
└─────────────────────────┘
```

**Features**:
- **File List Mode**: File name display, item count per file, selection indicator (>), scrollable list
- **Search Mode**: Real-time search input, match count display, cursor in input field
- **GitUI-style highlighting**: Blue hover bar, green selection bar
- **Text color adaptation**: White text on colored backgrounds
- **Mode switching**: Press "/" to toggle between modes

### 4. Bottom Panel - Code Display
**Purpose**: Show full code/command content for selected item
**Height**: 1/5 of screen (20%)
**Width**: Full width

**Content Structure**:
```
┌─ glog() ─────────────────────────────────────────────────┐
│ function glog() {                                        │
│     git log --oneline --graph --decorate --all          │
│     --color=always | head -20                           │
│ }                                                        │
└─────────────────────────────────────────────────────────┘
```

**Features**:
- Syntax highlighting
- Scrollable for long content
- Copy to clipboard functionality
- Code formatting

## Color Scheme

### Base Colors
- **Background**: Blue-gray (#2E3440)
- **Border**: Soft white (#E5E9F0) - subtle, textured border
- **Panel Background**: Slightly lighter blue-gray (#3B4252)
- **Text**: Light gray (#ECEFF4)

### Selection & Highlighting (GitUI Style)
- **Cursor Hover**: Bright blue background (#5E81AC) with white text (#FFFFFF)
- **Selected Item**: Green background (#A3BE8C) with white text (#FFFFFF)
- **Focused Panel**: Pure white thick border (#FFFFFF) - double thickness
- **Unfocused Panel**: Soft white border (#E5E9F0) - normal thickness

### Accent Colors
- **Alias Icon**: Green (#A3BE8C)  
- **Function Icon**: Orange (#D08770)
- **Search Match**: Yellow highlight (#EBCB8B) with dark text (#2E3440)
- **File Count**: Muted blue (#81A1C1)

### Status Colors
- **Search Input**: White background (#FFFFFF), dark text (#2E3440)
- **Error Text**: Red (#BF616A)
- **Success Text**: Green (#A3BE8C)

## Interaction Flow

**Note**: For detailed behavioral logic, see `UI_LOGIC.md` - Application States section.

### Visual Feedback
1. **Cursor Movement**: Blue background bar (#5E81AC) follows cursor with white text for clear tracking
2. **File Selection**: Green background bar (#A3BE8C) for selected files in Right Panel
3. **Panel Focus**: Pure white thick border (#FFFFFF) for focused panel, soft white (#E5E9F0) for others
4. **Text Contrast**: Dynamic text colors ensure readability on all background colors

### Navigation States
1. **Default State**: Focus on Right Panel (File List), Left Panel empty
2. **File Selected**: Focus moves to Left Panel, shows commands, auto-selects first
3. **Search Mode**: Right Panel becomes input field, Left Panel shows live results

### Key Interaction Patterns
1. **File Selection**: Enter/Space toggles selection in Right Panel (File List mode)
2. **Command Navigation**: Arrow keys in Left Panel immediately update Bottom Panel
3. **Search Activation**: "/" key switches Right Panel to search input mode
4. **Focus Transitions**: Automatic focus shift when file selected, manual with ESC/→
5. **Real-time Updates**: Search results and code display update immediately

**Complete interaction details available in `UI_LOGIC.md`**

## Responsive Behavior

### Small Screens (< 80 columns)
- Collapse right panels to overlay mode
- Tab switches between file list and search
- Maintain core functionality

### Large Screens (> 120 columns)
- Expand code panel for better visibility
- Show more context in file list
- Display additional shortcuts in top panel

## Keyboard Shortcuts

**Note**: Complete shortcut behavior detailed in `UI_LOGIC.md` - Keyboard Shortcuts Summary.

| Key | Action | Context |
|-----|--------|---------|
| `↑/↓` | Navigate with blue highlighting | Focused panel |
| `Enter/Space` | Toggle file selection (green) | Right Panel (File List) |
| `/` | Toggle search mode | Any state |
| `ESC` | Return focus/exit search | File Selected/Search Mode |
| `→` | Return focus (keep content) | File Selected state |
| `q` | Quit application | Any state |
| `c` | Copy to clipboard | Code display |

**Focus Management**: Only Left Panel (Commands) and Right Panel (Files/Search) can receive focus.

## Implementation Notes

### Panel Management
- Use `ratatui::layout::Layout` for responsive sizing
- Implement focus management system
- Handle panel switching with state machine

### Performance
- Lazy load file contents
- Cache parsed aliases/functions
- Debounce search input
- Virtual scrolling for large lists

### Accessibility  
- Clear visual focus indicators
- Consistent keyboard navigation
- Status announcements for screen readers
- High contrast mode support