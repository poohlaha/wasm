//! cache 作为Request 接口只读属性包含着请求的缓存模式。它控制着请求以何种方式与浏览器的 HTTP 缓存进行交互。

use web_sys::RequestCache;

#[doc = "https://developer.mozilla.org/zh-CN/docs/Web/API/Request/cache"]
#[derive(Debug, Clone, Copy, Default)]
pub enum Cache {

    /**
      浏览器从 HTTP 缓存中寻找匹配的请求
      - 如果缓存匹配上并且有效（ fresh）, 它将直接从缓存中返回资源。
      - 如果缓存匹配上但已经过期，浏览器将会使用传统（ conditional request ）的请求方式去访问远程服务器。如果服务器端显示资源没有改动，它将从缓存中返回资源。否则，如果服务器显示资源变动，那么重新从服务器下载资源更新缓存。
      - 如果缓存没有匹配，浏览器将会以普通方式请求，并且更新已经下载的资源缓存。
    */
    #[default]
    Default,

    /// 浏览器直接从远程服务器获取资源，不查看缓存，并且不会使用下载的资源更新缓存。
    NoStore,

    /// 浏览器直接从远程服务器获取资源，不查看缓存，然后使用下载的资源更新缓存
    Reload,

    /**
      浏览器在其 `HTTP` 缓存中寻找匹配的请求
      - 如果有匹配，无论是新的还是陈旧的，浏览器都会向远程服务器发出条件请求。如果服务器指示资源没有更改，则将从缓存中返回该资源。否则，将从服务器下载资源并更新缓存。
      - 如果没有匹配，浏览器将发出正常请求，并使用下载的资源更新缓存。
     */
    NoCache,

    /**
      浏览器在其 `HTTP` 缓存中寻找匹配的请求。
      - 如果有匹配项，不管是新匹配项还是旧匹配项，都将从缓存中返回。
      - 如果没有匹配，浏览器将发出正常请求，并使用下载的资源更新缓存。
    */
    ForceCache,

    /**
      浏览器在其 HTTP 缓存中寻找匹配的请求
      - 如果有匹配项 (新的或旧的)，则从缓存中返回
      - 如果没有匹配，浏览器将返回一个错误
    */
    OnlyIfCached,
}

impl From<Cache> for RequestCache {
    fn from(value: Cache) -> Self {
        match value {
            Cache::Default => RequestCache::Default,
            Cache::NoStore => RequestCache::NoStore,
            Cache::Reload => RequestCache::Reload,
            Cache::NoCache => RequestCache::NoCache,
            Cache::ForceCache => RequestCache::ForceCache,
            Cache::OnlyIfCached => RequestCache::OnlyIfCached,
        }
    }
}