use crate::multistackvm::{VM, StackOps};
use crate::stdlib::math::math_op::stdlib_math_op_inline;
use rust_dynamic::math::Ops;
use easy_error::{Error};

pub fn stdlib_mul_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_op_inline(vm, 2, StackOps::FromStack, Ops::Mul, "MUL".to_string())
}

pub fn stdlib_mul_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_op_inline(vm, 1, StackOps::FromWorkBench, Ops::Mul, "MUL.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("*".to_string(), stdlib_mul_inline);
    let _ = vm.register_inline("*.".to_string(), stdlib_mul_workbench_inline);
}
