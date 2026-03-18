use std::cell::Ref;

use koopa::ir::{Program, Value, ValueKind, entities::ValueData};

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
                insts.extend(init_global_var(init_data));
            }
            _ => unreachable!("Global variable should be a global alloc"),
        }
        insts.push(RiscvAsm::None);
    }

    insts
}

pub fn init_global_var(init_data: Ref<ValueData>) -> Vec<RiscvAsm> {
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
        _ => unreachable!("Global variable should be initialized"),
    }
    insts
}

pub fn load(
    src: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> Result<(Vec<RiscvAsm>, Register), String> {
    if src.is_global() {
        load_from_global(src, context, id)
    } else {
        load_from_local(src, context, id)
    }
}

pub fn load_from_local(
    src: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
) -> Result<(Vec<RiscvAsm>, Register), String> {
    let size = context.func_data.dfg().value(src).ty().size() as u32;
    let mut asms = vec![];

    let rd = expr::obtain_caller_directly_usable_register(context);
    let offset = context
        .memory_mapper
        .get_offset(&src, size)
        .ok_or(format!("Value {:?} is not mapped to stack memory", src))?;

    asms.extend(inst::add_lw_instruction(
        rd,
        Register::Sp,
        RV32Imm::new(offset as i32),
        id.map(|v| InstContext::new(context, v)),
        Some(rd),
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

pub fn store(
    src: Register,
    target: Value,
    context: &mut FunctionContext,
    id: Option<Value>,
    claim: bool,
) -> Result<Vec<RiscvAsm>, String> {
    if target.is_global() {
        store_to_global(src, target, context, id)
    } else {
        store_to_local(src, target, context, id, claim)
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
) -> Result<Vec<RiscvAsm>, String> {
    let mut asms = vec![];
    let tmp = expr::obtain_caller_directly_usable_register(context);
    let size = context.func_data.dfg().value(target).ty().size() as meta::RV32Usize;
    if claim {
        context.memory_mapper.stack_claim(target, size);
    }
    let offset = context
        .memory_mapper
        .get_offset(&target, size)
        .ok_or(format!("Value {:?} is not mapped to stack memory", target))?;
    asms.extend(inst::add_sw_instruction(
        src,
        Register::Sp,
        RV32Imm::new(offset as i32),
        id.map(|v| InstContext::new(context, v)),
        Some(tmp),
    ));
    Ok(asms)
}
