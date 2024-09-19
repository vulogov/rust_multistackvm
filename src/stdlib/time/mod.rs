use crate::multistackvm::VM;

pub mod timestamp;

pub fn init_stdlib(vm: &mut VM) {
    timestamp::init_stdlib(vm);
}
