/// `Request` 接口的 `redirect` 只读属性包含了处理重定向的模式。
use web_sys::RequestRedirect;

#[doc = "https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect"]
#[derive(Debug, Clone, Copy, Default)]
pub enum Redirect {
    /// 跟随所有重定向，获取资源
    #[default]
    Follow,

    /// 当请求遇到重定向时返回网络错误
    Error,

    /**
       在请求遇到重定向时，获取一个不透明的、经过重定向筛选的响应，以便允许服务工作者在离线时重新执行重定向。
       从其他方面看，该响应与网络错误无法区分，以遵守原子HTTP重定向处理的要求。
    */
    Manual,
}

impl From<Redirect> for RequestRedirect {
    fn from(value: Redirect) -> Self {
        match value {
            Redirect::Follow => RequestRedirect::Follow,
            Redirect::Error => RequestRedirect::Error,
            Redirect::Manual => RequestRedirect::Manual,
        }
    }
}

impl Redirect {
    pub fn get_redirect(redirect: String) -> Redirect {
        let redirect = redirect.trim();
        if redirect.is_empty() {
            return Redirect::Follow;
        }

        return match redirect.to_lowercase().as_str() {
            "error" => Redirect::Error,
            "manual" => Redirect::Manual,
            _ => Redirect::Follow,
        };
    }
}
