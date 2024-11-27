use std::io;

use macros::Inst;
use smallvec::SmallVec;

use super::{Inst, InstWrite};
use crate::{inst::impl_inst_write, ir_writer::FuncWriteCtx, module::FuncRef, Type, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Inst)]
#[inst(side_effect(super::SideEffect::Read))]
pub struct Mload {
    #[inst(value)]
    addr: ValueId,
    ty: Type,
}
impl_inst_write!(Mload, (addr: ValueId, ty: Type));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Inst)]
#[inst(side_effect(super::SideEffect::Write))]
pub struct Mstore {
    #[inst(value)]
    addr: ValueId,
    #[inst(value)]
    value: ValueId,
    ty: Type,
}
impl_inst_write!(Mstore, (value: ValueId, addr: ValueId, ty: Type));

#[derive(Debug, Clone, PartialEq, Eq, Hash, Inst)]
pub struct Gep {
    #[inst(value)]
    values: SmallVec<[ValueId; 8]>,
}
impl_inst_write!(Gep);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Inst)]
pub struct GetFunctionPtr {
    func: FuncRef,
}
impl InstWrite for GetFunctionPtr {
    fn write(&self, ctx: &FuncWriteCtx, w: &mut dyn io::Write) -> io::Result<()> {
        let name = self.as_text();
        ctx.func.ctx().func_sig(self.func, |sig| {
            let callee = sig.name();
            write!(w, "{name} %{callee}")
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Inst)]
#[inst(side_effect(super::SideEffect::Write))]
pub struct Alloca {
    ty: Type,
}
impl_inst_write!(Alloca);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Inst)]
pub struct InsertValue {
    #[inst(value)]
    dest: ValueId,
    #[inst(value)]
    idx: ValueId,
    #[inst(value)]
    value: ValueId,
}
impl_inst_write!(InsertValue);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Inst)]
pub struct ExtractValue {
    #[inst(value)]
    dest: ValueId,
    #[inst(value)]
    idx: ValueId,
}
impl_inst_write!(ExtractValue);
