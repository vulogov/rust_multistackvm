use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

fn make_bund_object(vm: &mut VM, name: String, value: Value) -> Result<Value, Error> {
    match value.type_of() {
        CLASS => {
            let mut res = match value.dup() {
                Ok(res) => res,
                Err(err) => bail!("VM error running DUP for the CLASS in OBJECT creation: {}", err),
            };
            let mut super_list = Value::list();
            let super_classes = match value.get(".super") {
                Ok(super_classes) => super_classes,
                Err(_) => Value::list(),
            };
            for o in super_classes {
                match o.cast_string() {
                    Ok(class_name) => {
                        if vm.is_class(class_name.clone()) {
                            let parent_value = match vm.get_class(class_name.clone()) {
                                Ok(class_value) => class_value,
                                Err(err) => bail!("VM error getting class {}: {}", &class_name, err),
                            };
                            match make_bund_object(vm, class_name.clone(), parent_value) {
                                Ok(parent_object) => {
                                    let init_lambda = match parent_object.get(".init") {
                                        Ok(init_lambda) => init_lambda,
                                        Err(_) => Value::lambda(),
                                    };
                                    match init_lambda.type_of() {
                                        STRING => {
                                            let init_method_name = match init_lambda.cast_string() {
                                                Ok(init_method_name) => init_method_name,
                                                Err(err) => bail!("Error casting init method name: {}", err),
                                            };
                                            if vm.is_method(init_method_name.clone()) {
                                                match vm.get_method(init_method_name.clone()) {
                                                    Ok(init_method) => {
                                                        match init_method(vm) {
                                                            Ok(_) => {},
                                                            Err(err) => bail!("CLASS {} constructor returns: {}", &class_name, err),
                                                        };
                                                    }
                                                    Err(err) => bail!("Error getting constructor for class {}: {}", &class_name, err),
                                                }
                                            }
                                        }
                                        LAMBDA => {
                                            match vm.lambda_eval(init_lambda) {
                                                Ok(_) => {},
                                                Err(err) => bail!("CLASS {} LAMBDA constructor returns: {}", &class_name, err),
                                            };
                                        }
                                        _ => log::debug!("Constructor for the class {} is not STRING or LAMBDA", &class_name),
                                    }
                                    super_list = super_list.push(parent_object);
                                }
                                Err(err) => bail!("VM error making parent object {}: {}", &class_name, err),
                            }
                        } else {
                            bail!("OBJECT class {} not registered", &name);
                        }
                    }
                    Err(err) => bail!("VM error casting class name: {}", err),
                }
            }
            res.dt = OBJECT;
            res = res.set(".class_name", Value::from_string(name.clone()));
            res = res.set(".super", super_list);
            return Ok(res);
        }
        OBJECT => {
            return Ok(value);
        }
        _ => bail!("VM source object for creating OBJECT is of incorrect type"),
    }
}

pub fn stdlib_object_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline object()");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    if vm.is_class(name.clone()) {
                        let object_value = match vm.get_class(name.clone()) {
                            Ok(class_value) => make_bund_object(vm, name, class_value),
                            Err(err) => bail!("VM error making object from class {}: {}", &name, err),
                        };
                        return match object_value {
                            Ok(obj) => vm.apply(obj),
                            Err(err) => bail!("VM OBJECT returns error: {}", err),
                        };
                    } else {
                        bail!("OBJECT class {} not registered", &name);
                    }
                }
                Err(err) => {
                    bail!("OBJECT returns error: {}", err);
                }
            }
        }
        None => {
            bail!("OBJECT returns: NO DATA");
        }
    }
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("object".to_string(), stdlib_object_inline);
}
