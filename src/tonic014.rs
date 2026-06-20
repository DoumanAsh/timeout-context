use crate::time;
use crate::TimeoutPropagation;
use crate::bytes::BytesWriter;

use tonic014::metadata::{self, MetadataMap};

use core::fmt;

impl TimeoutPropagation for MetadataMap {
    #[inline]
    fn set_timeout_ctx(&mut self, key: &str, value: time::Duration) {
        let mut out = BytesWriter::new();

        let key: metadata::AsciiMetadataKey = key.parse().expect("Timestamp propagation header name is always valid");
        let _ = fmt::Write::write_fmt(&mut out, format_args!("{}", time::TimeoutValue(value)));
        self.insert(key, out.into());

    }

    #[inline]
    fn get_header_value(&self, key: &str) -> Option<&[u8]> {
        MetadataMap::get(&self, key).map(AsRef::as_ref)
    }
}

impl<T> TimeoutPropagation for tonic014::Request<T> {
    #[inline]
    fn set_timeout_ctx(&mut self, key: &str, value: time::Duration) {
        TimeoutPropagation::set_timeout_ctx(self.metadata_mut(), key, value)
    }
    #[inline]
    fn get_header_value(&self, key: &str) -> Option<&[u8]> {
        TimeoutPropagation::get_header_value(self.metadata(), key)
    }
}
