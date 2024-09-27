use proc_macro::TokenStream;

use crate::parse::{struct_bounds_strings, Struct};
use crate::shared;

pub fn derive_ser_xml_struct(struct_: &Struct) -> TokenStream {
    let (generic_w_bounds, generic_no_bounds) = struct_bounds_strings(struct_, "SerXml");

    // let mut body = format!(
    //     r#"let mut elem = quick_xml::events::BytesStart::new(tag);"#,
    //     // struct_.name
    // );

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
    let body = if elem_body.is_empty() {
        format!(
            "self.ser_as_element_empty(serializer, \"{0}\")",
            struct_.name
        )
    } else {
        format!("self.ser_as_element(serializer, \"{0}\")", struct_.name)
    };
    // body.push_str("self.ser_elem_body(serializer)?;");
    // body.push_str("serializer.write_event(quick_xml::events::Event::End(elem.to_end()))");
    format!(
        "impl{0} SerXml for {1}{2} {{
            fn serialize_xml<W: std::io::Write>(&self, serializer: &mut quick_xml::Writer<W>) -> Result<(), quick_xml::Error> {{
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

    let mut xml_body = format!(
        r#"
        use quick_xml::events::Event;
        "#, // let mut elem = quick_xml::events::BytesStart::new(tag);"#,
            // struct_.name
    );
    let mut xml_body_return = format!("Ok({0} {{", struct_.name);

    // body.push_str("self.ser_elem_attributes(&mut elem);");
    let mut attrs_body = String::new();

    for field in &struct_.fields {
        let de_from_attribute = shared::attrs_xml_attribute(&field.attributes);
        let de_from_text = shared::attrs_xml_text(&field.attributes);
        let flatten = shared::attrs_flatten(&field.attributes);
        let field_name = field.field_name.as_ref().unwrap();
        let field_type = field.ty.path.as_str();
        // xml_body.push_str(&format!("let x = {0};", field_type));
        if de_from_attribute {
            // body.push_str("serializer.write_event(quick_xml::events::Event::Text())");
            // attrs_body.push_str(&format!(
            //     r#"elem.push_attribute(("{0}", self.{0}.as_cow_str().as_ref()));"#,
            //     field_name
            // ));
            xml_body.push_str(&format!(
                "let {0}: {1} = DeXml::deserialize_xml_from_attribute(start, \"{0}\")?;",
                field_name, field_type
            ));
        } else if flatten {
            // attrs_body.push_str(&format!("self.{0}.ser_elem_attributes(elem);", field_name))
            xml_body.push_str(&format!(
                "let {0}: {1} = DeXml::deserialize_xml_from_body(reader, start)?;",
                field_name, field_type
            ))
        } else if de_from_text {
            xml_body.push_str(&format!(
                "let {0}: {1} = DeXml::deserialize_xml_from_text(reader)?;",
                field_name, field_type
            ));
        } else {
            xml_body.push_str(&format!(
                "let {0}: {1} = DeXml::deserialize_xml_from_tag(reader, \"{0}\")?;",
                field_name, field_type
            ))
        }
        xml_body_return.push_str(&format!("{0},", field_name));
    }
    // body.push_str("serializer.write_event(quick_xml::events::Event::Start(elem.clone()))?;");

    // let mut elem_body = String::new();

    // for field in &struct_.fields {
    //     let ser_as_text = shared::attrs_xml_text(&field.attributes);
    //     let ser_as_attribute = shared::attrs_xml_attribute(&field.attributes);
    //     let flatten = shared::attrs_flatten(&field.attributes);
    //     let field_name = field.field_name.as_ref().unwrap();
    //     if ser_as_attribute {
    //         if ser_as_text {
    //             panic!(
    //                 "can't combine 'xml_attribute' and 'xml_text' for '{}' in {}",
    //                 field_name, struct_.name
    //             );
    //         }
    //         continue;
    //     }
    //     if flatten {
    //         elem_body.push_str(&format!("self.{0}.ser_elem_body(serializer)?;", field_name));
    //     } else if ser_as_text {
    //         // body.push_str("serializer.write_event(quick_xml::events::Event::Text())");
    //         elem_body.push_str(&format!("self.{0}.ser_as_text(serializer)?;", field_name));
    //     } else {
    //         elem_body.push_str(&format!(
    //             r#"self.{0}.ser_as_element(serializer, "{0}")?;"#,
    //             // field.field_name.as_ref().unwrap(),
    //             field_name
    //         ));
    //     }
    // }

    // body.push_str("self.ser_elem_body(serializer)?;");
    // body.push_str("serializer.write_event(quick_xml::events::Event::End(elem.to_end()))");
    xml_body_return.push_str("})");
    format!(
        "impl{0} DeXml for {1}{2} {{

            fn deserialize_xml<R: std::io::BufRead>(reader: &mut quick_xml::NsReader<R>) -> Result<Self, deserx::DeXmlError> {{
                Self::deserialize_xml_from_tag(reader, \"{1}\")
            }}
            fn deserialize_xml_from_body<R: std::io::BufRead>(
                reader: &mut NsReader<R>,
                start: &BytesStart,
            ) -> Result<Self, deserx::DeXmlError> {{
                {3}
                {4}
            }}
        }}",
        generic_w_bounds, struct_.name, generic_no_bounds, xml_body, xml_body_return// elem_body, attrs_body
    )
    .parse()
    .unwrap()
}
