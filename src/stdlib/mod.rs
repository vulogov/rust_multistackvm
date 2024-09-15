use crate::multistackvm::VM;

pub mod print;
pub mod alias;
pub mod autoadd;

pub fn init_stdlib(vm: &mut VM) {
    print::init_stdlib(vm);
    alias::init_stdlib(vm);
    autoadd::init_stdlib(vm);
}
