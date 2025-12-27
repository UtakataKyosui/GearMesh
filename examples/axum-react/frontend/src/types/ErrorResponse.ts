import { z } from 'zod';

/**
 * Error response
 */
export interface ErrorResponse {
    /** Error message */
    error: string;
}

// Zod Schema

export const ErrorResponseSchema = z.object({
    error: z.string(),
});
