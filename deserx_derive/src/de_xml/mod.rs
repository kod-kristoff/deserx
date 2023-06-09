mod analyze;
mod codegen;
mod lower;

pub use self::analyze::{analyze, Model};
pub use self::codegen::codegen;
pub use self::lower::{lower, Ir};

// use quote::quote;

// mod deserx_xml;

use crate::shared;

pub fn de_xml(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // parse
    let ast = shared::parse_derive(item);
    // let input = parse::parse_data(input);

    // extract struct attributes
    let model = analyze(ast);

    let ir = lower(model);

    let rust = codegen(ir);
    rust
}
