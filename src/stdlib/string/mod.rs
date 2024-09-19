use crate::multistackvm::VM;

pub mod case;

pub fn init_stdlib(vm: &mut VM) {
    case::init_stdlib(vm);
}
