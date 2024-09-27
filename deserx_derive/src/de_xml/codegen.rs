use quote::{quote, ToTokens};
use syn::{parse_quote, punctuated::Punctuated, token::Token, Expr, Stmt};

/// codegen
use super::{
    lower::{BodyImpl, DeField, DeFrom, DeReturn},
    Ir,
};

pub fn codegen(ir: Ir) -> proc_macro2::TokenStream {
    let Ir { body_impl, item } = ir;

    let ident = &item.ident;
    let tag = ident.to_string();
    let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

    let BodyImpl {
        de_fields,
        de_return,
    } = &body_impl;
    let body_expanded = quote! {
        #(#de_fields)*
        #de_return
    };
    quote::quote! {
        impl #impl_generics DeXml for #ident #type_generics #where_clause {
            fn deserialize_xml<R: std::io::BufRead>(reader: &mut quick_xml::NsReader<R>) -> Result<Self, deserx::DeXmlError> {
                Self::deserialize_xml_from_tag(reader, #tag)
            }
            fn deserialize_xml_from_body<R: std::io::BufRead>(
                reader: &mut NsReader<R>,
                start: &BytesStart,
            ) -> Result<Self, deserx::DeXmlError> {
                #body_expanded
            }
        }
    }.into()
}

impl ToTokens for BodyImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BodyImpl {
            de_fields,
            de_return,
        } = self;
        // let DeReturn { idents } = de_return;
        // for de_field in de_fields {
        let idents = de_return.idents.iter().map(|i| quote::quote!(#i,));
        // todo!("before parse");
        let stmt: Stmt = parse_quote!(
            #(#de_fields)*;
            Ok(Self {#(#idents)*})
        );
        stmt.to_tokens(tokens);
        // }
    } // return Ok(Self{
      //     #(#idents)*
      // });
}

impl ToTokens for DeField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DeField {
            ident,
            name,
            ty,
            de_from,
        } = self;
        let stmt: Stmt = match de_from {
            DeFrom::Attribute => {
                parse_quote!(let #ident: #ty = DeXml::deserialize_xml_from_attribute(start, #name)?;)
            }
            DeFrom::Body => {
                parse_quote!(let #ident: #ty = DeXml::deserialize_xml_from_body(reader, start)?;)
            }
            DeFrom::Tag => {
                parse_quote!(let #ident: #ty = DeXml::deserialize_xml_from_tag(reader, #name)?;)
            }
            DeFrom::Text => {
                parse_quote!(let #ident: #ty = DeXml::deserialize_xml_from_text(reader)?;)
            }
        };
        stmt.to_tokens(tokens);
    }
}

impl ToTokens for DeReturn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let idents = self.idents.iter().map(|i| quote::quote!(#i,));
        let DeReturn { idents } = self;
        let stmt: Stmt = parse_quote! {
            return Ok(Self{#(#idents,)*});
        };
        stmt.to_tokens(tokens);
    }
}
