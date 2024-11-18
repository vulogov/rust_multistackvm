use crate::multistackvm::{VM, StackOps};
use crate::stdlib::math::math_op::{stdlib_math_op_inline, stdlib_math_op_multiple_inline};
use rust_dynamic::math::Ops;
use easy_error::{Error};

pub fn stdlib_sub_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_op_inline(vm, 2, StackOps::FromStack, Ops::Sub, "SUB".to_string())
}

pub fn stdlib_sub_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_op_inline(vm, 1, StackOps::FromWorkBench, Ops::Sub, "SUB.".to_string())
}

pub fn stdlib_sub_multiple_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_op_multiple_inline(vm, 2, StackOps::FromStack, Ops::Sub, "*SUB".to_string())
}

pub fn stdlib_sub_workbench_multiple_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_op_multiple_inline(vm, 1, StackOps::FromWorkBench, Ops::Sub, "*SUB.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("-".to_string(), stdlib_sub_inline);
    let _ = vm.register_inline("-.".to_string(), stdlib_sub_workbench_inline);
    let _ = vm.register_inline("*-".to_string(), stdlib_sub_multiple_inline);
    let _ = vm.register_inline("*-.".to_string(), stdlib_sub_workbench_multiple_inline);
}
