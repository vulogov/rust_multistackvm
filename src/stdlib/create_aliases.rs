use crate::multistackvm::VM;

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_alias("display".to_string(), "print".to_string());
    let _ = vm.register_alias(".".to_string(), "return".to_string());
    let _ = vm.register_alias("!".to_string(), "execute".to_string());
    let _ = vm.register_alias("!.".to_string(), "execute.".to_string());
    let _ = vm.register_alias("?".to_string(), "if".to_string());
    let _ = vm.register_alias("dup".to_string(), "dup_one".to_string());
    let _ = vm.register_alias("swap".to_string(), "swap_one".to_string());
    let _ = vm.register_alias(",".to_string(), "concat_with_space".to_string());
    let _ = vm.register_alias("?true".to_string(), "if".to_string());
    let _ = vm.register_alias("?false".to_string(), "if.false".to_string());
    let _ = vm.register_alias("<-".to_string(), "stacks_left".to_string());
    let _ = vm.register_alias("←".to_string(), "stacks_left".to_string());
    let _ = vm.register_alias("->".to_string(), "stacks_right".to_string());
    let _ = vm.register_alias("→".to_string(), "stacks_right".to_string());
    let _ = vm.register_alias("<--".to_string(), "rotate_current_left".to_string());
    let _ = vm.register_alias("-->".to_string(), "rotate_current_right".to_string());
    let _ = vm.register_alias("λ".to_string(), "lambda".to_string());
    let _ = vm.register_alias("Λ".to_string(), "lambda".to_string());
    let _ = vm.register_alias("π".to_string(), "Pi".to_string());
    let _ = vm.register_alias("≠".to_string(), "!=".to_string());
    let _ = vm.register_alias("⩾".to_string(), ">=".to_string());
    let _ = vm.register_alias("⩽".to_string(), "<=".to_string());
}
