extern crate log;
use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    match value.get("run") {
        Ok(lambda_val) => {
            return vm.lambda_eval(lambda_val);
        }
        Err(err) => {
            log::debug!("CONDITIONAL:THROUGH returned error: {}", err);
        }
    }
    Ok(vm)
}
