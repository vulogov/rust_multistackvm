use crate::multistackvm::VM;

pub mod conversion;
// pub mod json_path;

pub fn init_stdlib(vm: &mut VM) {
    conversion::init_stdlib(vm);
    // json_path::init_stdlib(vm);
}
