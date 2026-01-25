/**
 * Domain types for the Constellation application.
 * These types represent the core business entities.
 */

/** Database item representing a tagged file or directory */
export interface Item {
  id: number
  path: string
  is_directory: boolean
  size: number | null
  modified_time: number | null
  created_at: number
  updated_at: number
}

/** Tag group for organizing tags */
export interface TagGroup {
  id: number
  name: string
  color: string | null
  display_order: number
  created_at: number
  updated_at: number
}

/** Tag that can be applied to items */
export interface Tag {
  id: number
  group_id: number
  value: string
  created_at: number
  updated_at: number
}

/** Tag template for applying multiple tags at once */
export interface TagTemplate {
  id: number
  name: string
  tag_ids: number[]
  created_at: number
  updated_at: number
}


/** Search criteria */
export interface SearchCriteria {
  tag_ids: number[]
  mode: 'and' | 'or'
  filename_query: string | null
}

/** Search history entry */
export interface SearchHistory {
  id: number
  criteria: SearchCriteria
  last_used_at: number
}
