pub mod author;
pub mod book;
pub mod fragment;

#[macro_export]
macro_rules! make_error {
    ($kind:expr,$message:expr) => {
        Err(status::Custom($kind, $message))
    };
    ($kind:expr) => {
        Err(status::Custom($kind))
    };
}

#[macro_export]
macro_rules! json_val_or_error {
    ($result:expr) => {
        match $result {
            Ok(val) => Ok(Json(val)),
            Err(e) => {
                info!("Error: {}", e.to_string());
                make_error!(Status::InternalServerError, e.to_string())
            }
        }
    };
}

pub(crate) use json_val_or_error;
pub(crate) use make_error;
