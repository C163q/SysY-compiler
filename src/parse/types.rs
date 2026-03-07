use koopa::ir::{Type, TypeKind};

/// 将字符串转换为 Koopa IR 中的 Type。
pub fn get_type(s: &str) -> Type {
    match s {
        "int" => Type::get(TypeKind::Int32),
        "void" => Type::get(TypeKind::Unit),
        _ => unimplemented!(),
    }
}
