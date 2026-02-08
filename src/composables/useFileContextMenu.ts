import ContextMenu from '@imengyu/vue3-context-menu'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import type { FileEntry } from '@/types'

export interface ContextMenuOptions {
  entry: FileEntry
  x: number
  y: number
}

export function useFileContextMenu() {
  const fileExplorerStore = useFileExplorerStore()

  function showFileContextMenu(options: ContextMenuOptions) {
    const { entry, x, y } = options

    const menuItems = [
      {
        label: 'Open',
        icon: '📂',
        onClick: () => {
          if (entry.is_directory) {
            fileExplorerStore.navigateTo(entry.path)
          } else {
            fileExplorerStore.openFileExternal(entry.path)
          }
        },
      },
      {
        label: 'Open with...',
        icon: '⚙️',
        disabled: entry.is_directory,
        onClick: () => {
          fileExplorerStore.openFileExternal(entry.path)
        },
      },
      {
        label: 'Show in Explorer',
        icon: '📁',
        onClick: () => {
          fileExplorerStore.revealInExplorer(entry.path)
        },
      },
    ]

    ContextMenu.showContextMenu({
      x,
      y,
      items: menuItems,
      theme: 'default',
      zIndex: 1000,
      minWidth: 180,
      customClass: 'file-context-menu',
    })
  }

  return {
    showFileContextMenu,
  }
}
