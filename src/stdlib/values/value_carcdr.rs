use crate::multistackvm::{VM, StackOps};
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum CarCdrOps {
    Car,
    Cdr,
    Head,
    Tail,
    At,
}

fn stdlib_carcdr_base(vm: &mut VM, op: StackOps, cop: CarCdrOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
    }
    let value_is = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let value = match value_is {
        Some(value) => value,
        None => {
            bail!("{} returned: NO DATA has been obtained", &err_prefix);
        }
    };
    match cop {
        CarCdrOps::Car => {
            match value.car() {
                Some(car_value) => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(car_value),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(car_value),
                    };
                }
                None => {
                    bail!("{} returned: NO DATA", &err_prefix);
                }
            }
        }
        CarCdrOps::Cdr => {
            match value.cdr() {
                Some(cdr_value) => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(cdr_value),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(cdr_value),
                    };
                }
                None => {
                    bail!("{} returned: NO DATA", &err_prefix);
                }
            }
        }
        CarCdrOps::Head => {
            let n_is = match op {
                StackOps::FromStack => vm.stack.pull(),
                StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
            };
            let n_value = match n_is {
                Some(n_value) => n_value,
                None => {
                    bail!("{} returned: NO DATA has been obtained", &err_prefix);
                }
            };
            let n = match n_value.cast_int() {
                Ok(n) => n,
                Err(err) => {
                    bail!("{} returned during index casting: {}", &err_prefix, err);
                }
            };
            match value.head(n) {
                Some(head_value) => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(head_value),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(head_value),
                    };
                }
                None => {
                    bail!("{} returned: NO DATA", &err_prefix);
                }
            }
        }
        CarCdrOps::Tail => {
            let n_is = match op {
                StackOps::FromStack => vm.stack.pull(),
                StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
            };
            let n_value = match n_is {
                Some(n_value) => n_value,
                None => {
                    bail!("{} returned: NO DATA has been obtained", &err_prefix);
                }
            };
            let n = match n_value.cast_int() {
                Ok(n) => n,
                Err(err) => {
                    bail!("{} returned during index casting: {}", &err_prefix, err);
                }
            };
            match value.tail(n) {
                Some(tail_value) => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(tail_value),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(tail_value),
                    };
                }
                None => {
                    bail!("{} returned: NO DATA", &err_prefix);
                }
            }
        }
        CarCdrOps::At => {
            let n_is = match op {
                StackOps::FromStack => vm.stack.pull(),
                StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
            };
            let n_value = match n_is {
                Some(n_value) => n_value,
                None => {
                    bail!("{} returned: NO DATA has been obtained", &err_prefix);
                }
            };
            let n = match n_value.cast_int() {
                Ok(n) => n,
                Err(err) => {
                    bail!("{} returned during index casting: {}", &err_prefix, err);
                }
            };
            match value.at(n) {
                Some(at_value) => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(at_value),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(at_value),
                    };
                }
                None => {
                    bail!("{} returned: NO DATA", &err_prefix);
                }
            }
        }
    }
    Ok(vm)
}

pub fn stdlib_value_car_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromStack, CarCdrOps::Car, "CAR".to_string())
}

pub fn stdlib_value_car_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromWorkBench, CarCdrOps::Car, "CAR.".to_string())
}

pub fn stdlib_value_cdr_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromStack, CarCdrOps::Cdr, "CDR".to_string())
}

pub fn stdlib_value_cdr_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromWorkBench, CarCdrOps::Cdr, "CDR.".to_string())
}

pub fn stdlib_value_head_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromStack, CarCdrOps::Head, "HEAD".to_string())
}

pub fn stdlib_value_head_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromWorkBench, CarCdrOps::Head, "HEAD.".to_string())
}

pub fn stdlib_value_tail_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromStack, CarCdrOps::Tail, "TAIL".to_string())
}

pub fn stdlib_value_tail_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromWorkBench, CarCdrOps::Tail, "TAIL.".to_string())
}

pub fn stdlib_value_at_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromStack, CarCdrOps::At, "AT".to_string())
}

pub fn stdlib_value_at_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_carcdr_base(vm, StackOps::FromWorkBench, CarCdrOps::At, "AT.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("car".to_string(), stdlib_value_car_stack);
    let _ = vm.register_inline("car.".to_string(), stdlib_value_car_workbench);
    let _ = vm.register_inline("cdr".to_string(), stdlib_value_cdr_stack);
    let _ = vm.register_inline("cdr.".to_string(), stdlib_value_cdr_workbench);
    let _ = vm.register_inline("head".to_string(), stdlib_value_head_stack);
    let _ = vm.register_inline("head.".to_string(), stdlib_value_head_workbench);
    let _ = vm.register_inline("tail".to_string(), stdlib_value_tail_stack);
    let _ = vm.register_inline("tail.".to_string(), stdlib_value_tail_workbench);
    let _ = vm.register_inline("at".to_string(), stdlib_value_at_stack);
    let _ = vm.register_inline("at.".to_string(), stdlib_value_at_workbench);
}
