pub const DEFAULT_ADMIN_PORT: i32 = 8870;
pub const DENY_RESPONSE: &str = r#"{
    "response_code": -1,
    "response_object": "The request has been blocked by the Spire!"
}"#;
pub const NOT_FOUND: &str = r#"{
    "response_code": -1,
    "response_object": "The route could not be found in the Proxy!"
}"#;
pub const DEFAULT_FIXEDWINDOW_MAP_SIZE: i32 = 3;

pub const TIMER_WAIT_SECONDS: u64 = 5;
pub const DEFAULT_HTTP_TIMEOUT: u64 = 10;
pub const DEFAULT_TEMPORARY_DIR: &str = "temporary";
pub const GRPC_STATUS_HEADER: &str = "grpc-status";
pub const GRPC_STATUS_OK: &str = "0";
