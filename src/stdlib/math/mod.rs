use crate::multistackvm::VM;

pub mod math_op;
pub mod add;
pub mod mul;
pub mod sub;
pub mod div;
pub mod float_math;
pub mod float;

pub fn init_stdlib(vm: &mut VM) {
    add::init_stdlib(vm);
    mul::init_stdlib(vm);
    sub::init_stdlib(vm);
    div::init_stdlib(vm);
    float_math::init_stdlib(vm);
    float::init_stdlib(vm);
}
