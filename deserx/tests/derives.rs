mod string {
    use deserx::DeXml;
    use quick_xml::NsReader;

    #[test]
    fn from_empty() -> anyhow::Result<()> {
        let src = "<beskrivning/>";
        let mut reader = NsReader::from_str(src);
        let actual = String::deserialize_xml_from_tag(&mut reader, "beskrivning")?;
        assert!(actual.is_empty());
        Ok(())
    }
}
