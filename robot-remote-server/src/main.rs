// Based on code from dxr
// https://github.com/ironthree/dxr/blob/main/dxr_tests/examples/server.rs

//! # Simple example server
//!
//! Run this example with `cargo run --example server --features server`. It will listen on
//! <http://0.0.0.0:3000> for incoming XML-RPC requests.
//!
//! Testing this local server is straightforward, either with the `client` example included
//! in the `dxr_client` crate, or with three lines of python:
//!
//! ```python3
//! >>> import xmlrpc.client
//! >>> proxy = xmlrpc.client.ServerProxy("http://0.0.0.0:3000/")
//! >>> proxy.hello("DXR")
//! 'Handler type says: Hello, DXR!'
//! >>> proxy.countme()
//! 0
//! >>> proxy.countme()
//! 1
//! >>> proxy.countme()
//! 2
//! >>> proxy.add(1, 2)
//! 3
//! ```

/*

https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#remote-library-interface

https://https://docs.rs/dxr_client/latest/dxr_client/
https://docs.rs/dxr/latest/dxr/
https://docs.rs/dxr_server/latest/dxr_server/

https://docs.rs/dxr/latest/dxr/struct.Value.html

cd /home/micha/Repos/PythonRemoteServer_abelikt
. venv/bin/activate
robot example/tests.robot

*/

use std::sync::RwLock;

use dxr::{Fault, TryFromParams, TryFromValue, TryToValue, Value};
use dxr_server::{
    async_trait, axum::http::HeaderMap, Handler, HandlerFn, HandlerResult, RouteBuilder, Server,
};

struct CounterHandler {
    counter: RwLock<u32>,
}

impl CounterHandler {
    fn new(init: u32) -> CounterHandler {
        CounterHandler {
            counter: RwLock::new(init),
        }
    }
}

#[async_trait]
impl Handler for CounterHandler {
    async fn handle(&self, _params: &[Value], _headers: HeaderMap) -> HandlerResult {
        let mut value = self.counter.write().unwrap();
        let result = (*value as i32).try_to_value()?;
        *value += 1;
        Ok(result)
    }
}

fn hello_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let name = String::try_from_params(params)?;
    Ok(format!("Handler function says: Hello, {name}!").try_to_value()?)
}

fn adder_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let (a, b): (i32, i32) = TryFromParams::try_from_params(params)?;
    Ok((a + b).try_to_value()?)
}

//get_keyword_names
fn get_keyword_names_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    println!("get_keyword_names_handler {:?}", params);

    //let name = String::try_from_params(params)?;
    let response = vec![
        "addme".to_string(),
        "Strings Should Be Equal".to_string(),
        "Count Items In Directory".to_string(),
    ];
    Ok(response.try_to_value()?)
}

fn run_addme(argument: i32) -> HandlerResult {
    println!("Function Argument {:#?}", argument);

    let result = argument + 9;

    use std::collections::HashMap;
    let mut response = HashMap::<&str, Value>::new();
    response.insert("status", "PASS".try_to_value()?);
    response.insert("return", result.try_to_value()?);
    response.insert("output", "lalaland".try_to_value()?);

    Ok(response.try_to_value()?)
}

fn run_strings_should_be_equal(s1: &str, s2: &str) -> HandlerResult {
    println!("Function Argument {:#?}", (s1, s2));
    use std::collections::HashMap;
    let mut response = HashMap::<&str, Value>::new();

    let status = if s1 == s2 { "PASS" } else { "FAIL" };
    let result = if s1 == s2 {
        "Result pass"
    } else {
        "Result fail"
    };

    response.insert("status", status.try_to_value()?);
    response.insert("return", result.try_to_value()?);
    response.insert(
        "output",
        format!("Comparing '{}' to '{}'.", s1, s2).try_to_value()?,
    );

    Ok(response.try_to_value()?)
}

fn run_count_items_in_directory(s1: &Vec<String>) -> HandlerResult {
    let result = "Result, ist egal?";
    use std::collections::HashMap;
    let mut response = HashMap::<&str, Value>::new();

    let status = "PASS";

    response.insert("status", status.try_to_value()?);
    response.insert("return", result.try_to_value()?);
    response.insert(
        "output",
        format!("Super duper output from {}", s1[0]).try_to_value()?,
    );

    Ok(response.try_to_value()?)
}

fn run_keyword_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    // run_keyword_handler [Value { value: String("addme") }, Value { value: Array { data: ArrayData { values: [Value { value: Integer(33) }] } } }]
    println!("run_keyword_handler {:?}", params);
    println!("run_keyword_handler {:#?}", params);

    // https://docs.rs/dxr/0.6.2/dxr/struct.Value.html

    let val = &params[0];
    println!("param 0 {:?}", val);

    let val = Value::try_from_value(&params[0]).ok().unwrap();
    println!("name {:?}", val);

    let (a, b): (Value, Value) = TryFromParams::try_from_params(params)?;

    println!("params a {:?}", a);
    println!("params b {:#?}", b);

    let function: String = TryFromValue::try_from_value(&a)?;
    println!("Function {:#?}", function);

    let response: HandlerResult;
    if function == "addme" {
        TryFromValue::try_from_value(&b).unwrap_or_else(|_| println!("Oh-no, conversion failed"));
        let params: Vec<i32> = TryFromValue::try_from_value(&b)?;
        println!("Function Params {:#?}", params);

        let argument: i32 = *params.get(0).unwrap();
        response = run_addme(argument);
    } else if function == "Strings Should Be Equal" {
        let (s1, s2): (String, String) = TryFromValue::try_from_value(&b)?;
        println!("Function Params {:#?}", params);

        response = run_strings_should_be_equal(&s1, &s2);
    } else if function == "Count Items In Directory" {
        //let s1 : String = TryFromValue::try_from_value(&b)?;
        let s1: Vec<String> = TryFromValue::try_from_value(&b).unwrap();
        println!("Function Params {:#?}", params);
        response = run_count_items_in_directory(&s1);
        println!("Response {:#?}", response);
    } else {
        response = Err(Fault::new(42, format!("Ooops keyword {}", function)));
    }

    println!("Response {:#?}", response);
    response
}

#[tokio::main]
async fn main() {
    let counter_handler = CounterHandler::new(0);

    let route = RouteBuilder::new()
        .set_path("/RPC2")
        .add_method("hello", Box::new(hello_handler as HandlerFn))
        .add_method("countme", Box::new(counter_handler))
        .add_method("add", Box::new(adder_handler as HandlerFn))
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
