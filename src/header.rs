use http::{HeaderMap, HeaderName, HeaderValue};
use tauri::Runtime;

/// Extension methods that allow an [`AppHandle`] (or any [`tauri::Manager`])
/// to manage global HTTP headers entirely in memory.
pub trait HttpHeaderExt<R: Runtime> {
    fn set_http_header(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> crate::Result<()>;
    fn remove_http_header(&self, key: impl AsRef<str>) -> crate::Result<()>;
    fn clear_http_headers(&self);
    fn http_headers(&self) -> HeaderMap;
}

impl<R: Runtime, T: tauri::Manager<R>> HttpHeaderExt<R> for T {
    fn set_http_header(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> crate::Result<()> {
        use std::str::FromStr;
        let state = self.state::<crate::Http>();
        let mut headers = state.headers.lock().unwrap();
        let name = HeaderName::from_str(key.as_ref())?;
        let value = HeaderValue::from_str(value.as_ref())?;
        headers.insert(name, value);
        Ok(())
    }

    fn remove_http_header(&self, key: impl AsRef<str>) -> crate::Result<()> {
        use std::str::FromStr;
        let state = self.state::<crate::Http>();
        let mut headers = state.headers.lock().unwrap();
        let name = HeaderName::from_str(key.as_ref())?;
        headers.remove(name);
        Ok(())
    }

    fn clear_http_headers(&self) {
        let state = self.state::<crate::Http>();
        state.headers.lock().unwrap().clear();
    }

    fn http_headers(&self) -> HeaderMap {
        let state = self.state::<crate::Http>();
        let cloned = {
            let guard = state.headers.lock().unwrap();
            guard.clone()
        };
        cloned
    }
}