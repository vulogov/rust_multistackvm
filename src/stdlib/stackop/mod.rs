use crate::multistackvm::VM;

pub mod conditional_move;

pub fn init_stdlib(vm: &mut VM) {
    conditional_move::init_stdlib(vm);
}
