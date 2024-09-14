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

}
