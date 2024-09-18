use macros::Inst;

use crate::{value_::ValueId, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Inst)]
pub struct Sext {
    #[inst(value)]
    from: ValueId,
    ty: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Inst)]
pub struct Zext {
    #[inst(value)]
    from: ValueId,
    ty: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Inst)]
pub struct Trunc {
    #[inst(value)]
    from: ValueId,
    ty: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Inst)]
pub struct Bitcast {
    #[inst(value)]
    from: ValueId,
    ty: Type,
}
