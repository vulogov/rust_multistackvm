use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

pub fn stdlib_logic_while(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline loop");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.stack.pull() {
                        Some(condition_val) => {
                            match condition_val.cast_bool() {
                                Ok(cond) => {
                                    if cond {
                                        match vm.lambda_eval(lambda_val.clone()) {
                                            Ok(_) => continue,
                                            Err(err) => {
                                                bail!("LOOP: lambda execution returns error: {}", err);
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    bail!("LOOP returns error: {}", err);
                                }
                            }
                        }
                        None => {
                            bail!("LOOP returns: NO DATA #2");
                        }
                    }
                }
            } else {
                bail!("LOOP: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("LOOP returns: NO DATA #1");
        }
    }
    Ok(vm)
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("while".to_string(), stdlib_logic_while);
}
