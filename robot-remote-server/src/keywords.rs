use dxr::{DxrError, Fault, TryFromValue, TryToValue, Value};
use dxr_server::HandlerResult;

use std::collections::HashMap;

fn annotate_err(err: &DxrError) {
    dbg!("Keyword Error: {:?}", err);
}

pub fn keyword_addone(value: &Value) -> HandlerResult {
    let params: Vec<i32>;
    match TryFromValue::try_from_value(&value) {
        Ok(p) => params = p,
        Err(e) => {
            annotate_err(&e);
            return Err(Fault::from(e));
        }
    }

    println!("Function Params {:?}", params);

    let argument: i32 = *params
        .get(0)
        .ok_or_else(|| Fault::new(400, format!("Can't parse parameter {:?}", params)))?;

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

pub fn keyword_strings_should_be_equal(value: &Value) -> HandlerResult {
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
pub fn keyword_count_items_in_directory(value: &Value) -> HandlerResult {
    let s1: Vec<String> = TryFromValue::try_from_value(&value).unwrap();
    println!("Function Params {:?}", s1);

    let mut response = HashMap::<&str, Value>::new();

    let status = "PASS";
    let result = 1;

    response.insert("return", result.try_to_value()?);
    response.insert("status", status.try_to_value()?);

    Ok(response.try_to_value()?)
}

// TODO Move to test code again
// TODO Simplfy validators
// TODO Validators for Errors
#[cfg(test)]
pub fn validate_response_success_return_i32(response: HandlerResult) {
    let response_val: Value = (response).expect("Can't parse response");
    let themap: std::collections::HashMap<String, Value> =
        TryFromValue::try_from_value(&response_val).expect("Can't parse response_val");
    //println!("{:#?}", response);

    let status = &themap["status"];
    // WTH rustc --explain E0790
    let stat = <String as TryFromValue>::try_from_value(status).expect("Can't convert status");
    assert_eq!(stat, "PASS");

    let return_value = &themap["return"];
    let return_val =
        <i32 as TryFromValue>::try_from_value(return_value).expect("Can't convert return_value");
    assert_eq!(return_val, 1);
}

#[cfg(test)]
mod tests {

    use super::*;

    // TODO Simplfy validators
    // TODO Validators for Errors

    fn validate_response_success(response: HandlerResult, output_expect: &str) {
        let response_val: Value = (response).expect("Can't parse response");
        let themap: std::collections::HashMap<String, Value> =
            TryFromValue::try_from_value(&response_val).expect("Can't parse response_val");

        let status = &themap["status"];
        // WTH rustc --explain E0790
        let stat = <String as TryFromValue>::try_from_value(status).expect("Can't convert status");
        assert_eq!(stat, "PASS");

        let output_map = &themap["output"];
        let output =
            <String as TryFromValue>::try_from_value(output_map).expect("Can't convert status");

        assert_eq!(output_expect, output);
    }

    fn validate_response_fail(response: HandlerResult, output_expect: &str) {
        let response_val: Value = (response).expect("Can't parse response");
        let themap: std::collections::HashMap<String, Value> =
            TryFromValue::try_from_value(&response_val).expect("Can't parse response_val");

        let status = &themap["status"];
        // WTH rustc --explain E0790
        let stat = <String as TryFromValue>::try_from_value(status).expect("Can't convert status");

        assert_ne!(stat, "PASS");

        let output_map = &themap["output"];
        let output =
            <String as TryFromValue>::try_from_value(output_map).expect("Can't convert status");
        assert_eq!(output_expect, output);

        let error_map = &themap["error"];
        let error =
            <String as TryFromValue>::try_from_value(error_map).expect("Can't convert status");
        assert_eq!("Given strings are not equal.", error);
    }

    fn validate_response_success_and_return_i32(
        response: HandlerResult,
        return_expect: i32,
        output_expect: &str,
    ) {
        let response_val: Value = (response).expect("Can't parse response");
        let themap: std::collections::HashMap<String, Value> =
            TryFromValue::try_from_value(&response_val).expect("Can't parse response_val");

        let status = &themap["status"];
        // WTH rustc --explain E0790
        let stat = <String as TryFromValue>::try_from_value(status).expect("Can't convert status");
        assert_eq!(stat, "PASS");

        let output_map = &themap["output"];
        let output =
            <String as TryFromValue>::try_from_value(output_map).expect("Can't convert status");
        assert_eq!(output_expect, output);

        let return_map = &themap["return"];
        let return_val =
            <i32 as TryFromValue>::try_from_value(return_map).expect("Can't convert status");
        assert_eq!(return_expect, return_val);
    }

    #[test]
    fn test_run_count_items_in_directory() {
        // TODO fix very ugly conversions
        let params_vec = vec![Value::string(String::from("/tmp"))];
        let params = TryToValue::try_to_value(&params_vec).unwrap();
        let response: HandlerResult = keyword_count_items_in_directory(&params);

        validate_response_success_return_i32(response);
    }

    #[test]
    fn test_run_strings_should_be_equal() {
        let s1 = "Equal";
        let s2 = "Equal";
        let params_vec = vec![
            Value::string(String::from(s1)),
            Value::string(String::from(s2)),
        ];
        let params = TryToValue::try_to_value(&params_vec).unwrap();

        let response: HandlerResult = keyword_strings_should_be_equal(&params);

        validate_response_success(response, &format!("Comparing '{}' to '{}'.", &s1, &s2));
    }

    #[test]
    fn test_run_strings_should_be_equal_fail() {
        let s1 = "Fail";
        let s2 = "Equal";
        let params_vec = vec![
            Value::string(String::from(s1)),
            Value::string(String::from(s2)),
        ];
        let params = TryToValue::try_to_value(&params_vec).unwrap();

        let response: HandlerResult = keyword_strings_should_be_equal(&params);

        validate_response_fail(response, &format!("Comparing '{}' to '{}'.", &s1, &s2));
    }

    #[test]
    fn test_run_addone_handler() {
        let params_vec = vec![Value::i4(88)];
        let params = TryToValue::try_to_value(&params_vec).unwrap();

        let response: HandlerResult = keyword_addone(&params);

        validate_response_success_and_return_i32(response, 89, &format!("Adding one to 88"));
    }
}
