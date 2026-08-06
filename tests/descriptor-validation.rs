//! BUG-007: ROS2 bridge 字段静默丢弃修复的集成测试
//!
//! 运行: `cargo test --test descriptor-validation`

use dora_core::descriptor::DescriptorExt;
use dora_message::descriptor::Descriptor;
use std::path::Path;

const CASES_DIR: &str = "tests/descriptor-validation/cases";

fn load_descriptor(name: &str) -> Descriptor {
    let path = Path::new(CASES_DIR).join(name);
    let yaml =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {name}: {e}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|e| panic!("failed to parse {name}: {e}"))
}

fn descriptor_should_fail(name: &str, expected_fields: &[&str]) {
    let desc = load_descriptor(name);
    let result = desc.resolve_aliases_and_set_defaults();
    match result {
        Err(e) => {
            let msg = format!("{e:#}");
            for field in expected_fields {
                assert!(
                    msg.contains(field),
                    "{name}: error should mention '{field}', got: {msg}"
                );
            }
        }
        Ok(_) => {
            panic!("{name}: expected rejection but passed");
        }
    }
}

// ═══════════════════════════════════════════════════════════
// BUG-007: ROS2 bridge — silently dropped fields
// ═══════════════════════════════════════════════════════════

#[test]
fn invalid_ros2_rejects_git() {
    descriptor_should_fail("invalid-ros2-git.yml", &["git", "not supported"]);
}

#[test]
fn invalid_ros2_rejects_metadata() {
    descriptor_should_fail(
        "invalid-ros2-metadata.yml",
        &["output_metadata", "pattern", "not supported"],
    );
}
