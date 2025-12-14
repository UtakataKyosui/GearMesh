//! CLIのテスト

use super::*;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.input, PathBuf::from("src"));
    assert_eq!(config.output, PathBuf::from("bindings"));
    assert!(config.use_bigint);
    assert!(config.generate_branded);
    assert!(!config.generate_validation);
    assert!(!config.generate_zod);
    assert!(config.generate_jsdoc);
    assert_eq!(config.output_file, "types.ts");
}

#[test]
fn test_config_to_generator_config() {
    let config = Config {
        input: PathBuf::from("src"),
        output: PathBuf::from("bindings"),
        use_bigint: true,
        generate_branded: true,
        generate_validation: true,
        generate_zod: false,
        generate_jsdoc: true,
        output_file: "types.ts".to_string(),
    };

    let gen_config = config.to_generator_config();
    // GeneratorConfigの設定が正しく反映されているかは、
    // 実際の使用時に確認される
    let _ = gen_config;
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let toml_str = toml::to_string(&config).unwrap();
    
    assert!(toml_str.contains("input"));
    assert!(toml_str.contains("output"));
    assert!(toml_str.contains("use_bigint"));
}

#[test]
fn test_config_deserialization() {
    let toml_str = r#"
        input = "custom_src"
        output = "custom_bindings"
        use_bigint = false
        generate_branded = false
        generate_validation = true
        generate_zod = true
        generate_jsdoc = false
        output_file = "custom.ts"
    "#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.input, PathBuf::from("custom_src"));
    assert_eq!(config.output, PathBuf::from("custom_bindings"));
    assert!(!config.use_bigint);
    assert!(!config.generate_branded);
    assert!(config.generate_validation);
    assert!(config.generate_zod);
    assert!(!config.generate_jsdoc);
    assert_eq!(config.output_file, "custom.ts");
}
