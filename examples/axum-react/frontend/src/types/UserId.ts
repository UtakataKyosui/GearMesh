import { z } from 'zod';

// Branded Type helper
type Brand<T, B> = T & { readonly __brand: B };

/**
 * User ID (Branded Type)
 */
export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;
