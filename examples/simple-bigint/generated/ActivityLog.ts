import { z } from 'zod';

/**
 * ユーザーの活動ログ
 *
 * IDやタイムスタンプなど、64bit整数を多く含みます。
 */
export interface ActivityLog {
    /** ログID (自動的にBigIntになります) */
    id: bigint;
    /** ユーザーID (usizeもBigIntになります) */
    user_id: bigint;
    /** アクション種別 */
    action: string;
    /** タイムスタンプ */
    timestamp: bigint;
    /** スコア (範囲制限付き) */
    score: bigint;
    /** タグリスト (配列のテスト) */
    tags: string[];
    /** メモ (オプショナルのテスト) */
    memo?: string | null;
}

// Zod Schema

export const ActivityLogSchema = z.object({
    id: z.bigint(),
    user_id: z.bigint(),
    action: z.string(),
    timestamp: z.bigint(),
    score: z.bigint(),
    tags: z.array(z.string()),
    memo: z.string().nullable(),
});
