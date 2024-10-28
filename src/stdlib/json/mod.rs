use crate::multistackvm::VM;

pub mod conversion;

pub fn init_stdlib(vm: &mut VM) {
    conversion::init_stdlib(vm);
}
