use std::collections::HashMap;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(deserx))]
pub struct DeserxStructAttributes {
    pub rename: Option<String>,
}
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(deserx))]
pub struct DeserxFieldAttributes {
    pub rename: Option<String>,
    #[deluxe(default = false)]
    pub xml_attribute: bool,
    #[deluxe(default = false)]
    pub xml_text: bool,
    #[deluxe(default = false)]
    pub flatten: bool,
}

fn extract_metadata_field_attrs(
    ast: &mut syn::DeriveInput,
) -> HashMap<String, DeserxFieldAttributes> {
    let mut field_attrs = HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let attrs: DeserxFieldAttributes = deluxe::extract_attributes(field).unwrap();
            field_attrs.insert(field_name, attrs);
        }
    }
    field_attrs
}
/// analyze
pub fn analyze(ast: syn::DeriveInput) -> Model {
    let mut item = ast;
    let struct_attrs: DeserxStructAttributes = deluxe::extract_attributes(&mut item).unwrap();
    let field_attrs = extract_metadata_field_attrs(&mut item);
    // let ts = match &input {
    //     parse::Data::Struct(struct_) if struct_.named => derive_de_xml_struct(struct_),

    // define impl variables

    //     _ => todo!(),
    // };

    // generate
    Model {
        struct_attrs,
        field_attrs,
        item,
    }
}

pub struct Model {
    pub struct_attrs: DeserxStructAttributes,
    pub field_attrs: HashMap<String, DeserxFieldAttributes>,
    pub item: syn::DeriveInput,
}
