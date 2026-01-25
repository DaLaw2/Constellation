/**
 * File icon utilities for grid view.
 */

/**
 * Get emoji icon for file type based on extension.
 */
export function getFileIcon(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase()

    const iconMap: Record<string, string> = {
        // Images
        'jpg': '🖼️',
        'jpeg': '🖼️',
        'png': '🖼️',
        'gif': '🖼️',
        'webp': '🖼️',
        'bmp': '🖼️',
        'svg': '🖼️',
        // Documents
        'pdf': '📄',
        'doc': '📝',
        'docx': '📝',
        'txt': '📝',
        'md': '📝',
        // Archives
        'zip': '📦',
        'rar': '📦',
        '7z': '📦',
        'tar': '📦',
        'gz': '📦',
        // Code
        'js': '📜',
        'ts': '📜',
        'py': '📜',
        'rs': '📜',
        'java': '📜',
        'cpp': '📜',
        'c': '📜',
        'html': '📜',
        'css': '📜',
        'vue': '📜',
        // Media
        'mp4': '🎬',
        'avi': '🎬',
        'mkv': '🎬',
        'mov': '🎬',
        'mp3': '🎵',
        'wav': '🎵',
        'flac': '🎵',
        // Spreadsheets
        'xlsx': '📊',
        'xls': '📊',
        'csv': '📊',
        // Presentations
        'pptx': '📊',
        'ppt': '📊',
    }

    return iconMap[ext || ''] || '📄'
}

/**
 * Check if file is an image based on extension.
 */
export function isImageFile(filename: string): boolean {
    const ext = filename.split('.').pop()?.toLowerCase()
    return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg'].includes(ext || '')
}
