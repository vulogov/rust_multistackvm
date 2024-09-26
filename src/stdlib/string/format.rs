use std::collections::HashMap;
use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use leon::{Template};
use easy_error::{Error, bail};

pub fn stdlib_string_format(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline format");
    }
    match vm.stack.pull() {
        Some(tpl_value) => {
            match tpl_value.cast_string() {
                Ok(str_tpl) => {
                    let template = match Template::parse(str_tpl.as_str()) {
                        Ok(template) => template,
                        Err(err) => {
                            bail!("FORMAT error parsing template: {}", err);
                        }
                    };
                    let mut values: HashMap<String, String> = HashMap::new();
                    for name in template.keys() {
                        if values.contains_key(&name.to_string().clone()) {
                            continue;
                        }
                        match vm.stack.pull() {
                            Some(value) => {
                                match value.conv(STRING) {
                                    Ok(str_val) => {
                                        match str_val.cast_string() {
                                            Ok(val) => {
                                                values.insert(name.to_string(), val);
                                            }
                                            Err(err) => {
                                                bail!("FORMAT error casting: {}", err);
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        bail!("FORMAT error converting: {}", err);
                                    }
                                }
                            }
                            None => {
                                bail!("FORMAT: stack is too shallow");
                            }
                        }
                    }
                    let res = match template.render(&values) {
                        Ok(res) => res.to_string(),
                        Err(err) => {
                            bail!("FORMAT error rendering: {}", err);
                        }
                    };
                    vm.stack.push(Value::from_string(res));
                }
                Err(err) => {
                    bail!("FORMAT return error: {}", err);
                }
            }
        }
        None => {
            bail!("FORMAT returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("format".to_string(), stdlib_string_format);
}
