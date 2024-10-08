pub mod http;
pub mod http_client;
pub mod messages;
pub mod resources;

pub mod errors;
pub mod json;
mod json_schemas;
mod signature;

const DEFAULT_PROTOCOL_VERSION: &str = "2.0";

lazy_static::lazy_static! {
  pub(crate) static ref LOG_LEVEL: Option<String> = {
      std::env::var("TBDEX_SDK_LOG_LEVEL").ok()
  };
}

pub(crate) mod logging {
    #[macro_export]
    macro_rules! log_dbg {
        ($msg:expr, $($arg:tt)*) => {
            if let Some(ref level) = *$crate::LOG_LEVEL {
                if level == "DEBUG" {
                    println!("[DEBUG] {}", format!($msg, $($arg)*));
                }
            }
        };
        ($closure:expr) => {
            if let Some(ref level) = *$crate::LOG_LEVEL {
                if level == "DEBUG" {
                    let msg = $closure();
                    println!("[DEBUG] {}", msg);
                }
            }
        };
    }
}
