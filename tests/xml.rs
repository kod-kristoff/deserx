use deserx::testing::{assert_ser_tokens, Token};
use deserx::{SerXml, XmlSerializeStruct, XmlSerializer};

#[test]
fn ser_attribute() {
    pub struct Contributor {
        resource: String,
    }

    impl SerXml for Contributor {
        fn serialize_xml<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: XmlSerializer,
        {
            let mut s = serializer.serialize_struct("Contributor")?;
            s.end()
        }
    }

    let val = Contributor {
        resource: "#A".into(),
    };

    assert_ser_tokens(
        &val,
        &[
            Token::Struct {
                name: "Contributor",
                len: 0,
            },
            Token::StructEnd,
        ],
    )
}
