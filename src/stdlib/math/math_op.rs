use crate::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use rust_dynamic::math::Ops;
use easy_error::{Error, bail};

pub fn stdlib_math_op_inline(vm: &mut VM, depth: usize, op: StackOps, mop: Ops, err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < depth {
        bail!("Stack is too shallow for inline {}", &err_prefix);
    }
    let first_operand_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match first_operand_value {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
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
