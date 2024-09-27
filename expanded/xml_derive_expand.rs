#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::io::Cursor;
use deserx::{DeXml, SerXml};
use quick_xml::{
    events::{BytesStart, Event},
    NsReader, Writer,
};
pub struct Attribute {
    #[deserx(xml_attribute)]
    resource: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "Attribute",
            "resource",
            &&self.resource,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Attribute {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Attribute) -> bool {
        self.resource == other.resource
    }
}
impl SerXml for Attribute {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element_empty(serializer, "Attribute")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        Ok(())
    }
    fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {
        elem.push_attribute(("resource", self.resource.as_cow_str().as_ref()));
    }
}
impl DeXml for Attribute {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Attribute")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let resource: String = DeXml::deserialize_xml_from_attribute(start, "resource")?;
        return Ok(Self { resource });
    }
}
struct Common {
    name: String,
}
impl SerXml for Common {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "Common")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.name.ser_as_element(serializer, "name")?;
        Ok(())
    }
    fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Common {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Common {
    #[inline]
    fn eq(&self, other: &Common) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Common {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "Common",
            "name",
            &&self.name,
        )
    }
}
impl DeXml for Common {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Common")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let name: String = DeXml::deserialize_xml_from_tag(reader, "name")?;
        return Ok(Self { name });
    }
}
pub struct Flatten {
    #[deserx(flatten)]
    common: Common,
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
}
impl SerXml for Flatten {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "Flatten")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.common.ser_elem_body(serializer)?;
        self.element.ser_as_element(serializer, "element")?;
        self.text.ser_as_text(serializer)?;
        Ok(())
    }
    fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {
        self.common.ser_elem_attributes(elem);
        elem.push_attribute(("attribute", self.attribute.as_cow_str().as_ref()));
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Flatten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Flatten",
            "common",
            &self.common,
            "attribute",
            &self.attribute,
            "element",
            &self.element,
            "text",
            &&self.text,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Flatten {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Flatten {
    #[inline]
    fn eq(&self, other: &Flatten) -> bool {
        self.common == other.common && self.attribute == other.attribute
            && self.element == other.element && self.text == other.text
    }
}
impl DeXml for Flatten {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Flatten")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let common: Common = DeXml::deserialize_xml_from_body(reader, start)?;
        let attribute: String = DeXml::deserialize_xml_from_attribute(
            start,
            "attribute",
        )?;
        let element: String = DeXml::deserialize_xml_from_tag(reader, "element")?;
        let text: String = DeXml::deserialize_xml_from_text(reader)?;
        return Ok(Self {
            common,
            attribute,
            element,
            text,
        });
    }
}
pub struct FlattenAttribute {
    #[deserx(flatten)]
    any_name: Attribute,
}
#[automatically_derived]
impl ::core::fmt::Debug for FlattenAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "FlattenAttribute",
            "any_name",
            &&self.any_name,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FlattenAttribute {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FlattenAttribute {
    #[inline]
    fn eq(&self, other: &FlattenAttribute) -> bool {
        self.any_name == other.any_name
    }
}
impl SerXml for FlattenAttribute {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "FlattenAttribute")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.any_name.ser_elem_body(serializer)?;
        Ok(())
    }
    fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {
        self.any_name.ser_elem_attributes(elem);
    }
}
impl DeXml for FlattenAttribute {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "FlattenAttribute")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let any_name: Attribute = DeXml::deserialize_xml_from_body(reader, start)?;
        return Ok(Self { any_name });
    }
}
struct FlattenTwice {
    #[deserx(flatten)]
    field: Flatten,
}
impl SerXml for FlattenTwice {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "FlattenTwice")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.field.ser_elem_body(serializer)?;
        Ok(())
    }
    fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {
        self.field.ser_elem_attributes(elem);
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FlattenTwice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "FlattenTwice",
            "field",
            &&self.field,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FlattenTwice {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FlattenTwice {
    #[inline]
    fn eq(&self, other: &FlattenTwice) -> bool {
        self.field == other.field
    }
}
impl DeXml for FlattenTwice {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "FlattenTwice")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let field: Flatten = DeXml::deserialize_xml_from_body(reader, start)?;
        return Ok(Self { field });
    }
}
pub struct Root {
    #[deserx(xml_attribute)]
    attribute: String,
    element: String,
    #[deserx(xml_text)]
    text: String,
    child: Common,
}
impl SerXml for Root {
    fn serialize_xml<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.ser_as_element(serializer, "Root")
    }
    fn ser_elem_body<W: std::io::Write>(
        &self,
        serializer: &mut quick_xml::Writer<W>,
    ) -> Result<(), quick_xml::Error> {
        self.element.ser_as_element(serializer, "element")?;
        self.text.ser_as_text(serializer)?;
        self.child.ser_as_element(serializer, "child")?;
        Ok(())
    }
    fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {
        elem.push_attribute(("attribute", self.attribute.as_cow_str().as_ref()));
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Root {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Root",
            "attribute",
            &self.attribute,
            "element",
            &self.element,
            "text",
            &self.text,
            "child",
            &&self.child,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Root {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Root {
    #[inline]
    fn eq(&self, other: &Root) -> bool {
        self.attribute == other.attribute && self.element == other.element
            && self.text == other.text && self.child == other.child
    }
}
impl DeXml for Root {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "Root")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut NsReader<R>,
        start: &BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let attribute: String = DeXml::deserialize_xml_from_attribute(
            start,
            "attribute",
        )?;
        let element: String = DeXml::deserialize_xml_from_tag(reader, "element")?;
        let text: String = DeXml::deserialize_xml_from_text(reader)?;
        let child: Common = DeXml::deserialize_xml_from_tag(reader, "child")?;
        return Ok(Self {
            attribute,
            element,
            text,
            child,
        });
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_attribute_empty"]
pub const deser_attribute_empty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_attribute_empty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(deser_attribute_empty())),
};
fn deser_attribute_empty() {
    let val = Attribute { resource: "#A".into() };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    val.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (&String::from_utf8_lossy(&buffer), &r##"<Attribute resource="#A"/>"##) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let val_copy = Attribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    match (&reader.read_event_into(&mut buf).unwrap(), &quick_xml::events::Event::Eof) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&val, &val_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "de_attribute_start_end"]
pub const de_attribute_start_end: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("de_attribute_start_end"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(de_attribute_start_end())),
};
fn de_attribute_start_end() {
    let data = r##"<Attribute resource="#A"></Attribute>"##;
    let mut reader = NsReader::from_str(data);
    let _contributor = Attribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    match (&reader.read_event_into(&mut buf).unwrap(), &quick_xml::events::Event::Eof) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_flatten_attribute_empty"]
pub const deser_flatten_attribute_empty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_flatten_attribute_empty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        deser_flatten_attribute_empty(),
    )),
};
fn deser_flatten_attribute_empty() {
    let val = FlattenAttribute {
        any_name: Attribute { resource: "#A".into() },
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    val.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (
        &String::from_utf8_lossy(&buffer),
        &r##"<FlattenAttribute resource="#A"></FlattenAttribute>"##,
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let val_copy = FlattenAttribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    match (&reader.read_event_into(&mut buf).unwrap(), &quick_xml::events::Event::Eof) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&val, &val_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "de_flatten_attribute_start_end"]
pub const de_flatten_attribute_start_end: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("de_flatten_attribute_start_end"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        de_flatten_attribute_start_end(),
    )),
};
fn de_flatten_attribute_start_end() {
    let data = r##"<FlattenAttribute resource="#A"/>"##;
    let mut reader = NsReader::from_str(data);
    let _contributor = FlattenAttribute::deserialize_xml(&mut reader).unwrap();
    let mut buf = Vec::new();
    match (&reader.read_event_into(&mut buf).unwrap(), &quick_xml::events::Event::Eof) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_derive_common"]
pub const deser_derive_common: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_derive_common"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(deser_derive_common())),
};
fn deser_derive_common() {
    let data = Common {
        name: "child content".to_string(),
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (
        &String::from_utf8_lossy(&buffer),
        &"<Common>\
            <name>child content</name>\
        </Common>\
        ",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let data_copy = Common::deserialize_xml(&mut reader).unwrap();
    match (&data, &data_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_root"]
pub const deser_root: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_root"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(deser_root())),
};
fn deser_root() {
    let data = Root {
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
        child: Common {
            name: "child content".to_string(),
        },
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (
        &String::from_utf8_lossy(&buffer),
        &"<Root attribute=\"attribute content\">\
            <element>element content</element>\
            text content\
            <child><name>child content</name></child>\
        </Root>\
        ",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let data_copy = Root::deserialize_xml(&mut reader).unwrap();
    match (&data, &data_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_derive_flatten"]
pub const deser_derive_flatten: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_derive_flatten"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(deser_derive_flatten())),
};
fn deser_derive_flatten() {
    let data = Flatten {
        common: Common { name: "Name".to_string() },
        attribute: "attribute content".to_string(),
        element: "element content".to_string(),
        text: "text content".to_string(),
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (
        &String::from_utf8_lossy(&buffer),
        &"<Flatten attribute=\"attribute content\">\
            <name>Name</name>\
            <element>element content</element>\
            text content\
        </Flatten>\
        ",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let data_copy = Flatten::deserialize_xml(&mut reader).unwrap();
    match (&data, &data_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_derive_flatten_twice"]
pub const deser_derive_flatten_twice: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_derive_flatten_twice"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(deser_derive_flatten_twice())),
};
fn deser_derive_flatten_twice() {
    let data = FlattenTwice {
        field: Flatten {
            common: Common { name: "Name".to_string() },
            attribute: "attribute content".to_string(),
            element: "element content".to_string(),
            text: "text content".to_string(),
        },
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (
        &String::from_utf8_lossy(&buffer),
        &"<FlattenTwice attribute=\"attribute content\">\
            <name>Name</name>\
            <element>element content</element>\
            text content\
        </FlattenTwice>\
        ",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let data_copy = FlattenTwice::deserialize_xml(&mut reader).unwrap();
    match (&data, &data_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deser_derive_vec"]
pub const deser_derive_vec: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("deser_derive_vec"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(deser_derive_vec())),
};
fn deser_derive_vec() {
    struct Base {
        field: Vec<Common>,
    }
    impl SerXml for Base {
        fn serialize_xml<W: std::io::Write>(
            &self,
            serializer: &mut quick_xml::Writer<W>,
        ) -> Result<(), quick_xml::Error> {
            self.ser_as_element(serializer, "Base")
        }
        fn ser_elem_body<W: std::io::Write>(
            &self,
            serializer: &mut quick_xml::Writer<W>,
        ) -> Result<(), quick_xml::Error> {
            self.field.ser_as_element(serializer, "field")?;
            Ok(())
        }
        fn ser_elem_attributes(&self, elem: &mut quick_xml::events::BytesStart) {}
    }
    impl DeXml for Base {
        fn deserialize_xml<R: std::io::BufRead>(
            reader: &mut quick_xml::NsReader<R>,
        ) -> Result<Self, quick_xml::Error> {
            Self::deserialize_xml_from_tag(reader, "Base")
        }
        fn deserialize_xml_from_body<R: std::io::BufRead>(
            reader: &mut NsReader<R>,
            start: &BytesStart,
        ) -> Result<Self, quick_xml::Error> {
            let field: Vec<Common> = DeXml::deserialize_xml_from_tag(reader, "field")?;
            return Ok(Self { field });
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Base {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Base",
                "field",
                &&self.field,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Base {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Base {
        #[inline]
        fn eq(&self, other: &Base) -> bool {
            self.field == other.field
        }
    }
    let data = Base {
        field: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Common {
                    name: "Name 1".to_string(),
                },
                Common {
                    name: "Name 2".to_string(),
                },
            ]),
        ),
    };
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    data.serialize_xml(&mut writer).unwrap();
    let buffer = writer.into_inner().into_inner();
    match (
        &String::from_utf8_lossy(&buffer),
        &"<Base>\
            <field>\
            <Common><name>Name 1</name></Common>\
            <Common><name>Name 2</name></Common>\
            </field>\
        </Base>\
        ",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let mut reader = NsReader::from_reader(Cursor::new(buffer));
    let data_copy = Base::deserialize_xml(&mut reader).unwrap();
    match (&data, &data_copy) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &de_attribute_start_end,
            &de_flatten_attribute_start_end,
            &deser_attribute_empty,
            &deser_derive_common,
            &deser_derive_flatten,
            &deser_derive_flatten_twice,
            &deser_derive_vec,
            &deser_flatten_attribute_empty,
            &deser_root,
        ],
    )
}
