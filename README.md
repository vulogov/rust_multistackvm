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
| print. | Takes a single value from workbench and prints it |
| println. | Takes a single value from workbench and prints it with newline |
| space | prints a single space |
| nl | Prints a new line |
| format | Taking a format string from the stack and then for all key sequentially start to get values from stack. Names are not important, but they will be resolved as they are allocated in the stack |
| format. | Taking a format string from the workbench and then for all key sequentially start to get values from stack. Names are not important, but they will be resolved as they are allocated in the stack |
| alias| Takes two string values from stack and create alias for a function or lambda |
| unalias | Takes a single string from stack and remove alias referred by this name |
| list | Places an empty list to stack |
| dict | Place empty dictionary to the stack |
| nodata | Place NODATA element to the stack |
| lambda | Places an empty lambda to stack |
| ptr | Places an PTR reference to function to stack |
| true | Places an TRUE boolean value to to stack |
| false | Places an FALSE boolean value to to stack |
| execute | Takes executable context information from the stack and execute inline or lambda referred by this information |
| execute. | Takes executable context information from workbench and execute inline or lambda referred by this information |
| resolve | Resolving lambda or inline function to PTR if function exists, raise an Error otherwise |
| len | Peeking for the value on the stack and pushing it's length |
| attribute | Assign attribute to the value on top of the stack |
| tag | Tag value on top of the stack |
| clear_stacks | Clear stack of stacks |
| drop_stacks | Drop top name of stack from stack of stacks |
| + | Mathematical add of two values on the stack. If the Value is of type LIST, appending or merging of lists will be executed. |
| - | Mathematical sub of two values on the stack |
| * | Mathematical mul of two values on the stack |
| / | Mathematical div of two values on the stack |
| +. | Mathematical add of value on workbench with value on the stack. Result is pushed to workbench |
| -. | Mathematical sub of value on workbench with value on the stack. Result is pushed to workbench |
| *. | Mathematical mul of value on workbench with value on the stack. Result is pushed to workbench |
| /. | Mathematical div of value on workbench with value on the stack. Result is pushed to workbench |

Example of the format function

```rust
41 42     // For format variables a
          // and b, a = 42, b = 41 since they are
          // resolved as allocated
"Answer is {a}, but not {b}. It is really {a}!" format
```
This snippet will leave foollowing string on the stack:

```rust
"Answer is 42, but not 41. It is really 42!"
```

Group of functions for boolean logic operations

| Function name | Description |
|---|---|
| not | Perform boolean NOT operation |
| and | Perform boolean AND operation |
| or | Perform boolean OR operation |

Group of function for comparing values on the stack. Returns a boolean value

| Function name | Description |
|---|---|
| == | EQ compare operation |
| != | NE compare operation |
| > | GT compare operation |
| < | LT compare operation |
| >= | GT or EQ compare operation |
| <= | LT or EQ compare operation |

```rust
1 0 <     // Will return TRUE on stack
```

Here is group of functions that is doing a conditional move of data in stack and workbench

| Function name | Description |
|---|---|
| ?. | Take lambda function from stack and execute it. If then top of the stack have a boolean TRUE, take the next value and move it to workbench |
| ?move | Take lambda function from stack and execute it. If then top of the stack have a boolean TRUE, take the name of the stack and value from the stack and move the value to named stack |

Here is some example

```rust
42.0              // Put some value to the stack
{ dup > 0 } ?.    // Check if value is more than 0
                  // Since logical operator consumes
                  // value from stack, we duplicate it inside lambda
                  // If test return TRUE, word "?." will move next value
                  // To workbench
```

The outcome of this snippet is value 42.0 stored in workbench if only this value is greater than 0

Here is the group of functions to work with application-defined anonymous and named functions

| Function name | Description |
|---|---|
| register | Register lambda function as named function |
| unregister | Remove named function |
| execute | Execute lambda function |

```rust
// First, let's define function and register it with name "FortyTwo"
:FortyTwo {
  42
} register
// And then let's execute it
FortyTwo
```

Here is the group of functions covering the application logic

| Function name | Description |
|---|---|
| if | This function will (or will not) execute lambda function stored in stack if stack having a boolean value TRUE |
| if. | This function will (or will not) execute lambda function stored in stack if workbench having a boolean value TRUE |
| if.stack | This function will (or will not) execute lambda function stored in stack if current stack matches the name |
| ?true | This is an alias for if function. Condition is taken from stack |
| ?true* | This is an alias for ifthenelse function. Condition is taken from stack and function executes then or else lambdas taken from stack |
| ?true. | This is an alias for if function. Condition is taken from workbench |
| ?true*. | This is an alias for ifthenelse. function. Condition is taken from workbench, then and else lambdas from the stack |
| ?false | This function will (or will not) execute lambda function stored in stack if stack having a boolean value FALSE |
| ?false. | This function will (or will not) execute lambda function stored in stack if workbench having a boolean value FALSE |
| loop | This function will loop over all elements of the list that is obtained from stack, push the value to the stack and evaluate lambda for each pushed value |
| loop. | This function will loop over all elements of the list that is obtained from workbench, push the value to the stack and evaluate lambda for each pushed value |
| while | This function will execute lambda function until detect FALSE value on the stack |
| while. | This function will execute lambda function until detect FALSE value on the workbench |
| for | This function will execute lambda function in the loop until it returns FALSE to stack |
| for. | This function will execute lambda function in the loop until it returns FALSE to workbench |
| do | This function will execute lambda function util current stack is empty |
| do. | This function will execute lambda function taken from workbench util current stack is empty |
| times | This function will execute lambda function number of times as specified by the value pushed from the stack |
| times. | This function will execute lambda function number of times as specified by the value pushed from the workbench |
| map | This function will execute lambda function over list of values obtained from stack and will return the list of values for which this function is applied to the stack. Note, function must leave value on stack |
| map. | This function will execute lambda function over list of values obtained from workbench and will return the list of values for which this function is applied to workbech. Note, function must leave value on stack |


