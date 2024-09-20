use crate::multistackvm::VM;

pub mod registry;


pub fn init_stdlib(vm: &mut VM) {
    registry::init_stdlib(vm);
}
