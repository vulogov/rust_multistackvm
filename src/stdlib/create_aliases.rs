use crate::multistackvm::VM;

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_alias(".".to_string(), "return".to_string());
    let _ = vm.register_alias("!".to_string(), "execute".to_string());
    let _ = vm.register_alias("?".to_string(), "if".to_string());
    let _ = vm.register_alias("dup".to_string(), "dup_one".to_string());
    let _ = vm.register_alias("swap".to_string(), "swap_one".to_string());
    let _ = vm.register_alias(",".to_string(), "concat_with_space".to_string());
}
