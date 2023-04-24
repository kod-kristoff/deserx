pub trait SerXml {
    fn serialize_xml<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: XmlSerializer;
}

pub trait XmlSerializer: Sized {
    type Ok;
    type Error: std::error::Error;
    type SerializeStruct: XmlSerializeStruct<Ok = Self::Ok, Error = Self::Error>;

    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct, Self::Error>;
}

pub trait XmlSerializeStruct {
    type Ok;
    type Error: std::error::Error;

    fn serialize_attribute(
        &mut self,
        key: &'static str,
        value: &'static str,
    ) -> Result<(), Self::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: SerXml;

    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error>;
}
