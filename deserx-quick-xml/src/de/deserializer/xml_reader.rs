use std::{borrow::Cow, mem::replace, ops::Deref};

use quick_xml::{
    events::{BytesCData, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Decoder, Reader,
};

use crate::errors::DeError;

pub struct XmlReader<'i, R: XmlRead<'i>> {
    /// A source of low-level XML events
    reader: R,
    /// Intermediate event, that could be returned by the next call to `next()`.
    /// If that is the `Text` event then leading spaces already trimmed, but
    /// trailing spaces is not. Before the event will be returned, trimming of
    /// the spaces could be necessary
    lookahead: Result<PayloadEvent<'i>, DeError>,
}

impl<'i, R: XmlRead<'i>> XmlReader<'i, R> {
    pub fn new(mut reader: R) -> Self {
        // Lookahead by one event immediately, so we do not need to check in the
        // loop if we need lookahead or not
        let lookahead = reader.next();

        Self {
            reader,
            lookahead,
            // entity_resolver,
        }
    }

    /// Read next event and put it in lookahead, return the current lookahead
    #[inline(always)]
    fn next_impl(&mut self) -> Result<PayloadEvent<'i>, DeError> {
        replace(&mut self.lookahead, self.reader.next())
    }

    /// Returns `true` when next event is not a text event in any form.
    #[inline(always)]
    const fn current_event_is_last_text(&self) -> bool {
        // If next event is a text or CDATA, we should not trim trailing spaces
        !matches!(
            self.lookahead,
            Ok(PayloadEvent::Text(_)) | Ok(PayloadEvent::CData(_))
        )
    }

    /// Read all consequent [`Text`] and [`CData`] events until non-text event
    /// occurs. Content of all events would be appended to `result` and returned
    /// as [`DeEvent::Text`].
    ///
    /// [`Text`]: PayloadEvent::Text
    /// [`CData`]: PayloadEvent::CData
    fn drain_text(&mut self, mut result: Cow<'i, str>) -> Result<DeEvent<'i>, DeError> {
        loop {
            if self.current_event_is_last_text() {
                break;
            }

            match self.next_impl()? {
                PayloadEvent::Text(mut e) => {
                    if self.current_event_is_last_text() {
                        // FIXME: Actually, we should trim after decoding text, but now we trim before
                        e.inplace_trim_end();
                    }
                    result.to_mut().push_str(&e.unescape()?);
                    // .push_str(&e.unescape_with(|entity| self.entity_resolver.resolve(entity))?);
                }
                PayloadEvent::CData(e) => todo!(),
                // PayloadEvent::CData(e) => result.to_mut().push_str(&e.decode()?),

                // SAFETY: current_event_is_last_text checks that event is Text or CData
                _ => unreachable!("Only `Text` and `CData` events can come here"),
            }
        }
        Ok(DeEvent::Text(Text { text: result }))
    }

    /// Return an input-borrowing event.
    pub fn next(&mut self) -> Result<DeEvent<'i>, DeError> {
        loop {
            return match self.next_impl()? {
                PayloadEvent::Start(e) => Ok(DeEvent::Start(e)),
                PayloadEvent::End(e) => Ok(DeEvent::End(e)),
                PayloadEvent::Text(mut e) => {
                    if self.current_event_is_last_text() && e.inplace_trim_end() {
                        // FIXME: Actually, we should trim after decoding text, but now we trim before
                        continue;
                    }
                    self.drain_text(e.unescape()?)
                    // self.drain_text(e.unescape_with(|entity| self.entity_resolver.resolve(entity))?)
                }
                PayloadEvent::CData(e) => todo!(),
                // PayloadEvent::CData(e) => self.drain_text(e.decode()?),
                PayloadEvent::DocType(e) => {
                    // self.entity_resolver
                    //     .capture(e)
                    //     .map_err(|err| DeError::Custom(format!("cannot parse DTD: {}", err)))?;
                    continue;
                }
                PayloadEvent::Eof => Ok(DeEvent::Eof),
            };
        }
    }
}

/// Decoded and concatenated content of consequent [`Text`] and [`CData`]
/// events. _Consequent_ means that events should follow each other or be
/// delimited only by (any count of) [`Comment`] or [`PI`] events.
///
/// Internally text is stored in `Cow<str>`. Cloning of text is cheap while it
/// is borrowed and makes copies of data when it is owned.
///
/// [`Text`]: Event::Text
/// [`CData`]: Event::CData
/// [`Comment`]: Event::Comment
/// [`PI`]: Event::PI
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text<'a> {
    text: Cow<'a, str>,
}

impl<'a> Deref for Text<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.text.deref()
    }
}

