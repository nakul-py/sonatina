use macros::inst_prop;

use crate::{inst, module::FuncRef, BlockId, Immediate, ValueId};

mod arith;
mod cast;
mod cmp;
mod control_flow;
mod logic;

#[inst_prop(Subset = "Interpretable")]
pub trait Interpret {
    fn interpret(&self, state: &mut dyn State) -> EvalValue;

    type Members = (
        inst::arith::Neg,
        inst::arith::Add,
        inst::arith::Sub,
        inst::arith::Mul,
        inst::arith::Sdiv,
        inst::arith::Udiv,
        inst::arith::Shl,
        inst::arith::Shr,
        inst::logic::Not,
        inst::logic::And,
        inst::logic::Or,
        inst::logic::Xor,
        inst::cast::Sext,
        inst::cast::Zext,
        inst::cast::Trunc,
        inst::cmp::Lt,
        inst::cmp::Gt,
        inst::cmp::Slt,
        inst::cmp::Sgt,
        inst::cmp::Le,
        inst::cmp::Ge,
        inst::cmp::Sle,
        inst::cmp::Sge,
        inst::cmp::Eq,
        inst::cmp::Ne,
        inst::cmp::IsZero,
        inst::control_flow::Jump,
        inst::control_flow::Br,
        inst::control_flow::BrTable,
        inst::control_flow::Phi,
        inst::control_flow::Call,
        inst::control_flow::Return,
    );
}

pub trait State {
    /// Retrieves the evaluated value associated with the given `ValueId`.
    ///
    /// This method looks up the value corresponding to the provided `ValueId`
    /// from the current state. It is typically used in an interpreter to
    /// fetch the result of a previously computed expression or constant.
    ///
    /// NOTE: If the value cannot be found or is uninitialized, this method is
    /// allowed to return an undefined (`Undef`) value. A state
    /// needs to decide how to deal with the situation(e.g., report an
    /// error, or cause a panic).
    fn lookup_val(&mut self, value: ValueId) -> EvalValue;

    fn eval_func(&mut self, func: FuncRef, args: Vec<EvalValue>) -> EvalValue;

    fn set_action(&mut self, action: Action);

    /// Returns the basic block that was executed immediately before
    /// the current block.
    ///
    /// This method retrieves the `BlockId` of the basic block that led to the
    /// current block.
    ///
    /// A state needs to handle the case where no previous block is found, and
    /// decide how to deal with the situation(e.g., report an error, or
    /// cause a panic).
    fn prev_block(&mut self) -> BlockId;
}

pub enum Action {
    Continue,
    JumpTo(BlockId),
    /// Indicate that branching instruction can't properly decide next
    /// destination.
    /// This happens e.g, the `BrTable` doesn't have a table entry that
    /// corresponds to scrutinee.
    FallThrough,
    Return(EvalValue),
}

#[derive(Clone, Copy, Default)]
pub enum EvalValue {
    Imm(Immediate),
    #[default]
    Undef,
}

impl EvalValue {
    pub fn with_imm<F, R>(self, f: F) -> Self
    where
        F: FnOnce(Immediate) -> R,
        R: Into<Self>,
    {
        match self {
            EvalValue::Imm(value) => f(value).into(),
            EvalValue::Undef => EvalValue::Undef,
        }
    }

    pub fn zip_with_imm<F, R>(lhs: Self, rhs: Self, f: F) -> Self
    where
        F: FnOnce(Immediate, Immediate) -> R,
        R: Into<Self>,
    {
        match (lhs, rhs) {
            (EvalValue::Imm(l), EvalValue::Imm(r)) => f(l, r).into(),
            _ => EvalValue::Undef,
        }
    }

    pub fn as_imm(&self) -> Option<Immediate> {
        match self {
            Self::Imm(imm) => Some(*imm),
            _ => None,
        }
    }
}

impl From<Immediate> for EvalValue {
    fn from(imm: Immediate) -> Self {
        Self::Imm(imm)
    }
}
