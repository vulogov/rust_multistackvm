use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_value_attr(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline attribute");
    }
    match vm.stack.pull() {
        Some(attr_val) => {
            match vm.stack.pull() {
                Some(mut value_val) => {
                    vm.stack.push(value_val.attr_add(attr_val));
                }
                None => {
                    bail!("ATTRIBUTE returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("ATTRIBUTE returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_value_tag(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for inline tag");
    }
    match vm.stack.pull() {
        Some(tag_val) => {
            let stag_val = match tag_val.cast_string() {
                Ok(stag_val) => stag_val,
                Err(err) => {
                    bail!("TAG value expected to be string: {}", err);
                }
            };
            match vm.stack.pull() {
                Some(tag_name_val) => {
                    let stag_name = match tag_name_val.cast_string() {
                        Ok(stag_name) => stag_name,
                        Err(err) => {
                            bail!("TAG key expected to be string: {}", err);
                        }
                    };
                    match vm.stack.pull() {
                        Some(mut value) => {
                            value.set_tag(stag_name, stag_val);
                            vm.stack.push(value);
                        }
                        None => {
                            bail!("TAG returns: NO DATA #3");
                        }
                    }
                }
                None => {
                    bail!("TAG returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("TAG returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("attribute".to_string(), stdlib_value_attr);
    let _ = vm.register_inline("tag".to_string(), stdlib_value_tag);
}
