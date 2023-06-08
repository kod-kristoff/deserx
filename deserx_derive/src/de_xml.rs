extern crate proc_macro;

// mod deserx_xml;
use crate::deserx_xml::*;

use crate::parse;
use crate::shared;

pub fn de_xml(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse::parse_data(input);

    let ts = match &input {
        parse::Data::Struct(struct_) if struct_.named => derive_de_xml_struct(struct_),

        _ => todo!(),
    };

    ts
}
