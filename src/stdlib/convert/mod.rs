use crate::multistackvm::VM;

pub mod internal;

pub fn init_stdlib(vm: &mut VM) {
    internal::init_stdlib(vm);
}
