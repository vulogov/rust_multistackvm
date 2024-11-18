use crate::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use rust_dynamic::math::Ops;
use easy_error::{Error, bail};

fn math_op(vm: &mut VM, value1: Value, value2: Value, op: StackOps, mop: Ops, err_prefix: String) -> Result<&mut VM, Error> {
    match Value::numeric_op(mop, value1, value2) {
        Ok(nvalue) => {
            match op {
                StackOps::FromStack => vm.stack.push(nvalue),
                StackOps::FromWorkBench => vm.stack.push_to_workbench(nvalue),
            };
        }
        Err(err) => {
            bail!("{} returns error: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}

pub fn stdlib_math_op_multiple_inline(vm: &mut VM, depth: usize, op: StackOps, mop: Ops, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < depth {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.current_stack_len() < depth {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
            if vm.stack.workbench.len() < 1 {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
    }
    loop {
        let first_operand_value = match op {
            StackOps::FromStack => vm.stack.pull(),
            StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
        };
        match first_operand_value {
            Some(value1) => {
                match vm.stack.pull() {
                    Some(value2) => {
                        if value2.type_of() == NODATA {
                            let _ = match op {
                                StackOps::FromStack => vm.stack.push(value1),
                                StackOps::FromWorkBench => vm.stack.push_to_workbench(value1),
                            };
                            break;
                        }
                        match math_op(vm, value1, value2, op.clone(), mop.clone(), err_prefix.clone()) {
                            Ok(_) => {}
                            Err(err) => {
                                bail!("{}", err);
                            }
                        }
                    }
                    None => {
                        let _ = match op {
                            StackOps::FromStack => vm.stack.push(value1),
                            StackOps::FromWorkBench => vm.stack.push_to_workbench(value1),
                        };
                        break;
                    }
                }
            }
            None => {
                bail!("{} can not get X", &err_prefix);
            }
        }
    }
    Ok(vm)
}

pub fn stdlib_math_op_inline(vm: &mut VM, depth: usize, op: StackOps, mop: Ops, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < depth {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.current_stack_len() < depth {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
            if vm.stack.workbench.len() < 1 {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
    }
    let first_operand_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let mmop = mop;
    match first_operand_value {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(mmop, value1, value2) {
                        Ok(nvalue) => {
                            match op {
                                StackOps::FromStack => vm.stack.push(nvalue),
                                StackOps::FromWorkBench => vm.stack.push_to_workbench(nvalue),
                            };
                        }
                        Err(err) => {
                            bail!("{} returns error: {}", &err_prefix, err);
                        }
                    }
                }
                None => {
                    bail!("{} returns: NO DATA #2", &err_prefix);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}
