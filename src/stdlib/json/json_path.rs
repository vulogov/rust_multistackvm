use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::JSON;
use serde_json_path::{JsonPath};
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
                                        let json_search_path = match JsonPath::parse(&search_path) {
                                            Ok(json_search_path) => json_search_path,
                                            Err(err) => {
                                                bail!("JSON.PATH compiling earch path returns: {}", err);
                                            }
                                        };
                                        let res = json_search_path.query(&value).all();
                                        let res_value = match serde_json::to_value(res.clone()) {
                                            Ok(res_value) => res_value,
                                            Err(err) => {
                                                bail!("JSON.PATH result conversion returns: {}", err);
                                            }
                                        };
                                        vm.stack.push(Value::json(res_value));
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
