/**
 * Search utility functions.
 */

/**
 * Performs a fuzzy match on text against a query.
 * @param text - The text to search in
 * @param query - The search query
 * @returns True if the query matches the text
 */
export function fuzzyMatch(text: string, query: string): boolean {
  // Empty query matches everything
  if (!query) return true

  text = text.toLowerCase()
  query = query.toLowerCase()

  // Fast path for substring match
  if (text.includes(query)) return true

  // Fuzzy match (characters in order)
  let queryIdx = 0
  let textIdx = 0

  while (queryIdx < query.length && textIdx < text.length) {
    if (query[queryIdx] === text[textIdx]) {
      queryIdx++
    }
    textIdx++
  }

  return queryIdx === query.length
}
