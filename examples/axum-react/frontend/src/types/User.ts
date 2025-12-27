import { z } from 'zod';

/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: UserId;
    /** User's display name */
    name: string;
    /** User's email address */
    email: string;
    /** User's age (optional) */
    age?: number | null;
}

// Zod Schema

export const UserSchema = z.object({
    id: z.unknown(),
    name: z.string().min(1).max(20),
    email: z.string(),
    age: z.number().min(1).max(100).nullable(),
});
