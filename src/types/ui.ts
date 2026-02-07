/**
 * UI types for the Constellation application.
 * These types represent UI state and display options.
 */

/** Main view mode for the left panel */
export type ViewMode = 'file-browser' | 'tag-management' | 'search'

/** Display mode for file lists */
export type DisplayMode = 'detail' | 'large-icons'

/** Search mode for tag-based searches */
export type SearchMode = 'and' | 'or'

/** Search input mode: simple filter UI or CQL query language */
export type SearchInputMode = 'simple' | 'cql'
