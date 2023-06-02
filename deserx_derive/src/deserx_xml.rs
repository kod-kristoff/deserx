use proc_macro::TokenStream;

use crate::parse::{struct_bounds_strings, Struct};
use crate::shared;

pub fn derive_ser_xml_struct(struct_: &Struct) -> TokenStream {
    let (generic_w_bounds, generic_no_bounds) = struct_bounds_strings(struct_, "SerXml");

    let mut body = format!(
        r#"let mut elem = quick_xml::events::BytesStart::new(tag);"#,
        // struct_.name
    );

    body.push_str("self.ser_elem_attributes(&mut elem);");
    let mut attrs_body = String::new();

    for field in &struct_.fields {
        let ser_as_attribute = shared::attrs_xml_attribute(&field.attributes);
        let flatten = shared::attrs_flatten(&field.attributes);
        let field_name = field.field_name.as_ref().unwrap();
        if ser_as_attribute {
            // body.push_str("serializer.write_event(quick_xml::events::Event::Text())");
            attrs_body.push_str(&format!(
                r#"elem.push_attribute(("{0}", self.{0}.as_cow_str().as_ref()));"#,
                field_name
            ));
        } else if flatten {
            attrs_body.push_str(&format!("self.{0}.ser_elem_attributes(elem);", field_name))
        }
    }
    body.push_str("serializer.write_event(quick_xml::events::Event::Start(elem.clone()))?;");

    let mut elem_body = String::new();

    for field in &struct_.fields {
        let ser_as_text = shared::attrs_xml_text(&field.attributes);
        let ser_as_attribute = shared::attrs_xml_attribute(&field.attributes);
        let flatten = shared::attrs_flatten(&field.attributes);
        let field_name = field.field_name.as_ref().unwrap();
        if ser_as_attribute {
            if ser_as_text {
                panic!(
                    "can't combine 'xml_attribute' and 'xml_text' for '{}' in {}",
                    field_name, struct_.name
                );
            }
            continue;
        }
        if flatten {
            elem_body.push_str(&format!("self.{0}.ser_elem_body(serializer)?;", field_name));
        } else if ser_as_text {
            // body.push_str("serializer.write_event(quick_xml::events::Event::Text())");
            elem_body.push_str(&format!("self.{0}.ser_as_text(serializer)?;", field_name));
        } else {
            elem_body.push_str(&format!(
                r#"self.{0}.ser_as_element(serializer, "{0}")?;"#,
                // field.field_name.as_ref().unwrap(),
                field_name
            ));
        }
    }

    body.push_str("self.ser_elem_body(serializer)?;");
    body.push_str("serializer.write_event(quick_xml::events::Event::End(elem.to_end()))");
    format!(
        "impl{0} SerXml for {1}{2} {{
            fn serialize_xml<W: std::io::Write>(&self, serializer: &mut quick_xml::Writer<W>) -> Result<(), quick_xml::Error> {{
                self.ser_as_element(serializer, \"{1}\")
            }}
            fn ser_as_element<W: std::io::Write>(&self, serializer: &mut quick_xml::Writer<W>, tag: &str) -> Result<(), quick_xml::Error> {{
                {3}
            }}
            fn ser_elem_body<W: std::io::Write>(&self, serializer: &mut quick_xml::Writer<W>) -> Result<(), quick_xml::Error> {{
                {4}
                Ok(())
            }}
            fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {{
                {5}
            }}
        }}",
        generic_w_bounds, struct_.name, generic_no_bounds,body, elem_body, attrs_body
    )
    .parse()
    .unwrap()
}

pub fn derive_de_xml_struct(struct_: &Struct) -> TokenStream {
    let (generic_w_bounds, generic_no_bounds) = struct_bounds_strings(struct_, "DeXml");

    let mut body = format!(
        r#"
        use quick_xml::events::Event;
        let mut buf = Vec::new();
        match reader.read_event_into(&mut buf)? {{
            Event::Start(evt) if evt.name().as_ref() == b"{}" => {{
                todo!("handle start")
            }}
        }}
        "#, // let mut elem = quick_xml::events::BytesStart::new(tag);"#,
        struct_.name
    );

    // body.push_str("self.ser_elem_attributes(&mut elem);");
    let mut attrs_body = String::new();

    for field in &struct_.fields {
        let ser_as_attribute = shared::attrs_xml_attribute(&field.attributes);
        let flatten = shared::attrs_flatten(&field.attributes);
        let field_name = field.field_name.as_ref().unwrap();
        if ser_as_attribute {
            // body.push_str("serializer.write_event(quick_xml::events::Event::Text())");
            attrs_body.push_str(&format!(
                r#"elem.push_attribute(("{0}", self.{0}.as_cow_str().as_ref()));"#,
                field_name
            ));
        } else if flatten {
            attrs_body.push_str(&format!("self.{0}.ser_elem_attributes(elem);", field_name))
        }
    }
    // body.push_str("serializer.write_event(quick_xml::events::Event::Start(elem.clone()))?;");

    let mut elem_body = String::new();

    for field in &struct_.fields {
        let ser_as_text = shared::attrs_xml_text(&field.attributes);
        let ser_as_attribute = shared::attrs_xml_attribute(&field.attributes);
        let flatten = shared::attrs_flatten(&field.attributes);
        let field_name = field.field_name.as_ref().unwrap();
        if ser_as_attribute {
            if ser_as_text {
                panic!(
                    "can't combine 'xml_attribute' and 'xml_text' for '{}' in {}",
                    field_name, struct_.name
                );
            }
            continue;
        }
        if flatten {
            elem_body.push_str(&format!("self.{0}.ser_elem_body(serializer)?;", field_name));
        } else if ser_as_text {
            // body.push_str("serializer.write_event(quick_xml::events::Event::Text())");
            elem_body.push_str(&format!("self.{0}.ser_as_text(serializer)?;", field_name));
        } else {
            elem_body.push_str(&format!(
                r#"self.{0}.ser_as_element(serializer, "{0}")?;"#,
                // field.field_name.as_ref().unwrap(),
                field_name
            ));
        }
    }

    // body.push_str("self.ser_elem_body(serializer)?;");
    // body.push_str("serializer.write_event(quick_xml::events::Event::End(elem.to_end()))");
    format!(
        "impl{0} DeXml for {1}{2} {{
            fn deserialize_xml<R: std::io::BufRead>(reader: &mut quick_xml::NsReader<R>) -> Result<Self, quick_xml::Error> {{
                {3}
            }}

        }}",
        generic_w_bounds, struct_.name, generic_no_bounds, body,// elem_body, attrs_body
    )
    .parse()
    .unwrap()
}
