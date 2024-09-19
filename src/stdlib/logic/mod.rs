use crate::multistackvm::VM;

pub mod if_fun;


pub fn init_stdlib(vm: &mut VM) {
    if_fun::init_stdlib(vm);
}
