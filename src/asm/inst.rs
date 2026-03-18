use koopa::ir::Value;

use crate::asm::meta::{
    FunctionContext, RV32Imm, RV32Imm12, Register, RegisterValue, RiscvAsm, RiscvInstruction,
};

#[derive(Debug)]
pub struct InstContext<'a, 'b: 'a> {
    pub context: &'a mut FunctionContext<'b>,
    pub id: Value,
}

impl InstContext<'_, '_> {
    pub fn new<'a, 'b>(context: &'a mut FunctionContext<'b>, id: Value) -> InstContext<'a, 'b> {
        InstContext { context, id }
    }
}

pub fn register_dest(dest: Register, context: Option<InstContext>) {
    if let Some(ctx) = context {
        ctx.context.register_mapper.remove_by_register(dest);
        ctx.context
            .register_mapper
            .insert(RegisterValue::InstRet(ctx.id), dest);
    }
}

pub fn label(label: &str) -> RiscvAsm {
    RiscvAsm::Label(label.to_string())
}

pub fn ret_instruction() -> RiscvAsm {
    RiscvAsm::Instruction(RiscvInstruction::Ret)
}

pub fn call_instruction(func: &str) -> RiscvAsm {
    RiscvAsm::Instruction(RiscvInstruction::Call {
        func: func.to_string(),
    })
}

pub fn li_instruction(dest: Register, imm: i32, context: Option<InstContext>) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Li {
        dest,
        imm: RV32Imm::new(imm),
    })
}

pub fn mv_instruction(dest: Register, src: Register, context: Option<InstContext>) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Mv { dest, src })
}

pub fn la_instruction(dest: Register, label: &str, context: Option<InstContext>) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::La {
        dest,
        label: label.to_string(),
    })
}

macro_rules! binary_instruction {
    ($dest:expr, $src1:expr, $src2:expr, $context:expr, $variant:tt) => {
        use crate::asm::meta::{RiscvAsm, RiscvInstruction};
        register_dest($dest, $context);
        return RiscvAsm::Instruction(RiscvInstruction::$variant {
            dest: $dest,
            src1: $src1,
            src2: $src2,
        });
    };
}

macro_rules! define_binary_instruction {
    ($name:ident, $variant:tt) => {
        pub fn $name(
            dest: Register,
            src1: Register,
            src2: Register,
            context: Option<InstContext>,
        ) -> RiscvAsm {
            binary_instruction!(dest, src1, src2, context, $variant);
        }
    };
}

define_binary_instruction!(add_instruction, Add);
define_binary_instruction!(sub_instruction, Sub);
define_binary_instruction!(mul_instruction, Mul);
define_binary_instruction!(div_instruction, Div);
define_binary_instruction!(rem_instruction, Mod);
define_binary_instruction!(and_instruction, And);
define_binary_instruction!(or_instruction, Or);
define_binary_instruction!(xor_instruction, Xor);
define_binary_instruction!(slt_instruction, Slt);
define_binary_instruction!(sgt_instruction, Sgt);

pub fn seqz_instruction(dest: Register, src: Register, context: Option<InstContext>) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Seqz { dest, src })
}

pub fn snez_instruction(dest: Register, src: Register, context: Option<InstContext>) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Snez { dest, src })
}

pub fn addi_instruction(
    dest: Register,
    src: Register,
    imm: i16,
    context: Option<InstContext>,
) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Addi {
        dest,
        src1: src,
        src2: RV32Imm12::new(imm),
    })
}

pub fn lw_instruction(
    dest: Register,
    base: Register,
    offset: i16,
    context: Option<InstContext>,
) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Lw {
        dest,
        base,
        offset: RV32Imm12::new(offset),
    })
}

pub fn sw_instruction(
    src: Register,
    base: Register,
    offset: i16,
    _context: Option<InstContext>,
) -> RiscvAsm {
    // we don't need to register_dest for sw, since it doesn't write to a register
    RiscvAsm::Instruction(RiscvInstruction::Sw {
        src,
        base,
        offset: RV32Imm12::new(offset),
    })
}

pub fn j_instruction(label: &str) -> RiscvAsm {
    RiscvAsm::Instruction(RiscvInstruction::J {
        label: label.to_string(),
    })
}

pub fn beqz_instruction(src: Register, label: &str) -> RiscvAsm {
    RiscvAsm::Instruction(RiscvInstruction::Beqz {
        src,
        label: label.to_string(),
    })
}

pub fn bnez_instruction(src: Register, label: &str) -> RiscvAsm {
    RiscvAsm::Instruction(RiscvInstruction::Bnez {
        src,
        label: label.to_string(),
    })
}

macro_rules! reusable_register_dest {
    ($ctx:expr, $dest:expr) => {
        if let Some(ctx) = $ctx {
            let new_ctx = Some(InstContext {
                context: ctx.context,
                id: ctx.id,
            });
            register_dest($dest, new_ctx);
            Some(ctx)
        } else {
            $ctx
        }
    };
}

pub fn add_lw_instruction(
    dest: Register,
    src: Register,
    offset: RV32Imm,
    context: Option<InstContext>,
    rd: Option<Register>,
) -> Vec<RiscvAsm> {
    let context = reusable_register_dest!(context, dest);
    let mut asms = vec![];
    if !(-2048..=2047).contains(&offset.value()) {
        match rd {
            Some(rd) => {
                asms.push(li_instruction(rd, offset.value(), None));
                asms.push(add_instruction(rd, src, rd, None));
                asms.push(lw_instruction(dest, rd, 0, context));
            }
            None => {
                panic!(
                    "Offset {} is out of range for add_lw_instruction, and no temporary register provided",
                    offset.value()
                );
            }
        }
        return asms;
    }
    vec![lw_instruction(dest, src, offset.value() as i16, context)]
}

pub fn add_sw_instruction(
    src: Register,
    base: Register,
    offset: RV32Imm,
    context: Option<InstContext>,
    rd: Option<Register>,
) -> Vec<RiscvAsm> {
    let mut asms = vec![];
    if !(-2048..=2047).contains(&offset.value()) {
        match rd {
            Some(rd) => {
                asms.push(li_instruction(rd, offset.value(), None));
                asms.push(add_instruction(rd, base, rd, None));
                asms.push(sw_instruction(src, rd, 0, None));
            }
            None => {
                panic!(
                    "Offset {} is out of range for add_sw_instruction, and no temporary register provided",
                    offset.value()
                );
            }
        }
        return asms;
    }
    vec![sw_instruction(src, base, offset.value() as i16, context)]
}

pub fn label_sw_instruction(
    src: Register,
    label: &str,
    _context: Option<InstContext>,
    rd: Option<Register>,
) -> Vec<RiscvAsm> {
    let mut asms = vec![];
    match rd {
        Some(rd) => {
            asms.push(la_instruction(rd, label, None));
            asms.push(sw_instruction(src, rd, 0, None));
        }
        None => {
            panic!(
                "No temporary register provided for label_sw_instruction with label {}",
                label
            );
        }
    }
    asms
}

pub fn lui_instruction(dest: Register, imm: RV32Imm, context: Option<InstContext>) -> RiscvAsm {
    register_dest(dest, context);
    RiscvAsm::Instruction(RiscvInstruction::Lui { dest, imm })
}
