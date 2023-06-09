use proc_macro2::TokenStream;
use proc_macro_error::abort;

pub fn attrs_xml_attribute(attributes: &[crate::parse::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attr| attr.tokens.len() == 1 && attr.tokens[0] == "xml_attribute")
}

pub fn attrs_xml_text(attributes: &[crate::parse::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attr| attr.tokens.len() == 1 && attr.tokens[0] == "xml_text")
}

pub fn attrs_flatten(attributes: &[crate::parse::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attr| attr.tokens.len() == 1 && attr.tokens[0] == "flatten")
}

pub fn parse_derive(tokens: TokenStream) -> syn::DeriveInput {
    match syn::parse2(tokens) {
        Ok(item) => item,
        Err(err) => {
            abort!("error: {}", err)
        }
    }
}
