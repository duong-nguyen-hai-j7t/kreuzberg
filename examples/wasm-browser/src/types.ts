/**
 * Type definitions for the Kreuzberg WASM API
 *
 * Most types are re-exported from @kreuzberg/wasm for consistency.
 */

import type { ExtractionResult } from "@kreuzberg/wasm";

// Re-export types from @kreuzberg/wasm
export type { ExtractionConfig, ExtractionResult } from "@kreuzberg/wasm";

export interface ExtractionError {
	message: string;
	code?: string;
}

export interface UIState {
	isProcessing: boolean;
	currentFile: {
		name: string;
		size: number;
		mimeType: string;
	} | null;
	results: ExtractionResult | null;
	error: ExtractionError | null;
}
