use gear_mesh::GearMesh;

/// ユーザーの活動ログ
/// 
/// IDやタイムスタンプなど、64bit整数を多く含みます。
#[derive(GearMesh, Clone, Debug)]
pub struct ActivityLog {
    /// ログID (自動的にBigIntになります)
    pub id: u64,
    /// ユーザーID (usizeもBigIntになります)
    pub user_id: usize,
    /// アクション種別
    pub action: String,
    /// タイムスタンプ
    pub timestamp: i64,
    /// スコア (範囲制限付き)
    #[gear_mesh(validate(range(min = 0, max = 1000000000)))]
    pub score: u64,
    /// タグリスト (配列のテスト)
    pub tags: Vec<String>,
    /// メモ (オプショナルのテスト)
    pub memo: Option<String>,
}

fn main() {
    // gear_mesh::generate_types_to_dir を使用して型定義を生成
    // 内部でGeneratorConfig::new()が呼ばれ、use_bigint=trueがデフォルトで有効になります
    gear_mesh::generate_types_to_dir("generated")
        .expect("Failed to generate TypeScript types");
}
