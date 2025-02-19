use chrono::{DateTime, NaiveDateTime, TimeZone};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PriorityLevel {
    Low,
    Medium,
    High
}
impl PriorityLevel {
    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "low" => { Some(PriorityLevel::Low) },
            "medium" => { Some(PriorityLevel::Medium) },
            "high" => { Some(PriorityLevel::High) },
            _ => { None },
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            PriorityLevel::Low => { "low" },
            PriorityLevel::Medium => { "medium" },
            PriorityLevel::High => { "high" },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SameSiteSetting {
    Strict,
    Lax,
    None
}
impl SameSiteSetting {
    pub fn from_str(val: &str) -> Option<Self>{
        match val {
            "strict" => { Some(SameSiteSetting::Strict) },
            "lax" => { Some(SameSiteSetting::Lax) },
            "none" => { Some(SameSiteSetting::None) },
            _ => { None },
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            SameSiteSetting::Strict => { "strict" },
            SameSiteSetting::Lax => { "lax" },
            SameSiteSetting::None => { "none" },
        }
    }
}

/// Basic HTTP cookie parser and serializer for HTTP servers.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct  CookieSerializeOptions {
    name: String,

    /// Specifies the value for the {@link https://tools.ietf.org/html/rfc6265#section-5.2.3|Domain Set-Cookie attribute}. By default, no
    /// domain is set, and most clients will consider the cookie to apply to only
    /// the current domain.
    domain: Option<String>,

    /// Specifies a function that will be used to encode a cookie's value. Since
    /// value of a cookie has a limited character set (and must be a simple
    /// string), this function can be used to encode a value into a string suited
    /// for a cookie's value.
    pub encode: dyn Fn(String) -> String,


    /// Specifies the `Date` object to be the value for the {@link https://tools.ietf.org/html/rfc6265#section-5.2.1|`Expires` `Set-Cookie` attribute}. By default,
    /// no expiration is set, and most clients will consider this a "non-persistent cookie" and will delete
    /// it on a condition like exiting a web browser application.
    expires: Option<NaiveDateTime>,

    /// Specifies the boolean value for the {@link https://tools.ietf.org/html/rfc6265#section-5.2.6|`HttpOnly` `Set-Cookie` attribute}.
    /// When truthy, the `HttpOnly` attribute is set, otherwise it is not. By
    /// default, the `HttpOnly` attribute is not set.
    ///
    /// *Note* be careful when setting this to true, as compliant clients will
    /// not allow client-side JavaScript to see the cookie in `document.cookie`.
    ///
    /// @Default `true`
    http_only: bool,

    /// Specifies the number (in seconds) to be the value for the `Max-Age`
    /// `Set-Cookie` attribute. The given number will be converted to an integer
    /// by rounding down. By default, no maximum age is set.
    ///
    /// Overrides the expires property if set
    max_age: Option<i32>,

    /// Specifies the `boolean` value for the [`Partitioned` `Set-Cookie`](rfc-cutler-httpbis-partitioned-cookies)
    /// attribute. When truthy, the `Partitioned` attribute is set, otherwise it is not. By default, the
    /// `Partitioned` attribute is not set.
    /// More information about can be found in [the proposal](https://github.com/privacycg/CHIPS)
    ///
    /// **note** This is an attribute that has not yet been fully standardized, and may change in the future.
    /// This also means many clients may ignore this attribute until they understand it.
    partitioned: bool,

    /// Specifies the value for the {@link https://tools.ietf.org/html/rfc6265#section-5.2.4|`Path` `Set-Cookie` attribute}.
    /// By default, the path is considered the "default path".
    path: Option<String>,

    /// Specifies the `string` to be the value for the [`Priority` `Set-Cookie` attribute][rfc-west-cookie-priority-00-4.1].
    /// - `'low'` will set the `Priority` attribute to `Low`.
    /// - `'medium'` will set the `Priority` attribute to `Medium`, the default priority when not set.
    /// - `'high'` will set the `Priority` attribute to `High`.
    ///
    /// More information about the different priority levels can be found in
    /// [the specification][rfc-west-cookie-priority-00-4.1].
    ///
    /// **note** This is an attribute that has not yet been fully standardized, and may change in the future.
    /// This also means many clients may ignore this attribute until they understand it.
    priority: Option<PriorityLevel>,

