//! 型定義の中間表現
//!
//! Rust型をTypeScriptに変換する際の、言語非依存な中間形式を定義します。

use serde::{Deserialize, Serialize};

/// gear-mesh型の中間表現
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearMeshType {
    /// 型の名前
    pub name: String,
    /// 型の種類
    pub kind: TypeKind,
    /// ドキュメントコメント
    pub docs: Option<crate::DocComment>,
    /// ジェネリクスパラメータ
    pub generics: Vec<GenericParam>,
    /// 属性フラグ
    pub attributes: TypeAttributes,
}

/// 型の種類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeKind {
    /// プリミティブ型 (i32, String, etc.)
    Primitive(PrimitiveType),
    /// 構造体
    Struct(StructType),
    /// 列挙型
    Enum(EnumType),
    /// 新型パターン (tuple struct with single field)
    Newtype(NewtypeType),
    /// タプル
    Tuple(Vec<TypeRef>),
    /// 配列/Vec
    Array(Box<TypeRef>),
    /// `Option<T>`
    Option(Box<TypeRef>),
    /// `Result<T, E>`
    Result { ok: Box<TypeRef>, err: Box<TypeRef> },
    /// `HashMap<K, V>`
    Map {
        key: Box<TypeRef>,
        value: Box<TypeRef>,
    },
    /// 型参照
    Reference(TypeRef),
}

/// プリミティブ型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    // 整数型
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    // 浮動小数点
    F32,
    F64,
    // その他
    Bool,
    Char,
    String,
    // Unit
    Unit,
}

impl PrimitiveType {
    /// この型がBigIntとして扱われるべきかどうか
    pub fn is_bigint(&self) -> bool {
        matches!(
            self,
            PrimitiveType::I64
                | PrimitiveType::I128
                | PrimitiveType::U64
                | PrimitiveType::U128
                | PrimitiveType::Isize
                | PrimitiveType::Usize
        )
    }

    /// TypeScriptでの型名を取得
    pub fn typescript_type(&self, use_bigint: bool) -> &'static str {
        match self {
            PrimitiveType::I8
            | PrimitiveType::I16
            | PrimitiveType::I32
            | PrimitiveType::U8
            | PrimitiveType::U16
            | PrimitiveType::U32
            | PrimitiveType::F32
            | PrimitiveType::F64 => "number",

            PrimitiveType::I64
            | PrimitiveType::I128
            | PrimitiveType::U64
            | PrimitiveType::U128
            | PrimitiveType::Isize
            | PrimitiveType::Usize => {
                if use_bigint {
                    "bigint"
                } else {
                    "number"
                }
            }

            PrimitiveType::Bool => "boolean",
            PrimitiveType::Char | PrimitiveType::String => "string",
            PrimitiveType::Unit => "null",
        }
    }
}

/// 構造体型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructType {
    /// フィールド一覧
    pub fields: Vec<FieldInfo>,
}

/// フィールド情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// フィールド名
    pub name: String,
    /// フィールドの型
    pub ty: TypeRef,
    /// ドキュメントコメント
    pub docs: Option<crate::DocComment>,
    /// バリデーションルール
    pub validations: Vec<crate::ValidationRule>,
    /// オプショナルかどうか
    pub optional: bool,
    /// serde属性
    pub serde_attrs: SerdeFieldAttrs,
}

/// serdeフィールド属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerdeFieldAttrs {
    /// リネーム
    pub rename: Option<String>,
    /// スキップ
    pub skip: bool,
    /// デフォルト値
    pub default: bool,
    /// flatten
    pub flatten: bool,
}

/// 列挙型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumType {
    /// バリアント一覧
    pub variants: Vec<EnumVariant>,
    /// タグ付け方式
    pub representation: EnumRepresentation,
}

/// 列挙型のバリアント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    /// バリアント名
    pub name: String,
    /// バリアントの内容
    pub content: VariantContent,
    /// ドキュメントコメント
    pub docs: Option<crate::DocComment>,
}

/// バリアントの内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantContent {
    /// ユニットバリアント
    Unit,
    /// タプルバリアント
    Tuple(Vec<TypeRef>),
    /// 構造体バリアント
    Struct(Vec<FieldInfo>),
}

/// 列挙型の表現方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumRepresentation {
    /// 外部タグ (デフォルト): { "Variant": data }
    External,
    /// 内部タグ: { "type": "Variant", ...data }
    Internal { tag: String },
    /// 隣接タグ: { "type": "Variant", "content": data }
    Adjacent { tag: String, content: String },
    /// タグなし
    Untagged,
}

/// 新型パターン
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewtypeType {
    /// 内部の型
    pub inner: TypeRef,
}

/// 型参照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRef {
    /// 型名
    pub name: String,
    /// ジェネリクス引数
    pub generics: Vec<TypeRef>,
}

impl TypeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            generics: Vec::new(),
        }
    }

    pub fn with_generics(name: impl Into<String>, generics: Vec<TypeRef>) -> Self {
        Self {
            name: name.into(),
            generics,
        }
    }
}

/// ジェネリクスパラメータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericParam {
    /// パラメータ名
    pub name: String,
    /// 制約
    pub bounds: Vec<String>,
}

/// 型の属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeAttributes {
    /// Branded Typeとして生成するか
    pub branded: bool,
    /// バリデーションを生成するか
    pub validate: bool,
    /// BigInt自動変換を有効にするか
    pub bigint_auto: bool,
    /// serde属性
    pub serde: SerdeTypeAttrs,
    /// Output path for automatic TypeScript generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_path: Option<String>,
}

/// serde型属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerdeTypeAttrs {
    /// リネーム規則
    pub rename_all: Option<RenameRule>,
    /// タグ
    pub tag: Option<String>,
    /// コンテンツ
    pub content: Option<String>,
    /// タグなし
    pub untagged: bool,
}

/// リネームルール
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenameRule {
    /// lowercase
    Lowercase,
    /// UPPERCASE
    Uppercase,
    /// camelCase
    CamelCase,
    /// snake_case
    SnakeCase,
    /// PascalCase
    PascalCase,
    /// SCREAMING_SNAKE_CASE
    ScreamingSnakeCase,
    /// kebab-case
    KebabCase,
    /// SCREAMING-KEBAB-CASE
    ScreamingKebabCase,
}

impl RenameRule {
    /// 名前を変換する
    pub fn apply(&self, name: &str) -> String {
        match self {
            RenameRule::Lowercase => name.to_lowercase(),
            RenameRule::Uppercase => name.to_uppercase(),
            RenameRule::CamelCase => to_camel_case(name),
            RenameRule::SnakeCase => to_snake_case(name),
            RenameRule::PascalCase => to_pascal_case(name),
            RenameRule::ScreamingSnakeCase => to_snake_case(name).to_uppercase(),
            RenameRule::KebabCase => to_snake_case(name).replace('_', "-"),
            RenameRule::ScreamingKebabCase => to_snake_case(name).to_uppercase().replace('_', "-"),
        }
    }
}

// ヘルパー関数
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else if i == 0 {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn to_pascal_case(s: &str) -> String {
    let camel = to_camel_case(s);
    let mut chars = camel.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_rules() {
        assert_eq!(RenameRule::CamelCase.apply("user_name"), "userName");
        assert_eq!(RenameRule::SnakeCase.apply("userName"), "user_name");
        assert_eq!(RenameRule::PascalCase.apply("user_name"), "UserName");
        assert_eq!(RenameRule::KebabCase.apply("userName"), "user-name");
        assert_eq!(
            RenameRule::ScreamingSnakeCase.apply("userName"),
            "USER_NAME"
        );
        assert_eq!(RenameRule::Lowercase.apply("UserName"), "username");
        assert_eq!(RenameRule::Uppercase.apply("userName"), "USERNAME");
    }

    #[test]
    fn test_primitive_bigint() {
        assert!(PrimitiveType::U64.is_bigint());
        assert!(PrimitiveType::I64.is_bigint());
        assert!(PrimitiveType::U128.is_bigint());
        assert!(PrimitiveType::I128.is_bigint());
        assert!(!PrimitiveType::I32.is_bigint());
        assert!(!PrimitiveType::U32.is_bigint());
    }

    #[test]
    fn test_primitive_typescript_type() {
        assert_eq!(PrimitiveType::I32.typescript_type(false), "number");
        assert_eq!(PrimitiveType::U64.typescript_type(true), "bigint");
        assert_eq!(PrimitiveType::U64.typescript_type(false), "number");
        assert_eq!(PrimitiveType::Bool.typescript_type(false), "boolean");
        assert_eq!(PrimitiveType::String.typescript_type(false), "string");
        assert_eq!(PrimitiveType::Unit.typescript_type(false), "null");
    }

    #[test]
    fn test_type_ref_creation() {
        let type_ref = TypeRef::new("String");
        assert_eq!(type_ref.name, "String");
        assert!(type_ref.generics.is_empty());

        let generic_ref = TypeRef::with_generics("Vec", vec![TypeRef::new("i32")]);
        assert_eq!(generic_ref.name, "Vec");
        assert_eq!(generic_ref.generics.len(), 1);
        assert_eq!(generic_ref.generics[0].name, "i32");
    }

    #[test]
    fn test_struct_type_serialization() {
        let struct_type = StructType {
            fields: vec![FieldInfo {
                name: "id".to_string(),
                ty: TypeRef::new("i32"),
                docs: None,
                validations: vec![],
                optional: false,
                serde_attrs: SerdeFieldAttrs::default(),
            }],
        };

        let json = serde_json::to_string(&struct_type).unwrap();
        let deserialized: StructType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.fields.len(), 1);
        assert_eq!(deserialized.fields[0].name, "id");
    }

    #[test]
    fn test_type_attributes_default() {
        let attrs = TypeAttributes::default();
        assert!(!attrs.branded);
        assert!(!attrs.validate);
        assert!(!attrs.bigint_auto);
    }
}
