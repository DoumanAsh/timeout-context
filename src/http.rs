use crate::time;
use crate::TimeoutPropagation;
use crate::bytes::BytesWriter;

pub use http::HeaderMap;

use core::fmt;

impl TimeoutPropagation for HeaderMap {
    #[inline]
    fn set_timeout_ctx(&mut self, key: &str, value: time::Duration) {
        let mut out = BytesWriter::new();

        let key = http::HeaderName::from_bytes(key.as_bytes()).expect("Timestamp propagation header name is always valid");
        let _ = fmt::Write::write_fmt(&mut out, format_args!("{}", time::TimeoutValue(value)));
        self.insert(key, out.into());

    }

    #[inline]
    fn get_header_value(&self, key: &str) -> Option<&[u8]> {
        HeaderMap::get(&self, key).map(AsRef::as_ref)
    }
}

impl<T> TimeoutPropagation for http::Request<T> {
    #[inline]
    fn set_timeout_ctx(&mut self, key: &str, value: time::Duration) {
        TimeoutPropagation::set_timeout_ctx(self.headers_mut(), key, value)
    }
    #[inline]
    fn get_header_value(&self, key: &str) -> Option<&[u8]> {
        TimeoutPropagation::get_header_value(self.headers(), key)
    }
}
