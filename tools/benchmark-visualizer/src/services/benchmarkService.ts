import type { AggregatedBenchmarkData } from '@/types/benchmark'
import { AggregatedBenchmarkDataSchema } from '@/schemas/benchmarkSchema'
import { ZodError } from 'zod'

interface BenchmarkMetadata {
  updated_at?: string
  commit?: string
  run_id?: string
  run_url?: string
}

export class BenchmarkDataService {
  private static readonly DATA_URL = './aggregated.json'
  private static readonly METADATA_URL = './metadata.json'
  private static readonly FETCH_TIMEOUT_MS = 30000 // 30 second timeout

  static async fetchData(): Promise<AggregatedBenchmarkData> {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), this.FETCH_TIMEOUT_MS)

    try {
      const response = await fetch(this.DATA_URL, { signal: controller.signal })

      if (!response.ok) {
        throw new Error(
          `Failed to fetch benchmark data: ${response.status} ${response.statusText}`
        )
      }

      let data: unknown
      try {
        data = await response.json()
      } catch (parseError) {
        const message =
          parseError instanceof SyntaxError
            ? `Failed to parse benchmark data JSON: ${parseError.message}`
            : 'Failed to parse benchmark data response'
        console.error('JSON parsing error in fetchData:', message)
        throw new Error(message)
      }

      // Validate the JSON response against the schema
      try {
        const validatedData = AggregatedBenchmarkDataSchema.parse(data)
        return validatedData
      } catch (error) {
        if (error instanceof ZodError) {
          const issues = error.issues
            .map(
              (issue) =>
                `Path: ${issue.path.join('.')} | Code: ${issue.code} | Message: ${issue.message}`
            )
            .join('\n')

          throw new Error(`Benchmark data validation failed:\n${issues}`)
        }
        throw error
      }
    } catch (error) {
      // Differentiate between timeout and network errors
      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          const message = `Benchmark data fetch timeout after ${this.FETCH_TIMEOUT_MS}ms`
          console.error(message)
          throw new Error(message)
        }
        // Re-throw with better context for network errors
        if (error.message.includes('Failed to fetch')) {
          console.error('Network error fetching benchmark data:', error.message)
          throw new Error(`Network error fetching benchmark data: ${error.message}`)
        }
      }
      throw error
    } finally {
      clearTimeout(timeoutId)
    }
  }

  static async fetchMetadata(): Promise<BenchmarkMetadata | null> {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), this.FETCH_TIMEOUT_MS)

    try {
      const response = await fetch(this.METADATA_URL, { signal: controller.signal })

      // Handle 404 specifically
      if (response.status === 404) {
        console.warn('Metadata file not found (404): metadata.json may not exist yet')
        return null
      }

      if (!response.ok) {
        const errorMessage = `Failed to fetch metadata: ${response.status} ${response.statusText}`
        console.error(errorMessage)
        throw new Error(errorMessage)
      }

      let metadata: unknown
      try {
        metadata = await response.json()
      } catch (parseError) {
        const message =
          parseError instanceof SyntaxError
            ? `Failed to parse metadata JSON: ${parseError.message}`
            : 'Failed to parse metadata response'
        console.error('JSON parsing error in fetchMetadata:', message)
        throw new Error(message)
      }

      return metadata as BenchmarkMetadata
    } catch (error) {
      // Differentiate between timeout, network errors, and other failures
      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          const message = `Metadata fetch timeout after ${this.FETCH_TIMEOUT_MS}ms`
          console.error(message)
          return null
        }

        // Network errors
        if (error.message.includes('Failed to fetch')) {
          console.error('Network error fetching metadata:', error.message)
          return null
        }

        // JSON parsing errors already logged above
        if (error.message.includes('Failed to parse')) {
          return null
        }
      }

      // Log unexpected errors
      console.error('Unexpected error in fetchMetadata:', error)
      return null
    } finally {
      clearTimeout(timeoutId)
    }
  }
}
