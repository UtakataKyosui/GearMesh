import { z } from 'zod';

// Branded Type helper
type Brand<T, B> = T & { readonly __brand: B };

/**
 * Error response
 */
export interface ErrorResponse {
    /** Error message */
    error: string;
}

/**
 * List of users
 */
export interface UserList {
    /** All users */
    users: User[];
    /** Total count */
    total: number;
}

/**
 * Response after creating a user
 */
export interface CreateUserResponse {
    /** The created user */
    user: User;
    /** Success message */
    message: string;
}

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

/**
 * User ID (Branded Type)
 */
export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;

// Zod Schemas

export const ErrorResponseSchema = z.object({
    error: z.string(),
});

export const UserListSchema = z.object({
    users: z.unknown(),
    total: z.number(),
});

export const CreateUserResponseSchema = z.object({
    user: z.unknown(),
    message: z.string(),
});

export const CreateUserRequestSchema = z.object({
    name: z.string(),
    email: z.string(),
    age: z.unknown().nullable(),
});

export const UserSchema = z.object({
    id: z.unknown(),
    name: z.string().min(1).max(20),
    email: z.string(),
    age: z.unknown().nullable(),
});

