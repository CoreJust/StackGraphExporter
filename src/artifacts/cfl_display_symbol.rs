use std::fmt::{Display, Formatter};

pub enum CFLDisplaySymbol {
    Epsilon,
    Push(usize),
    Pop(usize),
}

impl Display for CFLDisplaySymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Epsilon => {
                write!(f, "eps")?;
            }
            Self::Push(index) => {
                write!(f, "psh{index}")?;
            }
            Self::Pop(index) => {
                write!(f, "pp{index}")?;
            }
        }
        Ok(())
    }
}