    /// Specifies the boolean or string to be the value for the {@link https://tools.ietf.org/html/draft-ietf-httpbis-rfc6265bis-03#section-4.1.2.7|`SameSite` `Set-Cookie` attribute}.
    ///
    /// - `true` will set the `SameSite` attribute to `Strict` for strict same
    /// site enforcement.
    /// - `false` will not set the `SameSite` attribute.
    /// - `'lax'` will set the `SameSite` attribute to Lax for lax same site
    /// enforcement.
    /// - `'strict'` will set the `SameSite` attribute to Strict for strict same
    /// site enforcement.
    ///  - `'none'` will set the SameSite attribute to None for an explicit
    ///  cross-site cookie.
    ///
    /// More information about the different enforcement levels can be found in {@link https://tools.ietf.org/html/draft-ietf-httpbis-rfc6265bis-03#section-4.1.2.7|the specification}.
    ///
    /// *note* This is an attribute that has not yet been fully standardized, and may change in the future. This also means many clients may ignore this attribute until they understand it.
    same_site: Option<SameSiteSetting>,

    /// Specifies the boolean value for the {@link https://tools.ietf.org/html/rfc6265#section-5.2.5|`Secure` `Set-Cookie` attribute}. When truthy, the
    /// `Secure` attribute is set, otherwise it is not. By default, the `Secure` attribute is not set.
    ///
    /// *Note* be careful when setting this to `true`, as compliant clients will
    /// not send the cookie back to the server in the future if the browser does
    /// not have an HTTPS connection.
    ///
    /// @Default `false`
    secure: bool
}
impl Default for CookieSerializeOptions {
    fn default() -> Self {
        Self {
            name: String::frpm("greeno"),
            domain: None,
            encode: |v| v,
            expires: None,
            http_only: true,
            max_age: None,
            partitioned: false,
            path: None,
            priority: None,
            same_site: None,
            secure: false,
        }
    }
}
impl CookieSerializeOptions {
    pub fn name(&self) -> &str { &self.name}
    pub fn set_name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }

    pub fn domain(&self) -> &Option<String> { &self.domain}
    pub fn set_domain(&mut self, domain: Option<String>) -> &Self {
        self.domain = domain;
        self
    }

    pub fn expires(&self) -> Option<NaiveDateTime> { self.expires }
    pub fn set_expires(&mut self, expires: Option<NaiveDateTime>) -> &Self {
        self.expires = expires;
        self
    }

    pub fn http_only(&self) -> bool { self.http_only }
    pub fn set_http_only(&mut self, http_only: bool) -> &Self {
        self.http_only = http_only;
        self
    }

    pub fn max_age(&self) -> Option<i32> { self.max_age }
    pub fn set_max_age(&mut self, max_age: Option<i32>) -> &Self {
        self.max_age = max_age;
        self
    }

    pub fn partitioned(&self) -> bool { self.partitioned }
    pub fn set_partitioned(&mut self, partitioned: bool) -> &Self {
        self.partitioned = partitioned;
        self
    }

    pub fn path(&self) -> &Option<String> { &self.path }
    pub fn set_path(&mut self, path: Option<String>) -> &Self {
        self.path = path;
        self
    }

    pub fn priority(&self) -> &Option<PriorityLevel> { &self.priority }
    pub fn set_priority(&mut self, priority: Option<PriorityLevel>) -> &Self {
        self.priority = priority;
        self
    }

    pub fn same_site(&self) -> &Option<SameSiteSetting> { &self.same_site }
    pub fn set_same_site(&mut self, same_site: Option<SameSiteSetting>) -> &Self {
        self.same_site = same_site;
        self
    }

    pub fn secure(&self) -> bool { self.secure }
    pub fn set_secure(&mut self, secure: bool) -> &Self{
        self.secure = secure;
        self
    }
}