impl<'a> From<&'a str> for Text<'a> {
    #[inline]
    fn from(text: &'a str) -> Self {
        Self {
            text: Cow::Borrowed(text),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Simplified event which contains only these variants that used by deserializer
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeEvent<'a> {
    /// Start tag (with attributes) `<tag attr="value">`.
    Start(BytesStart<'a>),
    /// End tag `</tag>`.
    End(BytesEnd<'a>),
    /// Decoded and concatenated content of consequent [`Text`] and [`CData`]
    /// events. _Consequent_ means that events should follow each other or be
    /// delimited only by (any count of) [`Comment`] or [`PI`] events.
    ///
    /// [`Text`]: Event::Text
    /// [`CData`]: Event::CData
    /// [`Comment`]: Event::Comment
    /// [`PI`]: Event::PI
    Text(Text<'a>),
    /// End of XML document.
    Eof,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Simplified event which contains only these variants that used by deserializer,
/// but [`Text`] events not yet fully processed.
///
/// [`Text`] events should be trimmed if they does not surrounded by the other
/// [`Text`] or [`CData`] events. This event contains intermediate state of [`Text`]
/// event, where they are trimmed from the start, but not from the end. To trim
/// end spaces we should lookahead by one deserializer event (i. e. skip all
/// comments and processing instructions).
///
/// [`Text`]: Event::Text
/// [`CData`]: Event::CData
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PayloadEvent<'a> {
    /// Start tag (with attributes) `<tag attr="value">`.
    Start(BytesStart<'a>),
    /// End tag `</tag>`.
    End(BytesEnd<'a>),
    /// Escaped character data between tags.
    Text(BytesText<'a>),
    /// Unescaped character data stored in `<![CDATA[...]]>`.
    CData(BytesCData<'a>),
    /// Document type definition data (DTD) stored in `<!DOCTYPE ...>`.
    DocType(BytesText<'a>),
    /// End of XML document.
    Eof,
}

impl<'a> PayloadEvent<'a> {
    /// Ensures that all data is owned to extend the object's lifetime if necessary.
    #[inline]
    fn into_owned(self) -> PayloadEvent<'static> {
        match self {
            PayloadEvent::Start(e) => PayloadEvent::Start(e.into_owned()),
            PayloadEvent::End(e) => PayloadEvent::End(e.into_owned()),
            PayloadEvent::Text(e) => PayloadEvent::Text(e.into_owned()),
            PayloadEvent::CData(e) => PayloadEvent::CData(e.into_owned()),
            PayloadEvent::DocType(e) => PayloadEvent::DocType(e.into_owned()),
            PayloadEvent::Eof => PayloadEvent::Eof,
        }
    }
}

/// Trait used by the deserializer for iterating over input. This is manually
/// "specialized" for iterating over `&[u8]`.
///
/// You do not need to implement this trait, it is needed to abstract from
/// [borrowing](SliceReader) and [copying](IoReader) data sources and reuse code in
/// deserializer
pub trait XmlRead<'i> {
    /// Return an input-borrowing event.
    fn next(&mut self) -> Result<PayloadEvent<'i>, DeError>;

    /// Skips until end element is found. Unlike `next()` it will not allocate
    /// when it cannot satisfy the lifetime.
    fn read_to_end(&mut self, name: QName) -> Result<(), DeError>;

    /// A copy of the reader's decoder used to decode strings.
    fn decoder(&self) -> Decoder;
}

/// XML input source that reads from a slice of bytes and can borrow from it.
///
/// You cannot create it, it is created automatically when you call
/// [`Deserializer::from_str`].
pub struct SliceReader<'de> {
    pub(crate) reader: Reader<&'de [u8]>,
    pub(crate) start_trimmer: StartTrimmer,
}

impl<'de> SliceReader<'de> {
    /// Returns the underlying XML reader.
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use serde::Deserialize;
    /// use quick_xml::de::Deserializer;
    /// use quick_xml::Reader;
    ///
    /// #[derive(Deserialize)]
    /// struct SomeStruct {
    ///     field1: String,
    ///     field2: String,
    /// }
    ///
    /// // Try to deserialize from broken XML
    /// let mut de = Deserializer::from_str(
    ///     "<SomeStruct><field1><field2></SomeStruct>"
    /// //   0                           ^= 28        ^= 41
    /// );
    ///
    /// let err = SomeStruct::deserialize(&mut de);
    /// assert!(err.is_err());
    ///
    /// let reader: &Reader<&[u8]> = de.get_ref().get_ref();
    ///
    /// assert_eq!(reader.error_position(), 28);
    /// assert_eq!(reader.buffer_position(), 41);
    /// ```
    pub const fn get_ref(&self) -> &Reader<&'de [u8]> {
        &self.reader
    }
}

impl<'de> XmlRead<'de> for SliceReader<'de> {
    fn next(&mut self) -> Result<PayloadEvent<'de>, DeError> {
        loop {
            let event = self.reader.read_event()?;
            if let Some(event) = self.start_trimmer.trim(event) {
                return Ok(event);
            }
        }
    }

    fn read_to_end(&mut self, name: QName) -> Result<(), DeError> {
        match self.reader.read_to_end(name) {
            Err(e) => Err(e.into()),
            Ok(_) => Ok(()),
        }
    }

    fn decoder(&self) -> Decoder {
        self.reader.decoder()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Helper struct that contains a state for an algorithm of converting events
/// from raw events to semi-trimmed events that is independent from a way of
/// events reading.
pub(crate) struct StartTrimmer {
    /// If `true`, then leading whitespace will be removed from next returned
    /// [`Event::Text`]. This field is set to `true` after reading each event
    /// except [`Event::Text`] and [`Event::CData`], so [`Event::Text`] events
    /// read right after them does not trimmed.
    trim_start: bool,
}

impl StartTrimmer {
    /// Converts raw reader's event into a payload event.
    /// Returns `None`, if event should be skipped.
    #[inline(always)]
    pub fn trim<'a>(&mut self, event: Event<'a>) -> Option<PayloadEvent<'a>> {
        let (event, trim_next_event) = match event {
            Event::DocType(e) => (PayloadEvent::DocType(e), true),
            Event::Start(e) => (PayloadEvent::Start(e), true),
            Event::End(e) => (PayloadEvent::End(e), true),
            Event::Eof => (PayloadEvent::Eof, true),

            // Do not trim next text event after Text or CDATA event
            Event::CData(e) => (PayloadEvent::CData(e), false),
            Event::Text(mut e) => {
                // If event is empty after trimming, skip it
                if self.trim_start && e.inplace_trim_start() {
                    return None;
                }
                (PayloadEvent::Text(e), false)
            }

            _ => return None,
        };
        self.trim_start = trim_next_event;
        Some(event)
    }
}

impl Default for StartTrimmer {
    #[inline]
    fn default() -> Self {
        Self { trim_start: true }
    }
}
