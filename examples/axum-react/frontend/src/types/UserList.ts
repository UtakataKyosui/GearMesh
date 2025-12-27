import { z } from 'zod';
import type { User } from './User';
import { UserSchema } from './User';


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
    users: z.array(UserSchema),
    total: z.number(),
});
