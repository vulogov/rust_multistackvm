use crate::multistackvm::VM;

pub mod add;
pub mod mul;
pub mod sub;
pub mod div;

pub fn init_stdlib(vm: &mut VM) {
    add::init_stdlib(vm);
    mul::init_stdlib(vm);
    sub::init_stdlib(vm);
    div::init_stdlib(vm);
}
