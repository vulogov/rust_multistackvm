# What's on your stack ?

rust_multistackvm is a Rust crate that implements a Virtual Machine operating on top of a two-dimensional stack rather than a single stack and registers. This two-dimensional stack structure, defined in the rust_multistack crate, offers functionality for the Rust programming language. It can contain dynamically typed values provided by the rust_dynamic crate, organized in a stack of stacks structure. This approach assists in achieving data isolation and control. At its core is a circular ring buffer of references on the stacks. The actual data stacks could be anonymous or named. Users can define and manipulate stacks and perform data manipulation through callbacks.

# Show me the code!

```rust
let mut vm = VM::new();
vm.apply(Value::from(42.0).unwrap()).unwrap();
let val = vm.stack.pull().expect("No pull() happens");
assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
```

After creating a Virtual Machine, we assigned it a number. The MultistackVM's default behavior is to push data to the current stack. If the passed value is a type of CALL, it executes the requested function. It's as simple as that. We retrieve data from the current stack upon evaluating the number, confirming its value as 42.0.

```rust
let mut vm = VM::new();
vm.apply(Value::call("list".to_string(), Vec::new())).unwrap();
vm.apply(Value::call(":".to_string(), Vec::new())).unwrap();
vm.apply(Value::from(42.0).unwrap()).unwrap();
vm.apply(Value::call(";".to_string(), Vec::new())).unwrap();
let val = vm.stack.pull().expect("No pull() happens");
let list_val = val.cast_list().unwrap();
assert_eq!(list_val[0].cast_float().unwrap(), 42.0 as f64);
```

This snippet demonstrated two important concepts:
- Execution of the VM functions;
- Autoadd feature.

When we pass ```Value::call("list".to_string(), Vec::new())``` to VM::apply(), the default behavior is to execute the VM function or command or lambda. Function ``` list``` is defined as a VM inline function, and this function puts an empty Value::list to the current stack.

## What is auto-add?

When working with a VM, you may need to create a list or define a lambda and store some content. By default, the VM stores values on the stack or executes commands. The VM command ```:``` changes this behavior, bringing the VM into automated mode. In this mode, when the VM evaluates the data, instead of pushing the value to the stack, it pushes the data to the LIST or LAMBDA value on top of the current stack. For example, the Value::from(42.0) doesn't go to the stack but to the LIST stored earlier. The VM command ```;``` resets the VM behavior to the default. When casting LIST to Vector and checking an element in the list, we can verify that the autoadd feature is working.

# How to control VM

All virtual machine (VM) controls are accessed through methods implemented within the VM struct. Data processing is carried out using inline functions defined for either the VM or stack TS structures. The TS inline functions primarily handle stack management, while the VM inline functions are dedicated to overall logic and data processing. A VM inline function is a self-contained function that takes a reference to the VM as a parameter and returns a ```Result<>``` containing a reference to the same VM or an Error. Here is an example of VM inline function implementation.

```rust
pub fn stdlib_print_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline clear_in()");
    }
    match vm.stack.pull() {
        Some(value) => {
            print!("{}", &value);
        }
        None => {
            bail!("PRINT returns: NO DATA");
        }
    }
    Ok(vm)
}
```

In this straightforward example, each inline function clearly defines how it consumes data from the stack and outlines the expected behavior of that data. And here is an example of how you register new inline function.

```rust
let _ = vm.register_inline("print".to_string(), stdlib_print_inline);
```

## Methods of VM struct

| Function name | Description |
|---|---|
| VM::new() | Creates a new instance of the VM. Adds one named stack "main" to serve as initial default stack. Imports all internal TS and VM standard libraries |
| VM::register_alias() | Creates alias for existing lambda or inline functions |
| VM::is_alias() | Check if alias exists |
| VM::unregister_alias() | Removes existing alias |
| VM::get_alias() | Return alias or raise an Error |
| VM::register_inline() | Register new inline functions |
| VM::is_inline() | Check if inline function exists |
| VM::unregister_inline() | Removes existing inline function |
| VM::get_inline() | Return inline function or raise an Error |
| VM::register_lambda() | Register new lambda functions |
| VM::is_lambda() | Check if lambda function exists |
| VM::unregister_lambda() | Removes existing lambda function |
| VM::get_lambda() | Return lambda function or raise an Error |
| VM::lambda_eval() | Evaluate lambda passed as parameter |
| VM::lambda_eval_in() | Evaluate lambda passed as parameter in the context of named stack |
| VM::apply() | Evaluate single Value in VM |
| VM::apply_in() | Evaluate single Value in VM in the context of named stack |

## Inline functions of Multistack VM

These inline functions are defined for the VM, with additional inline functions specifically defined for the stack. All of these are documented in the ```rust_multistack``` crate.

| Function name | Description |
|---|---|
| print | Takes a single value from stack and prints it |
| println | Takes a single value from stack and prints it with newline |
| space | prints a single space |
| nl | Prints a new line |
| alias| Takes two string values from stack and create alias for a function or lambda |
| unalias | Takes a single string from stack and remove alias referred by this name |
| list | Places an empty list to stack |
| lambda | Places an empty lambda to stack |
| ptr | Places an PTR reference to function to stack |
| execute | Takes PTR function from the stack and execute inline or lambda referred by this PTR |
| clear_stacks | Clear stack of stacks |
| drop_stacks | Drop top name of stack from stack of stacks |



## VM control functions

| Function name | Description |
|---|---|
| : | Turn on auto-add feature |
| ; | Turn off auto-add feature |

## Default aliases

Alias is basically another name for the function or lambda. Here is an example, where I am calling function "." that is an alias for inline function "return".

```rust
let mut vm = VM::new();
vm.apply(Value::from(42.0).unwrap()).unwrap();
vm.call(".".to_string()).unwrap();
let val = vm.stack.pull_from_workbench().unwrap();
assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
```

This alias was previously defined in stdlib/create aliases.rs as

```rust
let _ = vm.register_alias(".".to_string(), "return".to_string());
```
First, I am appying a Value::float to VM, that is pushed this value to the stack. Then I am calling for function "." Alias is getting resolved and inline function "return" defined in TS is called. As expected, we can pull the value from vm.stack Workbench.

| Alias | Context | Inline name | Description |
|---|---|---|---|
| . | TS | return | Pull value from current stack and push it to Workbench |
| ! | VM | execute | Execute inline function referred by PTR |
