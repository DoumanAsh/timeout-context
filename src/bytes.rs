use bytes::BytesMut;

use core::fmt;

pub struct BytesWriter {
    inner: BytesMut
}

impl BytesWriter {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: BytesMut::new(),
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.inner.extend_from_slice(bytes);
    }

    #[inline]
    pub fn freeze(self) -> bytes::Bytes {
        self.inner.freeze()
    }
}

impl fmt::Write for BytesWriter {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.extend_from_slice(text.as_bytes());
        Ok(())
    }
}

#[cfg(feature = "http")]
impl From<BytesWriter> for http::HeaderValue {
    #[inline]
    fn from(value: BytesWriter) -> Self {
        unsafe {
            http::HeaderValue::from_maybe_shared_unchecked(
                value.freeze()
            )
        }
    }
}

#[cfg(feature = "tonic014")]
impl From<BytesWriter> for tonic014::metadata::MetadataValue<tonic014::metadata::Ascii> {
    #[inline]
    fn from(value: BytesWriter) -> Self {
        unsafe {
            tonic014::metadata::MetadataValue::from_shared_unchecked(
                value.freeze()
            )
        }
    }
}
