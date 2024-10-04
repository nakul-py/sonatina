use sonatina_ir::inst::logic::*;

use super::{EvalValue, Interpret, State};

impl Interpret for Not {
    fn interpret(&self, state: &mut dyn State) -> EvalValue {
        let arg = state.lookup_val(*self.arg());
        arg.with_imm(|arg| !arg)
    }
}

impl Interpret for And {
    fn interpret(&self, state: &mut dyn State) -> EvalValue {
        let lhs = state.lookup_val(*self.lhs());
        let rhs = state.lookup_val(*self.rhs());

        EvalValue::zip_with_imm(lhs, rhs, |lhs, rhs| lhs & rhs)
    }
}

impl Interpret for Or {
    fn interpret(&self, state: &mut dyn State) -> EvalValue {
        let lhs = state.lookup_val(*self.lhs());
        let rhs = state.lookup_val(*self.rhs());

        EvalValue::zip_with_imm(lhs, rhs, |lhs, rhs| lhs | rhs)
    }
}

impl Interpret for Xor {
    fn interpret(&self, state: &mut dyn State) -> EvalValue {
        let lhs = state.lookup_val(*self.lhs());
        let rhs = state.lookup_val(*self.rhs());

        EvalValue::zip_with_imm(lhs, rhs, |lhs, rhs| lhs ^ rhs)
    }
}
