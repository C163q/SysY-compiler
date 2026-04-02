use std::cell::Ref;

use koopa::ir::{Program, Type, TypeKind, Value, ValueKind, entities::ValueData};

use crate::asm::{
    expr,
    inst::{self, InstContext},
    meta::{self, FunctionContext, RV32Imm, Register, RiscvAsm},
};

pub fn register_global_var(program: &Program) -> Vec<RiscvAsm> {
    let mut insts = vec![];

    for &var in program.inst_layout() {
        let data = program.borrow_value(var);
        let name = &data
            .name()
            .as_ref()
            .expect("Global variable should have a name")[1..];
        insts.push(RiscvAsm::Global(name.to_string()));
        insts.push(RiscvAsm::Label(name.to_string()));
        match data.kind() {
            ValueKind::GlobalAlloc(alloc) => {
                let init_data = program.borrow_value(alloc.init());
                insts.extend(init_global_var(init_data, program));
            }
            _ => unreachable!("Global variable should be a global alloc"),
        }
        insts.push(RiscvAsm::None);
    }

    insts
}

pub fn init_global_var(init_data: Ref<ValueData>, program: &Program) -> Vec<RiscvAsm> {
    let mut insts = vec![];
    match init_data.kind() {
        ValueKind::ZeroInit(_) => {
            let zero = meta::RiscvInit::Zero(init_data.ty().size() as meta::RV32Usize);
            let init = meta::RiscvAsm::Init(zero);
            insts.push(init);
        }
        ValueKind::Integer(val) => {
            let val = meta::RiscvInit::Word(meta::RV32Imm::new(val.value()));
            let init = meta::RiscvAsm::Init(val);
            insts.push(init);
        }
        ValueKind::Aggregate(agg) => {
            // agg.len() * elem_ty.size() == alloc_ty.size()
            for &val in agg.elems() {
                let val_data = program.borrow_value(val);
                insts.extend(init_global_var(val_data, program));
            }
        }
        _ => unreachable!("Global variable should be initialized"),
    }
    insts
}

pub struct LoadContext {
    pub id: Option<Value>,
    pub ty: Type,
}

impl LoadContext {
    pub fn new(ty: Type) -> Self {
        Self { id: None, ty }
    }

    pub fn with_id(mut self, id: Value) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_ty(mut self, ty: Type) -> Self {
        self.ty = ty;
        self
    }
}

pub fn load(
    src: Value,
    context: &mut FunctionContext,
    cfg: LoadContext,
) -> Result<(Vec<RiscvAsm>, Register), String> {
    if src.is_global() {
        load_from_global(src, context, cfg.id)
    } else {
        load_from_local(src, context, cfg.id, cfg.ty)
    }
}

pub fn load_from_local(
    src: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
    ty: Type,
) -> Result<(Vec<RiscvAsm>, Register), String> {
    let mut asms = vec![];

    let rd = expr::obtain_caller_directly_usable_register(context);
    let offset = context
        .memory_mapper
        .get_offset(&src)
        .ok_or(format!("Value {:?} is not mapped to stack memory", src))?;

    let src_level = get_ptr_level(src, context);
    let dst_level = get_ptr_level_from_ty(ty.kind().clone());

    if dst_level != src_level && dst_level + 1 != src_level {
        panic!(
            "Cannot load a pointer of level {} from a pointer of level {}",
            src_level, dst_level
        );
    }

    // For example, load i32 to i32 or load *i32 to *i32
    if dst_level == src_level {
        asms.extend(inst::add_lw_instruction(
            rd,
            Register::Sp,
            RV32Imm::new(offset.offset() as i32),
            id.map(|v| InstContext::new(context, v)),
            Some(rd),
        ));
        return Ok((asms, rd));
    }

    // dst_level + 1 == src_level, for example, load *i32 to i32
    // First dereference the pointer to get the address, then load the value from the address
    asms.extend(inst::add_lw_instruction(
        rd,
        Register::Sp,
        RV32Imm::new(offset.offset() as i32),
        None,
        Some(rd),
    ));
    asms.push(inst::lw_instruction(
        rd,
        rd,
        0,
        id.map(|v| InstContext::new(context, v)),
    ));

    Ok((asms, rd))
}

