use deserx::de_xml::{DeXml, Visitor};

#[derive(PartialEq, Debug)]
pub struct Common {
    pub name: String,
}

impl<'de> DeXml<'de> for Common {
    fn deserialize_xml<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: deserx::de_xml::XmlDeserializer<'de>,
    {
        struct CommonVisitor;

        impl<'de> Visitor<'de> for CommonVisitor {
            type Value = Common;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("data")
            }

            fn visit_element<A>(self, elem: A) -> Result<Self::Value, A::Error>
            where
                A: deserx::de_xml::ElemAccess<'de>,
            {
                todo!()
            }
        }

        deserializer.deserialize_element("Common", CommonVisitor)
    }
}