Examples of use IF conditional functions

```rust
true
{
  // This code will be executed
}
if
```

```rust
true
{
  // This code will not be executed
}
?false
```

```rust
:main
{
  // This code will be executed only if name of the current stack is "main"
}
if.stack
```

Example of use LOOP function

```rust
[ 42 ] { . } loop
```

Example of WHILE function

```rust
true
{
  42.0 . false
}
while
```

After single loop over lambda, condition is met and number 42.0 is in workbench

Example of FOR function

```rust
{
  "This lambda function will be executed at least once " println
  false
} for
```
After single loop over lambda, condition is met.

Example of TIMES. function

```rust
0       // Send an initial value
3 .     // Send a counter to workbench
{
  +     // Perform math operation in lambda
} times.
```
Example of MAP function

```rust
[ "Hello World!"]   // First, let's define data
{ :string.upper } map
```
This snipper will return following value on the list ```[ "HELLO WORLD!"]```


This snippet will execute lambda function three times and leave number 3 on top of the stack.

In this example, we are pushing content of the list to stack and evaluate lambda after each push. This code will push integer 42 to workbench.

And here is functions that is working with dictionaries

| Function name | Description |
|---|---|
| dict | Create an empry dictionary and store it on stack |
| conditional | Create an empry conditional value and store it on stack |
| set | Set the value in dictionary. Alias for set is "," |
| get | Get value from dictionary |

```rust
// We create an empty dictionary
dict
  // Then provide a key-value
  :ANSWER
  42.0
  // And set the value. Dictionary is on top of stack
  set
  // Set the key and get the Value
  :ANSWER
  get
// This snippet shall render 42.0 on top of the stack
```


There are number of internal libraries for multistackvm

### Math library

| Function name | Description |
|---|---|
| float.NaN | Return float point NaN to stack |
| float.+Inf | Return float point +Inf to stack |
| float.-Inf | Return float point -Inf to stack |
| float.Pi| Return float point Pi to stack |
| float.E | Return float point E to stack |
| math.floor | Return floor() computed for float point to stack |
| math.abs | Return abs() computed for float point to stack |
| math.signum | Return signum() computed for float point to stack |
| math.cbrt | Return cbrt() - cubic root computed for float point to stack |
| math.ceil | Return ceil() computed for float point to stack |
| math.round | Return round() computed for float point to stack |
| math.fract | Return fract() computed for float point to stack |
| math.sqrt | Return sqrt() computed for float point to stack |
| math.sin | Return sin() computed for float point to stack |
| math.cos | Return cos() computed for float point to stack |
| math.tan | Return tan() computed for float point to stack |
| math.asin | Return asin() computed for float point to stack |
| math.acos | Return acos() computed for float point to stack |
| math.atan | Return atan() computed for float point to stack |
| math.sinh | Return sinh() computed for float point to stack |
| math.cosh | Return cosh() computed for float point to stack |
| math.tanh | Return tanh() computed for float point to stack |

### String library

| Function name | Description |
|---|---|
| string.upper | Return value stored in stack in UPPERCASE |
| string.lower | Return value stored in stack in LOWERCASE |
| string.title | Return value stored in stack in TITLE |
| string.camel | Return value stored in stack in CAMELCASE |
| string.snake | Return value stored in stack in SNAKECASE |


### Conversion library

Conversion library support both stack and workbench operations. For the word operating with workbench add "." to name. For example, the word ```convert.to_int.``` will perform conversion in workbench.

| Function name | Description |
|---|---|
| convert.to_string | Convert value from supported value types to STRING |
| convert.to_int | Convert value from supported value types to INT |
| convert.to_float | Convert value from supported value types to FLOAT |
| convert.to_bool | Convert value from supported value types to BOOL |
| convert.to_list | Convert value from supported value types to LIST |

### Time library

| Function name | Description |
|---|---|
| time.now | Return current timestamp to stack |
| time.timestamp | Create timestamp from INT that taken from stack |

### List library

| Function name | Description |
|---|---|
| car | Return first element of the list |
| cdr | Return reminder of the list |
| head | Return N elements from head of the list |
| tail | Return N elements from tail of the list |
| at | Return N-th elements from the list |

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
| , | DICT | set | Set value to dictionary |
