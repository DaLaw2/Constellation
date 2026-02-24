/**
 * Application constants for the Constellation application.
 */

// Layout constants
export const LAYOUT = {
  /** Height of each file item in the virtual scroller */
  FILE_ITEM_HEIGHT: 60,
  /** Minimum width of the tag area in file list */
  MIN_TAG_AREA_WIDTH: 400,
  /** Default width of the tag area in file list */
  DEFAULT_TAG_AREA_WIDTH: 500,
  /** Base height of a grid file card row at 100% zoom */
  GRID_ROW_HEIGHT: 220,
  /** Minimum grid card width (matches GridView baseSize) */
  GRID_MIN_CARD_WIDTH: 150,
  /** Default grid gap in pixels */
  GRID_GAP: 16,
  /** Height of a picture grid row */
  PICTURE_ROW_HEIGHT: 250,
  /** Minimum picture card width */
  PICTURE_MIN_CARD_WIDTH: 200,
} as const

// Tag display constants
export const TAG_DISPLAY = {
  /** Maximum width of a single tag badge */
  MAX_TAG_WIDTH: 120,
  /** Minimum width of a truncated tag badge */
  MIN_TAG_WIDTH: 45,
  /** Width of overflow indicator (+N) */
  OVERFLOW_BADGE_WIDTH: 35,
  /** Gap between tag badges */
  TAG_GAP: 4,
  /** Default visible tag count */
  DEFAULT_VISIBLE_COUNT: 3,
} as const

// Timing constants
export const TIMING = {
  /** Debounce delay for search input */
  SEARCH_DEBOUNCE_MS: 300,
  /** Debounce delay for resize operations */
  RESIZE_DEBOUNCE_MS: 100,
} as const

// Storage keys
export const STORAGE_KEYS = {
  /** Local storage key for tag area width */
  TAG_AREA_WIDTH: 'fileList.tagAreaWidth',
} as const
