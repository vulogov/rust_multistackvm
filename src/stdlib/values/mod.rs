use crate::multistackvm::VM;

pub mod value_len;
pub mod value_tag;
pub mod value_dict;
pub mod value_carcdr;

pub fn init_stdlib(vm: &mut VM) {
    value_len::init_stdlib(vm);
    value_tag::init_stdlib(vm);
    value_dict::init_stdlib(vm);
    value_carcdr::init_stdlib(vm);
}
