use std::collections::HashMap;

use proc_macro2::Ident;
use proc_macro_error::abort_call_site;
use syn::{Data, Fields};

/// lower
use super::{analyze::DeserxFieldAttributes, Model};

pub fn lower(model: Model) -> Ir {
    let Model {
        struct_attrs,
        field_attrs,
        item,
    } = model;
    let struct_data = match &item.data {
        Data::Struct(s) => s,
        Data::Enum(_) => abort_call_site!("enum is not yet supported"),
        Data::Union(_) => abort_call_site!("union is not yet supported"),
    };
    let mut body_impl = BodyImpl::default();

    match &struct_data.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let ident = field.ident.as_ref().unwrap();
                let name = ident.to_string();
                let ty = field.ty.clone();
                let de_from = de_from(&field_attrs, &name);
                body_impl.de_fields.push(DeField {
                    ident: ident.clone(),
                    name,
                    ty,
                    de_from,
                });
                body_impl.de_return.idents.push(ident.clone());
            }
        }
        Fields::Unnamed(_) => abort_call_site!("Unnamed fields not supported yet"),
        Fields::Unit => abort_call_site!("Unit fields not supported yet"),
    }
    Ir { body_impl, item }
}

pub struct Ir {
    pub body_impl: BodyImpl,
    pub item: syn::DeriveInput,
}

pub struct BodyImpl {
    pub de_fields: Vec<DeField>,
    pub de_return: DeReturn,
}

impl Default for BodyImpl {
    fn default() -> Self {
        Self {
            de_fields: Vec::default(),
            de_return: DeReturn::default(),
        }
    }
}
pub struct DeField {
    pub ident: Ident,
    pub name: String,
    pub ty: syn::Type,
    pub de_from: DeFrom,
}
pub struct DeReturn {
    pub idents: Vec<Ident>,
}

impl Default for DeReturn {
    fn default() -> Self {
        DeReturn {
            idents: Vec::default(),
        }
    }
}
pub enum DeFrom {
    Attribute,
    Body,
    Tag,
    Text,
}

pub fn de_from(attrs: &HashMap<String, DeserxFieldAttributes>, name: &String) -> DeFrom {
    let field_attrs = attrs.get(name);

    let field_attrs = match field_attrs {
        None => return DeFrom::Tag,
        Some(x) => x,
    };

    if field_attrs.xml_attribute {
        if field_attrs.xml_text {
            abort_call_site!("can't combine 'xml_attribute' and 'xml_text' for a field");
        }
        DeFrom::Attribute
    } else if field_attrs.xml_text {
        DeFrom::Text
    } else if field_attrs.flatten {
        DeFrom::Body
    } else {
        DeFrom::Tag
    }
}
