#[derive(Debug)]
pub enum ApiError {
    BadRequest(&'static str),
    /// 面向管理员的参数错误
    BadRequestCode(&'static str, &'static str),
    /// OTA 包校验失败
    InvalidPackage(String),
    Unauthorized,
    /// 401 带业务码：如 session_replaced
    UnauthorizedCode(&'static str, String),
    Forbidden,
    NotFound,
    NotFoundCode(&'static str, &'static str),
    Gone,
    TooLarge,
    Conflict(&'static str),
    ConflictCode(&'static str, &'static str),
    /// 409
    ConflictCodeMsg(&'static str, String),
    TooManyRequests,
    /// sudo 语义的二次密码复核失败
    PasswordMismatch,
    Internal(String),
}

impl ApiError {
    fn parts(&self) -> (StatusCode, &'static str, String) {
        match self {
            ApiError::BadRequest(c) => (StatusCode::BAD_REQUEST, "invalid_params", c.to_string()),
            ApiError::BadRequestCode(c, m) => (StatusCode::BAD_REQUEST, c, m.to_string()),
            ApiError::InvalidPackage(m) => (StatusCode::BAD_REQUEST, "invalid_package", m.clone()),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "未登录或凭据失效".into(),
            ),
            ApiError::UnauthorizedCode(c, m) => (
                StatusCode::UNAUTHORIZED,
                c,
                m.clone(),
            ),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "forbidden", "权限不足".into()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not_found", "资源不存在".into()),
            ApiError::NotFoundCode(c, m) => (StatusCode::NOT_FOUND, c, m.to_string()),
            ApiError::Gone => (StatusCode::GONE, "expired", "已过保留期".into()),
            ApiError::TooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "too_large",
                "body 超限".into(),
            ),
            ApiError::Conflict(c) => (StatusCode::CONFLICT, c, c.to_string()),
            ApiError::ConflictCode(c, m) => (StatusCode::CONFLICT, c, m.to_string()),
            ApiError::ConflictCodeMsg(c, m) => (StatusCode::CONFLICT, c, m.clone()),
            ApiError::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limited",
                "失败次数过多，稍后再试".into(),
            ),
            ApiError::PasswordMismatch => (
                StatusCode::FORBIDDEN,
                "password_mismatch",
                "密码错误".into(),
            ),
            ApiError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, "internal", e.clone()),
        }
    }
}

// 枚举定义「世界有哪几种状态」，match 强制你回答「每种状态怎么办」。
// 穷尽性检查把问题从运行时踩坑提前到编译时报错

// 状态机