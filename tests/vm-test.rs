#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistackvm::multistackvm::VM;

    #[test]
    fn test_vm_new() {
        let vm = VM::new();
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn test_vm_stdlib() {
        let mut vm = VM::new();
        assert_eq!(vm.is_inline("print".to_string()), true);
    }

    #[test]
    fn test_vm_is_alias() {
        let mut vm = VM::new();
        assert_eq!(vm.is_alias("display".to_string()), true);
    }

    #[test]
    fn test_vm_stacks_stack1() {
        let mut vm = VM::new();
        assert_eq!(vm.peek_stacks().unwrap(), "main");
    }

    #[test]
    fn test_vm_stacks_stack2() {
        let mut vm = VM::new();
        vm.to_stack("TEST".to_string()).unwrap();
        assert_eq!(vm.peek_stacks().unwrap(), "TEST");
    }

    #[test]
    fn test_vm_version() {
        assert_eq!(rust_multistackvm::version(), env!("CARGO_PKG_VERSION").to_string());
    }

}
