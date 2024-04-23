use dxr::{TryFromValue, TryToValue, Value};
use dxr_server::HandlerResult;

use std::collections::HashMap;

pub fn run_addone_handler(value: &Value) -> HandlerResult {
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

pub fn run_strings_should_be_equal(value: &Value) -> HandlerResult {
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

// TODO Make this less static
pub fn run_count_items_in_directory(value: &Value) -> HandlerResult {
    let s1: Vec<String> = TryFromValue::try_from_value(&value).unwrap();
    println!("Function Params {:?}", s1);

    let mut response = HashMap::<&str, Value>::new();

    let status = "PASS";
    let result = 1;

    response.insert("return", result.try_to_value()?);
    response.insert("status", status.try_to_value()?);

    Ok(response.try_to_value()?)
}
