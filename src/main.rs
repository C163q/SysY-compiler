use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
};

fn print_usage() {
    eprintln!("Usage: sysy-compiler <mode> <input> -o <output>");
    eprintln!("Modes:");
    eprintln!("  -koopa: Output Koopa IR");
    eprintln!("  -riscv: Output RISC-V assembly");
}

struct Args {
    pub mode: String,
    pub input: String,
    pub output: String,
}

impl Args {
    pub fn new(mode: String, input: String, output: String) -> Self {
        Self {
            mode,
            input,
            output,
        }
    }
}

fn parse_args() -> anyhow::Result<Args> {
    let mut args = env::args();
    args.next(); // Skip program name
    let mode = args.next().ok_or_else(|| anyhow::anyhow!("Missing mode"))?;
    let input = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing input file"))?;
    args.next(); // Skip -o
    let output = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing output file"))?;
    Ok(Args::new(mode, input, output))
}

fn main() -> anyhow::Result<()> {
    // 解析命令行参数（待改进）
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            print_usage();
            anyhow::bail!(e);
        }
    };

    let ast =
        sysy_compiler::read_and_parse(args.input.as_ref()).inspect_err(|e| eprintln!("{}", e))?;
    // println!("{}", ast);
    // println!("{:#?}", ast);
    let memory_ir = sysy_compiler::ast_to_ir(ast);
    match args.mode.as_str() {
        "-koopa" => {
            let ir = memory_ir.get_ir()?;
            // println!("{ir}");
            BufWriter::new(File::create(args.output)?).write_all(ir.as_bytes())?;
        }
        "-riscv" => {
            let mut asm_file = BufWriter::new(File::create(args.output)?);
            sysy_compiler::output_asm(memory_ir, &mut asm_file)?;
            return Ok(());
        }
        _ => {
            print_usage();
            anyhow::bail!("Unknown mode: {}", args.mode);
        }
    }
    Ok(())
}
