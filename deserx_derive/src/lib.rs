extern crate proc_macro;
use proc_macro_error::proc_macro_error;

mod deserx_xml;
use crate::deserx_xml::*;

mod de_xml;
mod parse;
mod ser_xml;

mod shared;

#[proc_macro_derive(SerXml, attributes(deserx))]
pub fn ser_xml(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ser_xml::ser_xml(input.into()).into()
    // let input = parse::parse_data(input);

    // let ts = match &input {
    //     parse::Data::Struct(struct_) if struct_.named => derive_ser_xml_struct(struct_),

    //     _ => todo!(),
    // };

    // ts
}

#[proc_macro_error]
#[proc_macro_derive(DeXml, attributes(deserx))]
pub fn de_xml(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    de_xml::de_xml(item.into()).into()
    // let input = parse::parse_data(input);

    // let ts = match &input {
    //     parse::Data::Struct(struct_) if struct_.named => derive_de_xml_struct(struct_),

    //     _ => todo!(),
    // };

    // ts
}
