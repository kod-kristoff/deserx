use std::iter::Peekable;

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

#[cfg(features = "no_std")]
use hashbrown::HashMap;

#[cfg(not(features = "no_std"))]
use std::collections::HashMap;

#[allow(dead_code)]
pub enum Data {
    Struct(Struct),
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub named: bool,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
    pub generics: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub tokens: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Visibility {
    Public,
    Crate,
    Restricted,
    Private,
}

#[derive(Debug)]
pub struct Field {
    pub attributes: Vec<Attribute>,
    pub vis: Visibility,
    pub field_name: Option<String>,
    pub ty: Type,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Type {
    pub is_option: bool,
    pub path: String,
}

pub fn parse_data(input: TokenStream) -> Data {
    let mut source = input.into_iter().peekable();

    let pub_or_type = next_ident(&mut source).expect("not an ident");

    let type_keyword = if pub_or_type == "pub" {
        next_ident(&mut source).expect("pub(whatever) is not supported yet")
    } else {
        pub_or_type
    };

    let res;

    match type_keyword.as_str() {
        "struct" => {
            let mut struct_ = next_struct(&mut source);
            // struct_.attributes = attributes;

            res = Data::Struct(struct_);
        }
        // "enum" => {
        //     let enum_ = next_enum(&mut source);
        //     res = Data::Enum(enum_);
        // }
        // "union" => unimplemented!("Unions are not supported"),
        unexpected => panic!("Unexpected keyword: {}", unexpected),
    }

    assert!(
        source.next().is_none(),
        "Unexpected data after end of the struct"
    );

    res
}

pub(crate) fn struct_bounds_strings(struct_: &Struct, bound_name: &str) -> (String, String) {
    let generics: &HashMap<String, String> = &struct_.generics;
    if generics.is_empty() {
        return ("".to_string(), "".to_string());
    }

    todo!("struct_bound_strings")
}
pub fn next_ident(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Option<String> {
    if let Some(TokenTree::Ident(ident)) = source.peek() {
        let ident = format!("{}", ident);
        source.next();
        Some(ident)
    } else {
        None
    }
}

pub fn next_struct(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Struct {
    let struct_name = next_ident(source).expect("Unnamed structs are not supported");
    let generic_types = get_bounds(source);
    let group = next_group(source);
    if group.is_none() {
        // unit struct
        // skip ; at the end of struct like this: "struct Foo;"
        let _ = next_punct(source);

        return Struct {
            name: struct_name,
            named: false,
            fields: vec![],
            attributes: vec![],
            generics: Default::default(),
        };
    }

    let group = group.unwrap();
    let delimiter = group.delimiter();
    let named = match delimiter {
        Delimiter::Parenthesis => false,
        Delimiter::Brace => true,

        _ => panic!("Struct with unsupported delimiter"),
    };

    let mut body = group.stream().into_iter().peekable();
    let fields = next_fields(&mut body, named, &generic_types);

    if named == false {
        next_exact_punct(source, ";").expect("Expected ; on end of tuple struct");
    }
    Struct {
        name: struct_name,
        named,
        fields,
        attributes: vec![],
        generics: generic_types,
    }
}

pub fn next_group(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Option<Group> {
    if let Some(TokenTree::Group(_)) = source.peek() {
        let group = match source.next().unwrap() {
            TokenTree::Group(group) => group,
            _ => unreachable!("just checked with peek()!"),
        };
        Some(group)
    } else {
        None
    }
}

pub fn next_punct(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Option<String> {
    if let Some(TokenTree::Punct(punct)) = source.peek() {
        let punct = format!("{}", punct);
        source.next();
        return Some(punct);
    }

    return None;
}

fn get_bounds(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> HashMap<String, String> {
    let mut ret = HashMap::new();

    ret
}

pub fn next_exact_punct(
    source: &mut Peekable<impl Iterator<Item = TokenTree>>,
    pattern: &str,
) -> Option<String> {
    if let Some(TokenTree::Punct(punct)) = source.peek() {
        let punct = format!("{}", punct);
        if punct == pattern {
            source.next();
            return Some(punct);
        }
    }

    return None;
}

fn next_fields(
    mut body: &mut Peekable<impl Iterator<Item = TokenTree>>,
    named: bool,
    generic_typenames: &HashMap<String, String>,
) -> Vec<Field> {
    let mut fields = vec![];

    loop {
        if next_eof(body).is_some() {
            break;
        }

        let attributes = next_attributes_list(&mut body);
        let _visibility = next_visibility_modifier(&mut body);
        let field_name = if named {
            let field_name = next_ident(&mut body).expect("Field name expected");

            let _ = next_exact_punct(&mut body, ":").expect("Delimeter after field name expected");
            Some(field_name)
        } else {
            None
        };

        let ty = next_type(&mut body, &generic_typenames).expect("Expected field type");
        let _punct = next_punct(&mut body);

        fields.push(Field {
            attributes,
            vis: Visibility::Public,
            field_name,
            ty,
        });
    }
    fields
}

pub fn next_eof<T: Iterator>(source: &mut Peekable<T>) -> Option<()> {
    if source.peek().is_none() {
        Some(())
    } else {
        None
    }
}

fn next_type<T: Iterator<Item = TokenTree>>(
    mut source: &mut Peekable<T>,
    generic_typenames: &HashMap<String, String>,
) -> Option<Type> {
    let group = next_group(&mut source);
    if group.is_some() {
        let mut group = group.unwrap().stream().into_iter().peekable();

        let mut tuple_type = Type {
            is_option: false,
            path: "".to_string(),
        };

        while let Some(next_ty) = next_type(&mut group, generic_typenames) {
            tuple_type.path.push_str(&format!("{}, ", next_ty.path));
        }

        return Some(tuple_type);
    }

    // read a path like a::b::c::d
    let mut ty = next_ident(&mut source)?;
    while let Some(_) = next_exact_punct(&mut source, ":") {
        let _second_colon = next_exact_punct(&mut source, ":").expect("Expecting second :");

        let next_ident = next_ident(&mut source).expect("Expecting next path part after ::");
        ty.push_str(&format!("::{}", next_ident));
    }

    let angel_bracket = next_exact_punct(&mut source, "<");
    if angel_bracket.is_some() {
        let mut generic_type =
            next_type(source, generic_typenames).expect("Expecting generic argument");
        while let Some(_comma) = next_exact_punct(&mut source, ",") {
            let next_ty = next_type(source, generic_typenames).expect("Expecting generic argument");
            generic_type.path.push_str(&format!(", {}", next_ty.path));
        }

        let _closing_bracket =
            next_exact_punct(&mut source, ">").expect("Expecting closing generic bracket");

        if ty == "Option" {
            Some(Type {
                path: generic_type.path.clone(),
                is_option: true,
            })
        } else {
            Some(Type {
                path: format!("{}<{}>", ty, generic_type.path),
                is_option: false,
            })
        }
    } else {
        Some(Type {
            path: ty.clone(),
            is_option: false,
        })
    }
}

fn next_attribute<T: Iterator<Item = TokenTree>>(
    mut source: &mut Peekable<T>,
) -> Option<Option<Attribute>> {
    // all attributes, even doc-comments, starts with "#"
    let next_attr_punct = next_punct(&mut source);
    if let Some("#") = next_attr_punct.as_deref() {
        let mut attr_group = next_group(&mut source)
            .expect("Expecting attribute body")
            .stream()
            .into_iter()
            .peekable();

        let name = next_ident(&mut attr_group).expect("Attributes should start with a name");

        if name != "deserx" {
            return Some(None);
        }

        let mut args_group = next_group(&mut attr_group)
            .expect("Expecting attribute body")
            .stream()
            .into_iter()
            .peekable();

        let mut attr_tokens = vec![];

        loop {
            let attribute_name = next_ident(&mut args_group).expect("Expecting attribute name");
            attr_tokens.push(attribute_name);

            // single-word attribute, like #[deserx(whatever)]
            if next_eof(&mut args_group).is_some() {
                break;
            }
            let _ = next_exact_punct(&mut args_group, "=")
                .expect("Expecting = after attribute argument name");
            let value = next_literal(&mut args_group).expect("Expecting argument value");

            attr_tokens.push(value);

            if next_eof(&mut args_group).is_some() {
                break;
            }
        }

        return Some(Some(Attribute {
            name,
            tokens: attr_tokens,
        }));
    }

    None
}

fn next_attributes_list(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Vec<Attribute> {
    let mut attributes = vec![];

    while let Some(attr) = next_attribute(source) {
        if let Some(deserx_attr) = attr {
            attributes.push(deserx_attr);
        }
    }

    attributes
}

pub fn next_literal(source: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Option<String> {
    if let Some(TokenTree::Literal(lit)) = source.peek() {
        let mut literal = lit.to_string();

        // the only way to check that literal is string :/
        if literal.starts_with("\"") {
            literal.remove(0);
            literal.remove(literal.len() - 1);
        }
        source.next();
        return Some(literal);
    }

    return None;
}

pub fn next_visibility_modifier(
    source: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Option<String> {
    if let Some(TokenTree::Ident(ident)) = source.peek() {
        if format!("{}", ident) == "pub" {
            source.next();

            // skip (crate) and alike
            if let Some(TokenTree::Group(group)) = source.peek() {
                if group.delimiter() == Delimiter::Parenthesis {
                    next_group(source);
                }
            }

            return Some("pub".to_string());
        }
    }

    return None;
}
