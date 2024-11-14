use serde_json::{json, Value as JSONValue};
use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::JSON;
use jsonpath_rust::{JsonPathValue, JsonPath};
use easy_error::{Error, bail};


pub fn stdlib_json_path(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline json.path");
    }
    match vm.stack.pull() {
        Some(j_value) => {
            if j_value.is_type(JSON) {
                match j_value.cast_json() {
                    Ok(value) => {
                        match vm.stack.pull() {
                            Some(search_path_value) => {
                                match search_path_value.cast_string() {
                                    Ok(search_path) => {
                                        let mut res: Vec<JSONValue> = Vec::new();
                                        let json_path: JsonPath<JSONValue> = match JsonPath::try_from(search_path.as_str()) {
                                            Ok(path) => path,
                                            Err(err) => {
                                                bail!("JSON.PATH compilation of search path returns: {}", err);
                                            }
                                        };
                                        let slice_of_data:Vec<JsonPathValue<JSONValue>> = json_path.find_slice(&value);
                                        for s in slice_of_data {
                                            match s {
                                                JsonPathValue::Slice(value, _) => {
                                                    res.push(value.clone());
                                                }
                                                _ => continue,
                                            }
                                            println!("{:?}", &s);
                                        }
                                        vm.stack.push(Value::json(json!(res)));
                                    }
                                    Err(err) => {
                                        bail!("JSON.PATH casting search path returns: {}", err);
                                    }
                                }
                            }
                            None => {
                                bail!("JSON.PATH returns: NO DATA #2");
                            }
                        }
                    }
                    Err(err) => {
                        bail!("Error casting from JSON value: {}", err);
                    }
                }
            } else {
                bail!("JSON.PATH: #1 parameter must be JSON");
            }
        }
        None => {
            bail!("JSON.PATH returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("json.path".to_string(), stdlib_json_path);
}
