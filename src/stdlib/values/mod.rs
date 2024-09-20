use crate::multistackvm::VM;

pub mod value_len;
pub mod value_tag;

pub fn init_stdlib(vm: &mut VM) {
    value_len::init_stdlib(vm);
    value_tag::init_stdlib(vm);
}
