// Based on code from dxr
// https://github.com/ironthree/dxr/blob/main/dxr_tests/examples/server.rs

//! Docs:
//! =====
//!
//! https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#remote-library-interface
//!
//! https://https://docs.rs/dxr_client/latest/dxr_client/
//! https://docs.rs/dxr/latest/dxr/
//! https://docs.rs/dxr_server/latest/dxr_server/
//! https://docs.rs/dxr/latest/dxr/struct.Value.html
//!
//! cd /home/micha/Repos/PythonRemoteServer_abelikt
//! . venv/bin/activate
//! robot example/tests.robot

use dxr::{Fault, TryFromParams, TryFromValue, TryToValue, Value};
use dxr_server::{axum::http::HeaderMap, HandlerFn, HandlerResult, RouteBuilder, Server};

use std::collections::HashMap;

mod keywords;
use keywords::*;

struct KeywordDispatcher<'a> {
    run_handler: HashMap<&'a str, fn(&Value) -> HandlerResult>,
}

impl<'a> KeywordDispatcher<'a> {
    pub fn new() -> Self {
        Self {
            run_handler: HashMap::<&str, fn(&Value) -> HandlerResult>::new(),
        }
    }

    // TODO Make this dynamic
    fn get_keyword_names_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
        println!("get_keyword_names_handler {:?}", params);

        let response = vec![
            "Addone".to_string(),
            "Strings Should Be Equal".to_string(),
            "Count Items In Directory".to_string(),
        ];
        Ok(response.try_to_value()?)
    }

    fn insert(&mut self, key: &'a str, value: fn(&Value) -> HandlerResult) {
        match self.run_handler.insert(key, value) {
            Some(_) => println!("Inserted"),
            None => println!("Can't insert"),
        }
    }

    fn get(&self, key: &str) -> Option<fn(&Value) -> HandlerResult> {
        self.run_handler.get(key).copied()
    }

    pub fn run_keyword_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
        println!("run_keyword_handler: {:?}", params);

        let (method_name, method_params): (Value, Value) = TryFromParams::try_from_params(params)?;

        println!("method_name as value: {:?}", method_name);
        println!("method_params as value: {:?}", method_params);

        let method_name: String = TryFromValue::try_from_value(&method_name)?;
        println!("method_name {:?}", method_name);

        unsafe {
            DISPATCHER.insert("Addone", keyword_addone);
            DISPATCHER.insert("Strings Should Be Equal", keyword_strings_should_be_equal);
            DISPATCHER.insert("Count Items In Directory", keyword_count_items_in_directory);
        }

        let response: HandlerResult;
        unsafe {
            let fun: fn(&Value) -> HandlerResult = DISPATCHER.get(&method_name as &str).unwrap();
            response = fun(&method_params);
        }

        println!("run_keyword_handler Response {:?}", response);
        response
    }
}

use once_cell::sync::Lazy;

static mut DISPATCHER: Lazy<KeywordDispatcher> = Lazy::new(|| {
    let m = KeywordDispatcher::new();
    m
});

#[tokio::main]
async fn main() {

    // TODO How can we trick this into using our dispatcher instance?
    // That's the Fn interface we need to adhere:
    // pub type HandlerFn = fn(params: &[Value], headers: HeaderMap) -> HandlerResult

    let route = RouteBuilder::new()
        .set_path("/RPC2")
        .add_method(
            "get_keyword_names",
            Box::new(KeywordDispatcher::get_keyword_names_handler as HandlerFn),
        )
        .add_method(
            "run_keyword",
            Box::new(KeywordDispatcher::run_keyword_handler as HandlerFn),
        )
        .build();

    let mut server = Server::from_route(route);
    let barrier = server.shutdown_trigger();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        barrier.notify_one();
    });

    server
        .serve("0.0.0.0:8270".parse().unwrap())
        .await
        .expect("Failed to run server.")
}

#[cfg(test)]
mod tests {

    use super::*;

    use keywords::validate_response_success_return_i32;

    // TODO Simplfy validators
    // TODO Validators for Errors

    #[test]
    fn test_get_keyword_names_handler() {
        let val = &vec![String::from("nope").try_to_value().unwrap()];
        let response = KeywordDispatcher::get_keyword_names_handler(val, HeaderMap::new());

        let response_expect = vec![
            "Addone".to_string(),
            "Strings Should Be Equal".to_string(),
            "Count Items In Directory".to_string(),
        ];

        let themap: Vec<String> =
            TryFromValue::try_from_value(&response.expect("Can't parse response"))
                .expect("Cant convert response into HashMap");

        assert_eq!(response_expect, themap);
    }

    #[test]
    fn test_run_keyword_handler_with_count_items_in_directory() {
        // TODO fix very ugly conversions
        let dir = String::from(
            "/home/micha/Repos/robot-remote-server-rs/tests/PythonRemoteServer_example",
        );
        let v = Value::string(dir);
        let params_vec = TryToValue::try_to_value(&vec![v]).expect("Cannot convert");
        let params = vec![
            Value::string(String::from("Count Items In Directory")),
            params_vec,
        ];
        let headers = HeaderMap::new();
        let response: HandlerResult = KeywordDispatcher::run_keyword_handler(&params, headers);

        validate_response_success_return_i32(response);
    }
}
