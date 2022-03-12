use serde::Serialize;

#[macro_export]
macro_rules! response {
    ($res:expr) => {
        Ok(match $res {
            Ok(res) => web::Json(Response::success(res)),
            Err(e) => web::Json(Response::error(e)),
        })
    };
}

#[derive(Serialize)]
pub struct Response<T> {
    success: bool,
    result: Option<T>,
    error: Option<String>,
}

impl<T> Response<T> {
    pub fn success(result: T) -> Self {
        Response {
            success: true,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Response {
            success: false,
            result: None,
            error: Some(error),
        }
    }
}
