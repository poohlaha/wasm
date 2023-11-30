/*!
    `Request` 接口的一个只读属性 `referrerPolicy`，该属性返回引用策略（referrer policy）。
    引用策略规定了在请求中应该包含哪些引用信息（以 `Referer` 标头发送的信息）。
*/
use web_sys::ReferrerPolicy as RequestReferrerPolicy;

#[doc = "https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy"]
#[derive(Debug, Clone, Copy, Default)]
pub enum ReferrerPolicy {

    /// 对应于没有引用策略，会回退到其他地方定义的引用策略，或者在没有这样的更高级策略可用的情况下，回退到默认的引用策略。
    None,

    /// 指定不发送任何引用信息随请求发送到任何来源。
    NoReferrer,

    /// `no-referrer-when-downgrade` 策略发送请求的完整 `referrerURL`，用于请求：
    /// 其 `引用 URL ` 和 `当前URL` 都是 `潜在可信任的URL`
    /// 其 `引用URL` 是 `非潜在可信任的URL`
    NoReferrerWhenDowngrade,

    /// 指定当进行同源引用请求和跨源引用请求时，只发送请求的 referrerURL 的 ASCII 序列化作为引用信息。
    Origin,

    /// 指定当进行同源引用请求时，发送请求的完整 referrerURL 作为引用信息，而在进行跨源引用请求时，只发送请求的 referrerURL 的 ASCII 序列化。
    OriginWhenCrossOrigin,

    /// 指定当进行同源引用请求和跨源引用请求时，都发送请求的完整 referrerURL 作为引用信息。
    UnsafeUrl,

    /// 指定当进行同源引用请求时，发送请求的完整 referrerURL 作为引用信息。
    SameOrigin,

    /// `strict-origin` 策略发送请求的引用URL的起源的 `ASCII` 序列化，用于请求：
    /// 其 `引用 URL ` 和 `当前URL` 都是 `潜在可信任的URL`
    /// 其 `引用URL` 是 `非潜在可信任的URL`
    StrictOrigin,

    /// 指定当进行同源引用请求时，发送请求的完整 `referrerURL` 作为引用信息，而在进行跨源引用请求时，只发送请求的引用URL的起源的 `ASCII` 序列化，用于请求：
    /// 其 `引用 URL ` 和 `当前URL` 都是 `潜在可信任的URL`
    /// 其 `引用URL` 是 `非潜在可信任的URL`
    #[default]
    StrictOriginWhenCrossOrigin,
}


impl From<ReferrerPolicy> for RequestReferrerPolicy {
    fn from(value: ReferrerPolicy) -> Self {
        match value {
            ReferrerPolicy::None => RequestReferrerPolicy::None,
            ReferrerPolicy::NoReferrer => RequestReferrerPolicy::NoReferrer,
            ReferrerPolicy::NoReferrerWhenDowngrade => {
                RequestReferrerPolicy::NoReferrerWhenDowngrade
            }
            ReferrerPolicy::Origin => RequestReferrerPolicy::Origin,
            ReferrerPolicy::OriginWhenCrossOrigin => RequestReferrerPolicy::OriginWhenCrossOrigin,
            ReferrerPolicy::UnsafeUrl => RequestReferrerPolicy::UnsafeUrl,
            ReferrerPolicy::SameOrigin => RequestReferrerPolicy::SameOrigin,
            ReferrerPolicy::StrictOrigin => RequestReferrerPolicy::StrictOrigin,
            ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                RequestReferrerPolicy::StrictOriginWhenCrossOrigin
            }
        }
    }
}
