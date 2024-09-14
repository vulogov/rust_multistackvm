use crate::multistackvm::VM;

pub mod print;

pub fn init_stdlib(vm: &mut VM) {
    print::init_stdlib(vm);
}
