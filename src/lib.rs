pub mod asm;
pub mod ir;
pub mod parse;

use std::{
    borrow::Cow,
    fmt::Display,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use lalrpop_util::lexer::Token;

use crate::{asm::{generate_asm, meta::RiscvAsm}, parse::Ast};

#[derive(Debug, Clone)]
pub struct OwnedToken(pub usize, pub String);

impl Display for OwnedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Token(self.0, &self.1).fmt(f)
    }
}

impl<'a> From<Token<'a>> for OwnedToken {
    fn from(token: Token<'a>) -> Self {
        OwnedToken(token.0, token.1.to_string())
    }
}

impl OwnedToken {
    pub fn as_token(&self) -> Token<'_> {
        Token(self.0, &self.1)
    }
}

impl<'a> From<CowToken<'a>> for OwnedToken {
    fn from(token: CowToken<'a>) -> Self {
        OwnedToken(token.0, token.1.into_owned())
    }
}

#[derive(Debug, Clone)]
pub struct CowToken<'a>(pub usize, pub Cow<'a, str>);

impl<'a> Display for CowToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Token(self.0, &self.1).fmt(f)
    }
}

impl<'a> From<Token<'a>> for CowToken<'a> {
    fn from(token: Token<'a>) -> Self {
        CowToken(token.0, Cow::Borrowed(token.1))
    }
}

impl From<OwnedToken> for CowToken<'_> {
    fn from(token: OwnedToken) -> Self {
        CowToken(token.0, Cow::Owned(token.1))
    }
}

impl<'a> CowToken<'a> {
    pub fn as_token(&self) -> Token<'_> {
        Token(self.0, &self.1)
    }
}

pub fn read_and_parse(
    path: &Path,
) -> Result<Ast, lalrpop_util::ParseError<usize, OwnedToken, io::Error>> {
    let f = File::open(path)?;
    src_to_ast(&mut io::BufReader::new(f))
}

pub fn src_to_ast<R: Read>(
    reader: &mut R,
) -> Result<Ast, lalrpop_util::ParseError<usize, OwnedToken, io::Error>> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| lalrpop_util::ParseError::User { error: e })?;
    let ast = parse::parse(&buf).map_err(|e| {
        e.map_error(io::Error::other)
            .map_token(|t| OwnedToken::from(t).to_owned())
    })?;

    Ok(ast)
}

pub fn ast_to_ir(ast: Ast) -> ir::Ast {
    ast.into()
}

pub fn ir_to_asm(ir: ir::Ast) -> Vec<RiscvAsm> {
    generate_asm(ir.program())
}

pub fn output_ir<W: Write>(ir: ir::Ast, writer: &mut W) -> io::Result<()> {
    let ir_str = ir.get_ir()?;
    writer.write_all(ir_str.as_bytes())?;
    Ok(())
}

pub fn output_asm<W: Write>(ir: ir::Ast, writer: &mut W) -> io::Result<()> {
    let asm = ir_to_asm(ir);
    for line in asm {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}
