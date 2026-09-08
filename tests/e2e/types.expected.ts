// Branded Type helper
type Brand<T, B> = T & { readonly __brand: B };

export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;

export type ProductId = Brand<number, "ProductId">;
export const ProductId = (value: number): ProductId => value as ProductId;

export type Status = "Pending" | "Complete";

/**
 * A record shared with a TypeScript consumer.
 */
export interface User {
    id: UserId;
    displayName: string;
    /** Missing values are represented by null, not an absent key. */
    nickname: string | null;
    tags: string[];
    balance: bigint;
    active: boolean;
    status: Status;
}

