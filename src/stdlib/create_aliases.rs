use crate::multistackvm::VM;

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_alias(".".to_string(), "return".to_string());
}
