import { z } from 'zod';

/**
 * List of users
 */
export interface UserList {
    /** All users */
    users: User[];
    /** Total count */
    total: number;
}

// Zod Schema

export const UserListSchema = z.object({
    users: z.array(z.unknown()),
    total: z.number(),
});
