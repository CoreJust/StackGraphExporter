use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CFLDisplaySymbol {
    Epsilon,
    Pop(usize),
    Push(usize),
}

impl Display for CFLDisplaySymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Epsilon => {
                write!(f, "eps")?;
            }
            Self::Pop(index) => {
                write!(f, "pp{index}")?;
            }
            Self::Push(index) => {
                write!(f, "psh{index}")?;
            }
        }
        Ok(())
    }
}
