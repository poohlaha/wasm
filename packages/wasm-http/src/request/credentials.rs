/*!
    `credentials` 是 `Request` 接口的只读属性，用于表示用户代理是否应该在跨域请求的情况下从其他域发送 `cookies`。
     这与 `XHR` 的 `withCredentials` 标志相似，不同的是有三个可选值（后者是两个）：
     - omit: 从不发送 `cookies`.
     - same-origin: 只有当 `URL` 与 `响应脚本同源` 才发送 `cookies`、`HTTP Basic authentication` 等验证信息.(浏览器默认值，在旧版本浏览器，例如 safari 11 依旧是 omit，safari 12 已更改)
     - include: 不论是不是跨域的请求，总是发送请求资源域在本地的 `cookies`、`HTTP Basic authentication` 等验证信息。
*/

use web_sys::RequestCredentials;

#[doc = "https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials"]
#[derive(Debug, Clone, Copy, Default)]
pub enum Credentials {
    /// 从不发送 cookies
    Omit,

    /// 只有当 `URL` 与 `响应脚本同源` 才发送 `cookies`、`HTTP Basic authentication` 等验证信息.(浏览器默认值，在旧版本浏览器，例如 safari 11 依旧是 omit，safari 12 已更改)
    #[default]
    SameOrigin,

    /// 不论是不是跨域的请求，总是发送请求资源域在本地的 `cookies`、`HTTP Basic authentication` 等验证信息。
    Include,
}

impl From<Credentials> for RequestCredentials {
    fn from(credentials: Credentials) -> Self {
        match credentials {
            Credentials::Omit => RequestCredentials::Omit,
            Credentials::SameOrigin => RequestCredentials::SameOrigin,
            Credentials::Include => RequestCredentials::Include,
        }
    }
}

impl Credentials {
    pub fn get_credentials(credentials: String) -> Credentials {
        let credentials = credentials.trim();
        if credentials.is_empty() {
            return Credentials::SameOrigin;
        }

        return match credentials.to_lowercase().as_str() {
            "omit" => Credentials::Omit,
            "include" => Credentials::Include,
            _ => Credentials::SameOrigin,
        };
    }
}
