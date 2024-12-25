pub(crate) mod cache;
pub(crate) mod credentials;
pub(crate) mod mode;
pub(crate) mod redirect;
pub(crate) mod referrer_policy;

use crate::request::cache::Cache;
use crate::request::credentials::Credentials;
use crate::request::mode::Mode;
use crate::request::redirect::Redirect;
use crate::request::referrer_policy::ReferrerPolicy;
use web_sys::{AbortSignal, RequestInit};

#[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    pub cache: Option<Cache>,
    pub credentials: Option<Credentials>,
    pub integrity: Option<String>, // 资源完整性验证
    pub mode: Option<Mode>,
    pub redirect: Option<Redirect>,
    pub referrer: Option<String>,
    pub referrer_policy: Option<ReferrerPolicy>,
    pub(crate) signal: Option<AbortSignal>,
}

impl HttpRequest {
    pub fn new() -> Self {
        Default::default()
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)"]
    pub fn cache(mut self, cache: Cache) -> Self {
        self.cache = Some(cache);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)"]
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/zh-CN/docs/Web/API/AbortController)"]
    pub fn integrity(mut self, integrity: String) -> Self {
        self.integrity = Some(integrity);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)"]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/zh-CN/docs/Web/API/AbortController)"]
    pub fn redirect(mut self, redirect: Redirect) -> Self {
        self.redirect = Some(redirect);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)"]
    pub fn referrer(mut self, referrer: String) -> Self {
        self.referrer = Some(referrer);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)"]
    pub fn referrer_policy(mut self, referrer_policy: ReferrerPolicy) -> Self {
        self.referrer_policy = Some(referrer_policy);
        self
    }

    #[doc = "[MDN Documentation](https://developer.mozilla.org/zh-CN/docs/Web/API/AbortController)"]
    pub fn signal(mut self, signal: Option<&AbortSignal>) -> Self {
        if let Some(signal) = signal {
            self.signal = Some(signal.clone());
        } else {
            self.signal = None;
        }

        self
    }
}

impl From<HttpRequest> for RequestInit {
    fn from(value: HttpRequest) -> Self {
        let mut init = RequestInit::new();

        if let Some(cache) = value.cache {
            init.cache(cache.into());
        }

        if let Some(credentials) = value.credentials {
            init.credentials(credentials.into());
        }

        if let Some(ref integrity) = value.integrity {
            init.integrity(integrity);
        }

        if let Some(mode) = value.mode {
            init.mode(mode.into());
        }

        if let Some(redirect) = value.redirect {
            init.redirect(redirect.into());
        }

        if let Some(ref referrer) = value.referrer {
            init.referrer(referrer);
        }

        if let Some(referrer_policy) = value.referrer_policy {
            init.referrer_policy(referrer_policy.into());
        }

        if let Some(signal) = value.signal {
            init.signal(Some(&signal));
        }

        init
    }
}
