use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum TypeCond {
    IfTrue,
    IfFalse,
}

#[time_graph::instrument]
fn stdlib_logic_ifthenelse_base(vm: &mut VM, cond: TypeCond, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 3 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.current_stack_len() < 2 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let then_lambda = match vm.stack.pull() {
        Some(then_lambda) => {
            if then_lambda.type_of() == LAMBDA {
                then_lambda
            } else {
                bail!("{}: #1 parameter must be lambda", &err_prefix);
            }
        },
        None => bail!("{} returns: NO DATA #1", &err_prefix),
    };
    let else_lambda = match vm.stack.pull() {
        Some(else_lambda) => {
            if else_lambda.type_of() == LAMBDA {
                else_lambda
            } else {
                bail!("{}: #2 parameter must be lambda", &err_prefix);
            }
        },
        None => bail!("{} returns: NO DATA #2", &err_prefix),
    };
    let cond_v = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match cond_v {
        Some(condition_val) => {
            match condition_val.cast_bool() {
                Ok(cond_bool) => {
                    match cond {
                        TypeCond::IfTrue => {
                            if cond_bool {
                                return vm.lambda_eval(then_lambda);
                            } else {
                                return vm.lambda_eval(else_lambda);
                            }
                        }
                        TypeCond::IfFalse => {
                            if ! cond_bool {
                                return vm.lambda_eval(else_lambda);
                            } else {
                                return vm.lambda_eval(then_lambda);
                            }
                        }
                    }
                }
                Err(err) => {
                    bail!("{}: #3 parameter must be boolean: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #3", &err_prefix);
        }
    }
}

pub fn stdlib_logic_ifthenelse(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_ifthenelse_base(vm, TypeCond::IfTrue, StackOps::FromStack, "IFTHENELSE".to_string())
}

pub fn stdlib_logic_ifthenelse_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_ifthenelse_base(vm, TypeCond::IfTrue, StackOps::FromWorkBench, "IFTHENELSE.".to_string())
}

pub fn stdlib_logic_ifthenelse_false(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_ifthenelse_base(vm, TypeCond::IfFalse, StackOps::FromStack, "NOTIFTHENELSE".to_string())
}

pub fn stdlib_logic_ifthenelse_in_workbench_false(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_ifthenelse_base(vm, TypeCond::IfFalse, StackOps::FromWorkBench, "NOTIFTHENELSE.".to_string())
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("ifthenelse".to_string(), stdlib_logic_ifthenelse);
    let _ = vm.register_inline("ifthenelse.".to_string(), stdlib_logic_ifthenelse_in_workbench);
    let _ = vm.register_inline("notifthenelse".to_string(), stdlib_logic_ifthenelse_false);
    let _ = vm.register_inline("notifthenelse.".to_string(), stdlib_logic_ifthenelse_in_workbench_false);
}
