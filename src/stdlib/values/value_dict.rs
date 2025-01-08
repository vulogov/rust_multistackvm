use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_value_set(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for inline set");
    }
    match vm.stack.pull() {
        Some(d_val) => {
            match vm.stack.pull() {
                Some(tag_name_val) => {
                    let key_name = match tag_name_val.cast_string() {
                        Ok(key_name) => key_name,
                        Err(err) => {
                            bail!("SET key expected to be string: {}", err);
                        }
                    };
                    match vm.stack.pull() {
                        Some(mut value) => {
                            vm.stack.push(value.set(key_name, d_val));
                        }
                        None => {
                            bail!("SET returns: NO DATA #3");
                        }
                    }
                }
                None => {
                    bail!("SET returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("SET returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_value_get(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline get");
    }
    match vm.stack.pull() {
        Some(tag_name_val) => {
            let key_name = match tag_name_val.cast_string() {
                Ok(key_name) => key_name,
                Err(err) => {
                    bail!("GET key expected to be string: {}", err);
                }
            };
            match vm.stack.pull() {
                Some(value) => {
                    match value.get(key_name) {
                        Ok(d_val) => {
                            vm.stack.push(d_val);
                        }
                        Err(err) => {
                            bail!("GET returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("GET returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("GET returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_value_has_key(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline get");
    }
    match vm.stack.pull() {
        Some(tag_name_val) => {
            let key_name = match tag_name_val.cast_string() {
                Ok(key_name) => key_name,
                Err(err) => {
                    bail!("GET key expected to be string: {}", err);
                }
            };
            match vm.stack.pull() {
                Some(value) => {
                    match value.has_key(key_name) {
                        Ok(d_val) => {
                            vm.stack.push(value);
                            vm.stack.push(d_val);
                        }
                        Err(err) => {
                            bail!("HAS_KEY returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("HAS_KEY returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("HAS_KEY returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("set".to_string(), stdlib_value_set);
    let _ = vm.register_inline("get".to_string(), stdlib_value_get);
    let _ = vm.register_inline("?key".to_string(), stdlib_value_has_key);
}
