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

/**
 * Splits text into segments based on a query match for highlighting.
 * Currently supports simple substring matching (case-insensitive).
 * 
 * @param text - The text to highlight
 * @param query - The search query
 * @returns Array of segments with { text, highlight: boolean }
 */
export function getHighlightRanges(text: string, query: string): Array<{ text: string, highlight: boolean }> {
  if (!query) {
    return [{ text, highlight: false }]
  }

  const lowerText = text.toLowerCase()
  const lowerQuery = query.toLowerCase()
  const index = lowerText.indexOf(lowerQuery)

  if (index === -1) {
    return [{ text, highlight: false }]
  }

  const segments = []

  // Before match
  if (index > 0) {
    segments.push({ text: text.slice(0, index), highlight: false })
  }

  // Match
  segments.push({ text: text.slice(index, index + query.length), highlight: true })

  // After match
  if (index + query.length < text.length) {
    // Recursively highlight the rest? For now just simple single match or loop
    // Let's support multiple occurrences
    let remaining = text.slice(index + query.length)
    let remainingRanges = getHighlightRanges(remaining, query)
    segments.push(...remainingRanges)
  }

  return segments
}
