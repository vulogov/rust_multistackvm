use crate::multistackvm::VM;

pub mod registry;
pub mod resolve;


pub fn init_stdlib(vm: &mut VM) {
    registry::init_stdlib(vm);
    resolve::init_stdlib(vm);
}
