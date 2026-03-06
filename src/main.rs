use std::{env, fs::File, io::Write};

fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let mut args = env::args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    let ast = sysy_compiler::read_and_parse(input.as_ref()).inspect_err(|e| eprintln!("{}", e))?;
    // println!("{}", ast);
    // println!("{:#?}", ast);
    let memory_ir = sysy_compiler::parse_to_ir(ast);
    let ir = memory_ir.get_ir()?;
    // println!("{ir}");
    File::create(output)?.write_all(ir.as_bytes())?;

    Ok(())
}
