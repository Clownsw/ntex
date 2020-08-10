use bytes::BytesMut;

/// Decoding of frames via buffers.
pub trait Decoder {
    /// The type of decoded frames.
    type Item;

    /// The type of unrecoverable frame decoding errors.
    ///
    /// If an individual message is ill-formed but can be ignored without
    /// interfering with the processing of future messages, it may be more
    /// useful to report the failure as an `Item`.
    type Error;

    /// Attempts to decode a frame from the provided buffer of bytes.
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;

    /// A default method available to be called when there are no more bytes
    /// available to be read from the underlying I/O.
    ///
    /// This method defaults to calling `decode` and returns an error if
    /// `Ok(None)` is returned while there is unconsumed data in `buf`.
    /// Typically this doesn't need to be implemented unless the framing
    /// protocol differs near the end of the stream.
    fn decode_eof(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => Ok(None),
        }
    }
}
