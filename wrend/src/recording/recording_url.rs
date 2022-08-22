use std::ops::Deref;

use log::warn;
use web_sys::Url;

/// Wrapper around a raw string url. Should not be modified in place.
///
/// Releases url from window memory when done to prevent memory leak,
/// since manually created Urls do not get released automatically, unlike
/// most of web memory.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordingUrl(String);

impl RecordingUrl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Drop for RecordingUrl {
    fn drop(&mut self) {
        if let Err(err) = Url::revoke_object_url(&self.0) {
            warn!(
                "Error occurred while attempting to revoke the Url used for recorded video: {:?}",
                err
            );
        }
    }
}

impl<S: Into<String>> From<S> for RecordingUrl {
    fn from(string: S) -> Self {
        Self(string.into())
    }
}

impl Deref for RecordingUrl {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
