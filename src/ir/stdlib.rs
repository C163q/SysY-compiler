use koopa::ir::{FunctionData, Type};

pub struct FunctionDecl<'a> {
    name: &'a str,
    params_ty: Box<[Type]>,
    ret_ty: Option<Type>,
}

impl<'a> FunctionDecl<'a> {
    pub const fn new(
        name: &'a str,
        params_ty: Box<[Type]>,
        ret_ty: Option<Type>,
    ) -> FunctionDecl<'a> {
        Self {
            name,
            params_ty,
            ret_ty,
        }
    }
}

thread_local! {
    static FUNCTION_DECLS: [FunctionDecl<'static>; 8] = [
        FunctionDecl::new("getint", Box::new([]), Some(Type::get_i32())),
        FunctionDecl::new("getch", Box::new([]), Some(Type::get_i32())),
        FunctionDecl::new("getarray", Box::new([Type::get_pointer(Type::get_i32())]), Some(Type::get_i32())),
        FunctionDecl::new("putint", Box::new([Type::get_i32()]), None),
        FunctionDecl::new("putch", Box::new([Type::get_i32()]), None),
        FunctionDecl::new("putarray", Box::new([Type::get_i32(), Type::get_pointer(Type::get_i32())]), None),
        FunctionDecl::new("starttime", Box::new([]), None),
        FunctionDecl::new("stoptime", Box::new([]), None),
    ];
}

pub fn get_function_decls() -> Vec<FunctionData> {
    let mut decls = Vec::new();
    FUNCTION_DECLS.with(|funcs| {
        funcs.iter().for_each(|decl| {
            decls.push(FunctionData::new(
                format!("@{}", decl.name),
                decl.params_ty.to_vec(),
                decl.ret_ty.clone().unwrap_or(Type::get_unit()),
            ))
        })
    });
    decls
}
