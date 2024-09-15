use crate::multistackvm::VM;
use easy_error::{Error, bail};


pub fn stdlib_alias_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline alias()");
    }
    match vm.stack.pull() {
        Some(alias_val) => {
            match vm.stack.pull() {
                Some(name_val) => {
                    match alias_val.cast_string() {
                        Ok(alias) => {
                            match name_val.cast_string() {
                                Ok(name) => {
                                    match vm.register_alias(alias, name) {
                                        Ok(_) => { return Ok(vm); }
                                        Err(err) => {
                                            bail!("ALIAS returns error: {}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("ALIAS on name returns: {}", err);
                                }
                            }
                        }
                        Err(err) => {
                            bail!("ALIAS on alias returns: {}", err);
                        }
                    }
                }
                None => {
                    bail!("ALIAS returns: NO DATA on name");
                }
            }
        }
        None => {
            bail!("ALIAS returns: NO DATA on alias");
        }
    }
}

pub fn stdlib_unalias_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline unalias()");
    }
    match vm.stack.pull() {
        Some(alias_val) => {
            match alias_val.cast_string() {
                Ok(alias) => {
                    match vm.unregister_alias(alias) {
                        Ok(_) => { return Ok(vm); }
                        Err(err) => {
                            bail!("UNALIAS returns error: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("UNALIAS on name returns: {}", err);
                }
            }
        }
        None => {
            bail!("UNALIAS returns: NO DATA on name");
        }
    }
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("alias".to_string(), stdlib_alias_inline);
    let _ = vm.register_inline("unalias".to_string(), stdlib_unalias_inline);
}
