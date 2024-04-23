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

use dxr::{TryFromParams, TryFromValue, TryToValue, Value};
use dxr_server::{axum::http::HeaderMap, HandlerFn, HandlerResult, RouteBuilder, Server};

use std::collections::HashMap;

fn get_keyword_names_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    println!("get_keyword_names_handler {:?}", params);

    let response = vec![
        "Addone".to_string(),
        "Strings Should Be Equal".to_string(),
        "Count Items In Directory".to_string(),
    ];
    Ok(response.try_to_value()?)
}

fn run_addone_handler(value: &Value) -> HandlerResult {
    //TryFromValue::try_from_value(&value).unwrap_or_else(|_| println!("Oh-no, conversion failed"));
    let params: Vec<i32> = TryFromValue::try_from_value(&value)?;

    println!("Function Params {:?}", params);

    let argument: i32 = *params.get(0).unwrap();

    println!("Function Argument {:?}", argument);

    let result = argument + 1;

    let mut response = HashMap::<&str, Value>::new();
    response.insert("status", "PASS".try_to_value()?);
    response.insert("return", result.try_to_value()?);
    response.insert(
        "output",
        format!("Adding one to {}", argument).try_to_value()?,
    );

    Ok(response.try_to_value()?)
}

fn run_strings_should_be_equal(value: &Value) -> HandlerResult {
    let (s1, s2): (String, String) = TryFromValue::try_from_value(&value)?;
    println!("Function Argument {:?}", (&s1, &s2));
    let mut response = HashMap::<&str, Value>::new();

    let status;
    let error;
    let output;
    let traceback = "nice traceback";

    output = format!("Comparing '{}' to '{}'.", &s1, &s2);
    response.insert("output", output.try_to_value()?);

    if s1 == s2 {
        status = "PASS";
    } else {
        status = "FAIL";
        error = "Given strings are not equal.";
        response.insert("error", error.try_to_value()?);
        response.insert("traceback", traceback.try_to_value()?);
    };

    response.insert("status", status.try_to_value()?);

    Ok(response.try_to_value()?)
}

fn run_count_items_in_directory(value: &Value) -> HandlerResult {
    let s1: Vec<String> = TryFromValue::try_from_value(&value).unwrap();
    println!("Function Params {:?}", s1);

    let mut response = HashMap::<&str, Value>::new();

    let status = "PASS";
    let result = 1;

    response.insert("return", result.try_to_value()?);
    response.insert("status", status.try_to_value()?);

    Ok(response.try_to_value()?)
}

fn run_keyword_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    //let val = &params[0];
    println!("run_keyword_handler: {:?}", params);

    let (method_name, method_params): (Value, Value) = TryFromParams::try_from_params(params)?;

    println!("method_name as value: {:?}", method_name);
    println!("method_params as value: {:?}", method_params);

    let method_name: String = TryFromValue::try_from_value(&method_name)?;
    println!("method_name {:?}", method_name);

    let mut run_handler = HashMap::<&str, fn(&Value) -> HandlerResult>::new();

    run_handler.insert("Addone", run_addone_handler);
    run_handler.insert("Strings Should Be Equal", run_strings_should_be_equal);
    run_handler.insert("Count Items In Directory", run_count_items_in_directory);

    let response: HandlerResult;
    let fun: &fn(&Value) -> HandlerResult = run_handler.get(&method_name as &str).unwrap();
    response = fun(&method_params);

    println!("run_keyword_handler Response {:?}", response);
    response
}

#[tokio::main]
async fn main() {
    let route = RouteBuilder::new()
        .set_path("/RPC2")
        .add_method(
            "get_keyword_names",
            Box::new(get_keyword_names_handler as HandlerFn),
        )
        .add_method("run_keyword", Box::new(run_keyword_handler as HandlerFn))
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

    fn validate_response_sucess_i32(response: HandlerResult) {
        let response_val: Value = (response).unwrap();
        let themap: std::collections::HashMap<String, Value> =
            TryFromValue::try_from_value(&response_val).unwrap();
        //println!("{:#?}", response);

        let status = &themap["status"];
        let stat = <String as TryFromValue>::try_from_value(status).unwrap(); // WTH rustc --explain E0790
        assert_eq!(stat, "PASS");

        let return_value = &themap["return"];
        let return_val = <i32 as TryFromValue>::try_from_value(return_value).unwrap(); // WTH rustc --explain E0790
        assert_eq!(return_val, 1);
    }

    #[test]
    fn test_count_items_in_directory() {
        // TODO fix very ugly conversions
        let dir = String::from(
            "/home/micha/Repos/robot-remote-server-rs/tests/PythonRemoteServer_example",
        );
        let v = Value::string(dir);
        let values = TryToValue::try_to_value(&vec![v]).expect("Cannot convert");
        let params = vec![
            Value::string(String::from("Count Items In Directory")),
            values,
        ];
        let headers = HeaderMap::new();
        let response: HandlerResult = run_keyword_handler(&params, headers);

        validate_response_sucess_i32(response);
    }

    #[test]
    fn test_run_count_items_in_directory() {
        // TODO fix very ugly conversions
        let params = vec![Value::string(String::from("/tmp"))];
        let params2 = TryToValue::try_to_value(&params).unwrap();
        let response: HandlerResult = run_count_items_in_directory(&params2);

        validate_response_sucess_i32(response);
    }
}
