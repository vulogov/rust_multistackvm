use crate::multistackvm::VM;

pub mod if_fun;
pub mod loop_fun;
pub mod while_fun;

pub fn init_stdlib(vm: &mut VM) {
    if_fun::init_stdlib(vm);
    loop_fun::init_stdlib(vm);
    while_fun::init_stdlib(vm);
}
