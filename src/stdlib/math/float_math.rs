use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub enum Ops {
    Floor,
    Abs,
    Signum,
    Acos,
    Atan,
    Asin,
    Cbrt,
    Ceil,
    Round,
    Fract,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
}

pub fn stdlib_float_floor_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Floor)
}

pub fn stdlib_float_abs_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Abs)
}

pub fn stdlib_float_signum_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Signum)
}

pub fn stdlib_float_acos_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Acos)
}

pub fn stdlib_float_asin_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Asin)
}

pub fn stdlib_float_atan_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Atan)
}

pub fn stdlib_float_cbrt_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Cbrt)
}

pub fn stdlib_float_ceil_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Ceil)
}

pub fn stdlib_float_round_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Round)
}

pub fn stdlib_float_fract_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Fract)
}

pub fn stdlib_float_sqrt_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Sqrt)
}

pub fn stdlib_float_cos_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Cos)
}

pub fn stdlib_float_sin_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Sin)
}

pub fn stdlib_float_tan_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Tan)
}

pub fn stdlib_float_cosh_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Cosh)
}

pub fn stdlib_float_sinh_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Sinh)
}

pub fn stdlib_float_tanh_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_float_inline(vm, Ops::Tanh)
}

pub fn stdlib_float_inline(vm: &mut VM, op: Ops) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline float_op");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.cast_float() {
                Ok(fvalue) => {
                    match op {
                        Ops::Floor => {
                            vm.stack.push(Value::from_float(fvalue.floor()));
                        }
                        Ops::Abs => {
                            vm.stack.push(Value::from_float(fvalue.abs()));
                        }
                        Ops::Signum => {
                            vm.stack.push(Value::from_float(fvalue.signum()));
                        }
                        Ops::Acos => {
                            vm.stack.push(Value::from_float(fvalue.acos()));
                        }
                        Ops::Atan => {
                            vm.stack.push(Value::from_float(fvalue.atan()));
                        }
                        Ops::Asin => {
                            vm.stack.push(Value::from_float(fvalue.asin()));
                        }
                        Ops::Cbrt => {
                            vm.stack.push(Value::from_float(fvalue.cbrt()));
                        }
                        Ops::Ceil => {
                            vm.stack.push(Value::from_float(fvalue.ceil()));
                        }
                        Ops::Round => {
                            vm.stack.push(Value::from_float(fvalue.round()));
                        }
                        Ops::Fract => {
                            vm.stack.push(Value::from_float(fvalue.fract()));
                        }
                        Ops::Sin => {
                            vm.stack.push(Value::from_float(fvalue.sin()));
                        }
                        Ops::Cos => {
                            vm.stack.push(Value::from_float(fvalue.cos()));
                        }
                        Ops::Tan => {
                            vm.stack.push(Value::from_float(fvalue.tan()));
                        }
                        Ops::Sinh => {
                            vm.stack.push(Value::from_float(fvalue.sinh()));
                        }
                        Ops::Cosh => {
                            vm.stack.push(Value::from_float(fvalue.cosh()));
                        }
                        Ops::Tanh => {
                            vm.stack.push(Value::from_float(fvalue.tanh()));
                        }
                        Ops::Sqrt => {
                            vm.stack.push(Value::from_float(fvalue.sqrt()));
                        }
                    }
                }
                Err(err) => {
                    bail!("FLOAT_OP returns error: {}", err);
                }
            }
        }
        None => {
            bail!("FLOAT_OP returns: NO DATA #1");
        }
    }
    Ok(vm)
}




pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("math.floor".to_string(), stdlib_float_floor_inline);
    let _ = vm.register_inline("math.abs".to_string(), stdlib_float_abs_inline);
    let _ = vm.register_inline("math.signum".to_string(), stdlib_float_signum_inline);
    let _ = vm.register_inline("math.cbrt".to_string(), stdlib_float_cbrt_inline);
    let _ = vm.register_inline("math.ceil".to_string(), stdlib_float_ceil_inline);
    let _ = vm.register_inline("math.round".to_string(), stdlib_float_round_inline);
    let _ = vm.register_inline("math.fract".to_string(), stdlib_float_fract_inline);
    let _ = vm.register_inline("math.sqrt".to_string(), stdlib_float_sqrt_inline);
    let _ = vm.register_inline("math.sin".to_string(), stdlib_float_sin_inline);
    let _ = vm.register_inline("math.cos".to_string(), stdlib_float_cos_inline);
    let _ = vm.register_inline("math.tan".to_string(), stdlib_float_tan_inline);
    let _ = vm.register_inline("math.asin".to_string(), stdlib_float_asin_inline);
    let _ = vm.register_inline("math.acos".to_string(), stdlib_float_acos_inline);
    let _ = vm.register_inline("math.atan".to_string(), stdlib_float_atan_inline);
    let _ = vm.register_inline("math.sinh".to_string(), stdlib_float_sinh_inline);
    let _ = vm.register_inline("math.cosh".to_string(), stdlib_float_cosh_inline);
    let _ = vm.register_inline("math.tanh".to_string(), stdlib_float_tanh_inline);
}
