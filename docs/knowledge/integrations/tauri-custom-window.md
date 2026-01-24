# Tauri 2.x Custom Window (Frameless/Custom Title Bar)

## Overview

Tauri allows removing the default OS window decorations and implementing a custom title bar using HTML/CSS/JS.

## Configuration

### tauri.conf.json

```json
{
  "app": {
    "windows": [
      {
        "title": "App Name",
        "width": 1280,
        "height": 720,
        "decorations": false,
        "transparent": true,
        "resizable": true
      }
    ]
  }
}
```

| Option | Description |
|--------|-------------|
| `decorations: false` | Removes OS title bar and window frame |
| `transparent: true` | Enables transparent/rounded corners (optional) |
| `resizable: true` | May not work on Windows when decorations=false (known issue) |

## Custom Title Bar Implementation (Vue 3)

### Component: TitleBar.vue

```vue
<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-icon">
      <!-- App icon -->
    </div>
    <span class="titlebar-title" data-tauri-drag-region>App Name</span>
    <div class="titlebar-controls">
      <button class="titlebar-btn" @click="minimize" aria-label="Minimize">
        <MinusIcon />
      </button>
      <button class="titlebar-btn" @click="toggleMaximize" aria-label="Maximize">
        <SquareIcon />
      </button>
      <button class="titlebar-btn titlebar-btn-close" @click="close" aria-label="Close">
        <XIcon />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()

const minimize = () => appWindow.minimize()
const toggleMaximize = () => appWindow.toggleMaximize()
const close = () => appWindow.close()
</script>

<style scoped>
.titlebar {
  height: 32px;
  background: var(--color-bg-secondary);
  display: flex;
  align-items: center;
  justify-content: space-between;
  user-select: none;
  -webkit-user-select: none;
  -webkit-app-region: drag;
}

.titlebar-controls {
  display: flex;
  -webkit-app-region: no-drag;
}

.titlebar-btn {
  width: 46px;
  height: 32px;
  border: none;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.titlebar-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.titlebar-btn-close:hover {
  background: #e81123;
  color: white;
}
</style>
```

## Key Concepts

### data-tauri-drag-region

- Add to elements that should drag the window when clicked
- **Does NOT inherit** to child elements
- Must be added to each draggable element individually
- Buttons inside drag region work normally (no need for no-drag)

```html
<!-- Correct: drag region on parent, buttons work automatically -->
<div data-tauri-drag-region>
  <span data-tauri-drag-region>Title</span>
  <button>Close</button> <!-- This works, no conflict -->
</div>
```

### Manual Drag (Alternative)

If `data-tauri-drag-region` doesn't work in certain cases:

```typescript
import { getCurrentWindow } from '@tauri-apps/api/window'

const startDrag = async (e: MouseEvent) => {
  if (e.buttons === 1) { // Left mouse button
    await getCurrentWindow().startDragging()
  }
}
```

```html
<div @mousedown="startDrag">Draggable Area</div>
```

## Known Issues (Tauri 2.x)

### Windows Resize Bug

When `decorations: false`, window may not be resizable even with `resizable: true`.

**Workaround**: Implement custom resize handles:

```vue
<template>
  <div class="resize-handle resize-right" @mousedown="startResize('right')"></div>
  <div class="resize-handle resize-bottom" @mousedown="startResize('bottom')"></div>
  <div class="resize-handle resize-corner" @mousedown="startResize('corner')"></div>
</template>

<style>
.resize-handle {
  position: fixed;
  background: transparent;
}
.resize-right {
  right: 0;
  top: 0;
  width: 4px;
  height: 100%;
  cursor: ew-resize;
}
.resize-bottom {
  bottom: 0;
  left: 0;
  width: 100%;
  height: 4px;
  cursor: ns-resize;
}
.resize-corner {
  right: 0;
  bottom: 0;
  width: 12px;
  height: 12px;
  cursor: nwse-resize;
}
</style>
```

### Drag Not Working When Window Unfocused

Window must be focused before dragging works. No known workaround.

### Windows Snap Overlay

Windows 11 snap layouts (hover over maximize) don't work with custom title bar.

**Plugin**: `tauri-plugin-decorum` can help, but limited support.

## API Reference

```typescript
import { getCurrentWindow } from '@tauri-apps/api/window'

const win = getCurrentWindow()

// Window controls
await win.minimize()
await win.maximize()
await win.unmaximize()
await win.toggleMaximize()
await win.close()

// State queries
const isMaximized = await win.isMaximized()
const isMinimized = await win.isMinimized()
const isFocused = await win.isFocused()

// Listen to state changes
await win.onResized(({ payload }) => {
  console.log('New size:', payload.width, payload.height)
})

await win.listen('tauri://focus', () => {
  console.log('Window focused')
})
```

## References

- [Tauri v2 Window Customization](https://v2.tauri.app/learn/window-customization/)
- [Tauri Window API](https://v2.tauri.app/reference/javascript/api/namespacewindow/)
- [GitHub Discussion: Custom Titlebar](https://github.com/tauri-apps/tauri/discussions/3093)
- [GitHub Issue: Resize Bug](https://github.com/tauri-apps/tauri/issues/8519)
