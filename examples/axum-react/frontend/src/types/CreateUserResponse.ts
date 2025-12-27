import { z } from 'zod';
import type { User } from './User';


/**
 * Response after creating a user
 */
export interface CreateUserResponse {
    /** The created user */
    user: User;
    /** Success message */
    message: string;
}

// Zod Schema

export const CreateUserResponseSchema = z.object({
    user: z.unknown(),
    message: z.string(),
});
