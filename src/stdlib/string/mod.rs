use crate::multistackvm::VM;

pub mod case;
pub mod concat_with_space;

pub fn init_stdlib(vm: &mut VM) {
    case::init_stdlib(vm);
    concat_with_space::init_stdlib(vm);
}
