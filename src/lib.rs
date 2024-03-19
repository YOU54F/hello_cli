//! Pact file verification and schemas
use std::{ffi::CStr, str::FromStr};

use comfy_table::Table;
use log::{LevelFilter, SetLoggerError};
use pact_broker::types::{BrokerDetails, OutputType};
use serde_json::json;
use serde_json::Value;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::ffi::CString;
use std::os::raw::c_char;
mod cli;
mod pact_broker;
pub mod verification;
pub fn setup_loggers(level: &str) -> Result<(), SetLoggerError> {
    let log_level = match level {
        "none" => LevelFilter::Off,
        _ => LevelFilter::from_str(level).unwrap(),
    };
    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
}

pub fn glob_value(v: String) -> Result<(), String> {
    match glob::Pattern::new(&v) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("'{}' is not a valid glob pattern - {}", v, err)),
    }
}

#[no_mangle]
pub extern "C" fn ffi_broker_list_latest_pact_versions(args: *const c_char) -> *const c_char {
    // parse the args as json
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap() };
    let args_json: Value = match serde_json::from_str(args_str) {
        Ok(json) => json,
        Err(err) => {
            let error_msg = json!({
                "code": 1,
                "reason": "args_parse_error",
                "message": format!("Failed to parse args as JSON: {}", err),
            });
            return CString::new(error_msg.to_string()).unwrap().into_raw();
        }
    };

    let broker_details = BrokerDetails {
        auth: None, // TODO: Extract auth details from args_json
        url: match args_json["broker_base_url"].as_str() {
            Some(url) => url.to_string(),
            None => {
                let error_msg = json!({
                    "code": 1,
                    "reason": "missing_broker_url",
                    "message": "Missing 'broker_url' in args",
                });
                return CString::new(error_msg.to_string()).unwrap().into_raw();
            }
        },
    };
    let output_type = match args_json["output_type"].as_str() {
        Some("json") => OutputType::Json,
        Some("table") => OutputType::Table,
        Some("text") => OutputType::Text,
        Some("pretty") => OutputType::Pretty,
        _ => OutputType::Json, // default to JSON output
    };
    let verbose = args_json["verbose"].as_bool().unwrap_or(false);

    let res = pact_broker::list_latest_pact_versions::list_latest_pact_versions(
        &broker_details,
        output_type,
        verbose,
    );

    match res {
        Ok(result) => {
            // Convert the result to a C string
            // let result_str = CString::new(result).unwrap().into_raw();
            // Return success message along with the result string
            let success_msg = json!({
                "code": 0,
                "message": "success",
                "result": result,
            });
            CString::new(success_msg.to_string()).unwrap().into_raw()
        }
        Err(err) => {
            let error_msg = json!({
                "code": 1,
                "reason": "broker_error",
                "message": format!("Failed to list latest pact versions: {}", err),
            });
            return CString::new(error_msg.to_string()).unwrap().into_raw();
        }
    }
}

#[no_mangle]
pub extern "C" fn ffi_free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}
