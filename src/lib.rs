pub mod ast;
pub mod generate;
pub mod lexer;
pub mod parser;

pub mod prelude {
    pub use crate::{
        ast::{
            expression::*, function::*, module::*, operator::*, value::AstIdent, value::AstValue,
            value::Value,
        },
        generate::*,
        lexer::{Token::*, Value::I64, *},
        parser::*,
    };
    pub use codegen::{ir::Function, settings::Flags, verify_function};
    pub use cranelift::codegen::{ir::entities, Context};
    pub use cranelift::prelude::{isa::*, types::*, *};
    pub use cranelift_module::{default_libcall_names, Linkage, Module};
    pub use cranelift_object::ObjectModule;
    pub use std::error::Error;
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use std::process::Command;
}
