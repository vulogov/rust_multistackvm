pub mod stdlib;
pub mod multistackvm;
pub mod multistackvm_inline;
pub mod multistackvm_methods;
pub mod multistackvm_object;
pub mod multistackvm_command;
pub mod multistackvm_alias;
pub mod multistackvm_lambdas;
pub mod multistackvm_classes;
pub mod multistackvm_vars;
pub mod multistackvm_lambda_eval;
pub mod multistackvm_lambda_eval_in;
pub mod multistackvm_apply;
pub mod multistackvm_apply_in;
pub mod multistackvm_call;
pub mod multistackvm_stacks_stack;
pub mod multistackvm_to_stack;
pub mod multistackvm_call_internal_word;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
