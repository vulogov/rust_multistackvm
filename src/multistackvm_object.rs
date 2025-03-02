use crate::multistackvm::*;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use easy_error::{bail, Error};

fn locate_value_in_object(name: String, value: Value) -> Option<Value> {
    match value.get(name.clone()) {
        Ok(value) => return Some(value),
        Err(_) => {}
    }
    let super_list = match value.get(".super") {
        Ok(super_list) => super_list,
        Err(_) => return None,
    };
    if super_list.type_of() != LIST {
        return None;
    }
    for s in super_list {
        match locate_value_in_object(name.clone(), s) {
            Some(v) => return Some(v),
            None => continue,
        }
    }
    return None;
}

impl VM {
    #[time_graph::instrument]
    pub fn value_locate(&mut self, name: String) -> Result<Value, Error> {
        let value = match self.stack.peek() {
            Some(value) => value,
            None => bail!("VM stack is empty"),
        };
        let method_value = match value.type_of() {
            OBJECT => match locate_value_in_object(name.clone(), value) {
                Some(method_value) => method_value,
                None => bail!("VM no method {} has been registered", &name),
            },
            _ => bail!("VM there is no value of type OBJECT in the stack"),
        };
        Ok(method_value)
    }

    #[time_graph::instrument]
    pub fn m(&mut self, name: String) -> Result<&mut VM, Error> {
        let value = match self.stack.peek() {
            Some(value) => value,
            None => bail!("VM stack is empty"),
        };
        let method_value = match value.type_of() {
            OBJECT => match locate_value_in_object(name.clone(), value) {
                Some(method_value) => method_value,
                None => bail!("VM no method {} has been registered", &name),
            },
            _ => bail!("VM there is no value of type OBJECT in the stack"),
        };
        match method_value.type_of() {
            STRING => {
                match method_value.cast_string() {
                    Ok(method_name) => {
                        if self.is_method(method_name.clone()) {
                            match self.get_method(method_name.clone()) {
                                Ok(fun) => {
                                    return fun(self);
                                }
                                Err(err) => {
                                    bail!("m({}) returned: {}", &name, err);
                                }
                            }
                        } else {
                            bail!("VM Method {} not registered", &method_name);
                        }
                    }
                    Err(err) => bail!("VM method name casting error: {}", err),
                }
            }
            LAMBDA => {
                return self.lambda_eval(method_value);
            }
            _ => bail!("VM method body is not acceptable for {}", &name),
        }
    }

}
