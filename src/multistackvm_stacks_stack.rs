use crate::multistackvm::*;
use easy_error::{Error};

impl VM {
    pub fn push_stacks(&mut self, name: String) -> Result<&mut VM, Error> {
        self.stacks_stack.push_back(name);
        Ok(self)
    }

    pub fn pop_stacks(&mut self) -> Option<String> {
        if self.stacks_stack.len() > 1 {
            return self.stacks_stack.pop_back();
        } else {
            return self.peek_stacks();
        }
    }

    pub fn peek_stacks(&mut self) -> Option<String> {
        match self.stacks_stack.back() {
            Some(last_stack) => {
                return Some(last_stack.clone());
            }
            None => {
                return None;
            }
        }
    }

    pub fn clear_stacks(&mut self) -> bool {
        if self.stacks_stack.len() > 1 {
            match self.pop_stacks() {
                Some(last_stack) => {
                    self.stacks_stack.clear();
                    let _ = self.push_stacks(last_stack);
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

}
