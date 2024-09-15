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

## What is autoadd?

When working with a VM, you may need to create a list or define a lambda and store some content. By default, the VM stores values on the stack or executes commands. The VM command ```:``` changes this behavior, bringing the VM into automated mode. In this mode, when the VM evaluates the data, instead of pushing the value to the stack, it pushes the data to the LIST or LAMBDA value on top of the current stack. For example, the Value::from(42.0) doesn't go to the stack but to the LIST stored earlier. The VM command ```;``` resets the VM behavior to the default. When casting LIST to Vector and checking an element in the list, we can verify that the autoadd feature is working.
