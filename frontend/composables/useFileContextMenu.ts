import { ContextMenu } from '@imengyu/vue3-context-menu'
import { useFileExplorerStore } from '../stores/fileExplorer'
import type { FileEntry } from '../stores/fileExplorer'

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
      {
        label: 'separator',
        divided: true,
      },
      {
        label: 'Add to Library',
        icon: '➕',
        onClick: () => {
          // TODO: Implement add to library
          console.log('Add to library:', entry.path)
        },
      },
      {
        label: 'Tag...',
        icon: '🏷️',
        onClick: () => {
          // TODO: Implement tagging
          console.log('Tag file:', entry.path)
        },
      },
      {
        label: 'separator',
        divided: true,
      },
      {
        label: 'Properties',
        icon: 'ℹ️',
        onClick: async () => {
          try {
            const metadata = await fileExplorerStore.getFileMetadata(entry.path)
            console.log('File metadata:', metadata)
            // TODO: Show properties dialog
          } catch (e) {
            console.error('Failed to get metadata:', e)
          }
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
