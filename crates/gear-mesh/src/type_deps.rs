use gear_mesh_core::{GearMeshType, TypeRef};
use std::collections::HashSet;

/// Extract all custom type names referenced by a type
pub fn extract_type_dependencies(ty: &GearMeshType) -> HashSet<String> {
    let mut deps = HashSet::new();

    match &ty.kind {
        gear_mesh_core::TypeKind::Struct(s) => {
            for field in &s.fields {
                collect_type_refs(&field.ty, &mut deps);
            }
        }
        gear_mesh_core::TypeKind::Enum(e) => {
            for variant in &e.variants {
                match &variant.content {
                    gear_mesh_core::VariantContent::Tuple(types) => {
                        for ty_ref in types {
                            collect_type_refs(ty_ref, &mut deps);
                        }
                    }
                    gear_mesh_core::VariantContent::Struct(fields) => {
                        for field in fields {
                            collect_type_refs(&field.ty, &mut deps);
                        }
                    }
                    gear_mesh_core::VariantContent::Unit => {}
                }
            }
        }
        gear_mesh_core::TypeKind::Newtype(n) => {
            collect_type_refs(&n.inner, &mut deps);
        }
        _ => {}
    }

    deps
}

fn collect_type_refs(ty_ref: &TypeRef, deps: &mut HashSet<String>) {
    // Check if this is a container type (Vec, Option, etc.)
    let is_container = matches!(
        ty_ref.name.as_str(),
        "Vec" | "Option" | "Result" | "HashMap" | "HashSet" | "Box" | "Arc" | "Rc" | "Cow"
    );

    // If it's not a container and not a primitive, add it to dependencies
    if !is_container && !is_primitive(&ty_ref.name) {
        deps.insert(ty_ref.name.clone());
    }

    // Always recursively collect from generics (even for containers)
    for generic in &ty_ref.generics {
        collect_type_refs(generic, deps);
    }
}

fn is_primitive(type_name: &str) -> bool {
    matches!(
        type_name,
        "String"
            | "str"
            | "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
    )
}
