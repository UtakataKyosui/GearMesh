import { ProductId, UserId, type User } from "./types";

export const user: User = {
    id: UserId(1),
    displayName: "Alice",
    nickname: null,
    tags: ["rust", "typescript"],
    balance: 100n,
    active: true,
    status: "Pending",
};

export const namedUser: User = { ...user, nickname: "Ali" };

// These must remain errors; unused directives fail the type check too.
// @ts-expect-error Distinct brands cannot be assigned to each other.
const wrongBrand: UserId = ProductId(1);
// @ts-expect-error Large Rust integers map to bigint, not number.
const wrongBalance: User["balance"] = 100;
// @ts-expect-error Nullable does not mean undefined.
const undefinedNickname: User["nickname"] = undefined;
// @ts-expect-error A nullable field is still a required key.
const missingNickname: User = { id: UserId(1), displayName: "Alice", tags: [], balance: 0n, active: true, status: "Pending" };
// @ts-expect-error Enum variants are a closed union.
const wrongStatus: User["status"] = "Unknown";
