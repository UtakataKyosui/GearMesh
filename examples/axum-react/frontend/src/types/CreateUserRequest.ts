import { z } from 'zod';

/**
 * Request to create a new user
 */
export interface CreateUserRequest {
    /** Display name */
    name: string;
    /** Email address */
    email: string;
    /** Age (optional) */
    age?: number | null;
}

// Zod Schema

export const CreateUserRequestSchema = z.object({
    name: z.string().min(1).max(20),
    email: z.string(),
    age: z.number().min(1).max(100).nullable(),
});