pub fn load_from_global(
    src: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> Result<(Vec<RiscvAsm>, Register), String> {
    let data = context.program.borrow_value(src);
    let mut asms = vec![];

    let rd = expr::obtain_caller_directly_usable_register(context);
    asms.push(inst::la_instruction(
        rd,
        &data
            .name()
            .as_ref()
            .ok_or("Global variable should have a name")?[1..],
        None,
    ));
    asms.push(inst::lw_instruction(
        rd,
        rd,
        0,
        id.map(|v| InstContext::new(context, v)),
    ));
    inst::register_dest(rd, id.map(|v| InstContext::new(context, v)));

    Ok((asms, rd))
}

pub struct StoreContext {
    pub id: Option<Value>,
    pub claim: bool,
    pub ty: Type,
}

impl StoreContext {
    pub fn new(ty: Type) -> Self {
        Self {
            id: None,
            claim: false,
            ty,
        }
    }

    pub fn with_id(mut self, id: Value) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_claim(mut self, claim: bool) -> Self {
        self.claim = claim;
        self
    }

    pub fn with_ty(mut self, ty: Type) -> Self {
        self.ty = ty;
        self
    }
}

pub fn store(
    src: Register,
    target: Value,
    context: &mut FunctionContext,
    cfg: StoreContext,
) -> Result<Vec<RiscvAsm>, String> {
    if target.is_global() {
        store_to_global(src, target, context, cfg.id)
    } else {
        store_to_local(src, target, context, cfg.id, cfg.claim, cfg.ty)
    }
}

pub fn store_to_global(
    src: Register,
    dest: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> Result<Vec<RiscvAsm>, String> {
    let data = context.program.borrow_value(dest);
    let mut asms = vec![];
    let name = data
        .name()
        .as_ref()
        .ok_or("Global variable should have a name")?[1..]
        .to_string();

    let tmp = expr::obtain_caller_directly_usable_register(context);
    asms.extend(inst::label_sw_instruction(
        src,
        &name,
        id.map(|v| InstContext::new(context, v)),
        Some(tmp),
    ));
    Ok(asms)
}

pub fn store_to_local(
    src: Register,
    target: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
    claim: bool,
    ty: Type,
) -> Result<Vec<RiscvAsm>, String> {
    let mut asms = vec![];
    let tmp = expr::obtain_caller_directly_usable_register(context);
    if claim {
        context.memory_mapper.stack_claim(target, ty.clone());
    }
    let offset = context
        .memory_mapper
        .get_offset(&target)
        .ok_or(format!("Value {:?} is not mapped to stack memory", target))?;

    let src_level = get_ptr_level_from_ty(ty.kind().clone());
    let dst_level = get_ptr_level(target, context);

    if src_level != dst_level && src_level + 1 != dst_level {
        panic!(
            "Cannot store a pointer of level {} to a pointer of level {}",
            src_level, dst_level
        );
    }

    // For example, store i32 to i32 or store *i32 to *i32
    if src_level == dst_level {
        asms.extend(inst::add_sw_instruction(
            src,
            Register::Sp,
            RV32Imm::new(offset.offset() as i32),
            id.map(|v| InstContext::new(context, v)),
            Some(tmp),
        ));
        return Ok(asms);
    }

    // src_level + 1 == dst_level, for example, store i32 to *i32
    // First dereference the pointer to get the address, then store the value to the address
    asms.extend(inst::add_lw_instruction(
        tmp,
        Register::Sp,
        RV32Imm::new(offset.offset() as i32),
        None,
        Some(tmp),
    ));
    asms.push(inst::sw_instruction(
        src,
        tmp,
        0,
        id.map(|v| InstContext::new(context, v)),
    ));
    Ok(asms)
}

pub fn get_ptr_level_from_ty(ty: TypeKind) -> u32 {
    match ty {
        TypeKind::Pointer(base) => 1 + get_ptr_level_from_ty(base.kind().clone()),
        TypeKind::Array(base, _) => 1 + get_ptr_level_from_ty(base.kind().clone()),
        _ => 0,
    }
}

pub fn get_ptr_level(val: Value, context: &FunctionContext) -> u32 {
    let ty = if val.is_global() {
        context.program.borrow_value(val).ty().kind().clone()
    } else {
        context.func_data.dfg().value(val).ty().kind().clone()
    };
    get_ptr_level_from_ty(ty)
}

pub fn get_ptr_base_ty(ty_ptr: &Type) -> &Type {
    match ty_ptr.kind() {
        TypeKind::Pointer(ty) => ty,
        _ => panic!("Expected to be pointer, but found {:?}", ty_ptr),
    }
}

pub fn get_value_ty(value: Value, context: &FunctionContext) -> Type {
    if value.is_global() {
        context.program.borrow_value(value).ty().clone()
    } else {
        context.func_data.dfg().value(value).ty().clone()
    }
}
