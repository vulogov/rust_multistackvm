use crate::multistackvm::VM;

pub mod if_fun;
pub mod loop_fun;
pub mod map_fun;
pub mod do_fun;
pub mod while_fun;
pub mod for_fun;
pub mod times_fun;
pub mod logic_ops_fun;
pub mod logic_compare_fun;

pub fn init_stdlib(vm: &mut VM) {
    if_fun::init_stdlib(vm);
    loop_fun::init_stdlib(vm);
    map_fun::init_stdlib(vm);
    do_fun::init_stdlib(vm);
    while_fun::init_stdlib(vm);
    for_fun::init_stdlib(vm);
    times_fun::init_stdlib(vm);
    logic_ops_fun::init_stdlib(vm);
    logic_compare_fun::init_stdlib(vm);
}
