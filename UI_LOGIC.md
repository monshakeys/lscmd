# lscmd TUI Logic Specification

## Overview
This document defines the interaction logic and behavioral rules for the lscmd TUI interface. For visual design elements, refer to `UI_DESIGN.md`.

## Focus Management

### Focusable Panels
Only two panels can receive focus and display the thick white border:
- **Left Panel**: Aliases & Functions List
- **Right Panel**: File List / Search Input

### Focus Indicators
- **Focused Panel**: Pure white thick border (#FFFFFF) - double thickness
- **Unfocused Panel**: Soft white border (#E5E9F0) - normal thickness
- **Non-focusable Panels**: Always use soft white border (Top Panel, Bottom Panel)

## Application States

### 1. Default State (File Selection Mode)
**Initial Condition**:
- Focus: Right Panel (File List mode)
- Left Panel: Empty/blank state
- Bottom Panel: Empty/welcome message

**User Actions**:
- `↑/↓`: Navigate through file list with blue hover highlighting
- `Enter/Space`: Toggle file selection (green background for selected)
- `Tab`: No effect (only two focusable panels)
- `ESC`: No effect (already in default state)
- `/`: Switch to Search Mode

### 2. File Selected State (Command Browsing Mode)
**Triggered When**: User selects a .sh file in Right Panel

**State Changes**:
- Focus: Automatically shifts to Left Panel
- Left Panel: Displays all aliases and functions from selected file
- Left Panel: Auto-selects first command (green background)
- Bottom Panel: Shows code for the first selected command

**User Actions**:
- `↑/↓`: Navigate through commands (blue hover highlighting)
- **No selection action**: Movement directly updates bottom panel (no Enter/Space needed)
- `ESC`: Return focus to Right Panel, clear Left Panel
- `→`: Return focus to Right Panel, keep Left Panel content
- `/`: Switch to Search Mode

### 3. Search Mode
**Triggered When**: User presses `/` in any state

**State Changes**:
- Focus: Right Panel (Search Input mode)
- Right Panel: File list replaced with search input box
- Left Panel: Shows search results as user types
- Bottom Panel: Shows code for selected search result

**User Actions**:
- `Type`: Real-time search, Left Panel updates with each keystroke
- `↑/↓`: Navigate through search results in Left Panel
- `Enter`: Select search result, show code in Bottom Panel
- `ESC`: Exit search mode, return to previous state
- `/`: Toggle off search mode, return to File Selection Mode

## Panel Behavior Details

### Right Panel - File List Mode
```
Focus State: Can receive focus
Selection: Single file can be selected (green background)
Navigation: ↑/↓ keys with blue hover highlighting
Action: Enter/Space toggles selection state
```

### Right Panel - Search Input Mode
```
Focus State: Can receive focus (input cursor visible)
Input: Real-time text input with search triggering
Navigation: Text cursor movement with ←/→
Action: Each keystroke triggers search in database
```

### Left Panel - Command List Mode
```
Focus State: Can receive focus
Selection: No explicit selection (no green background)
Navigation: ↑/↓ keys with blue hover highlighting
Action: Movement immediately updates Bottom Panel
Constraint: Cannot multi-select or toggle selection
```

### Left Panel - Search Results Mode
```
Focus State: Can receive focus  
Selection: Auto-selects first result, no manual selection
Navigation: ↑/↓ keys with blue hover highlighting
Action: Movement immediately updates Bottom Panel
Behavior: Updates in real-time as search query changes
```

## State Transition Rules

### From Default State:
- Select file → File Selected State (auto-focus Left Panel)
- Press `/` → Search Mode (focus Right Panel input)

### From File Selected State:
- Press `ESC` → Default State (focus Right Panel, clear Left Panel)
- Press `→` → Default State (focus Right Panel, keep Left Panel)
- Press `/` → Search Mode (focus Right Panel input)

### From Search Mode:
- Press `ESC` → Return to previous state before search
- Press `/` again → Default State (focus Right Panel file list)
- Clear search input → Keep search mode but clear results

## Navigation Flow

### Default Navigation Path:
```
1. User enters application
2. Focus on Right Panel (File List)
3. User selects file with Enter/Space
4. Focus automatically moves to Left Panel
5. Left Panel shows file commands, first item selected
6. Bottom Panel shows first command's code
7. User can navigate commands with ↑/↓
8. Each movement updates Bottom Panel immediately
```

### Search Navigation Path:
```
1. User presses `/` from any state
2. Focus moves to Right Panel (Search Input)
3. User types search query
4. Left Panel shows real-time results
5. User can press ↑/↓ to focus Left Panel and navigate
6. Bottom Panel updates with selected result's code
7. User presses `/` or ESC to exit search
```

## Keyboard Shortcuts Summary

| Key | Context | Action |
|-----|---------|--------|
| `↑/↓` | Any focusable panel | Navigate items with blue highlighting |
| `Enter/Space` | Right Panel (File List) | Toggle file selection (green background) |
| `Enter` | Left Panel | No action (movement is immediate) |
| `ESC` | File Selected State | Return to Default State, clear Left Panel |
| `ESC` | Search Mode | Exit search, return to previous state |
| `→` | File Selected State | Return focus to Right Panel, keep Left Panel |
| `/` | Any state | Toggle Search Mode on/off |
| `q` | Any state | Quit application |

## Error States

### No Files Found:
- Right Panel: Display "No .sh files found" message
- Left Panel: Remains empty
- Search disabled

### Search No Results:
- Left Panel: Display "No matching commands found"  
- Bottom Panel: Show helpful search tips

### File Parse Error:
- Left Panel: Display "Error parsing file" with file name
- Bottom Panel: Show error details

## Implementation References

### State Management:
```rust
enum AppState {
    Default,           // Focus: Right Panel (File List)
    FileSelected(PathBuf), // Focus: Left Panel (Commands)  
    SearchMode(String), // Focus: Right Panel (Search Input)
}

enum PanelFocus {
    Left,   // Commands panel
    Right,  // Files/Search panel
}
```

### Focus Rendering:
- Focused panels render with `Style::default().border_color(Color::White).border_type(BorderType::Thick)`
- Unfocused panels render with `Style::default().border_color(Color::Rgb(229, 233, 240)).border_type(BorderType::Plain)`

This logic specification works in conjunction with the visual design defined in `UI_DESIGN.md` to create a cohesive user experience.