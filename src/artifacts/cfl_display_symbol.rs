use std::fmt::{Display, Formatter};

use crate::core::CFLRuleIndex;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CFLDisplaySymbol {
    Epsilon,
    Pop(CFLRuleIndex),
    Push(CFLRuleIndex),
    VPop(CFLRuleIndex),
    VPush(CFLRuleIndex),
}

impl CFLDisplaySymbol {
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Epsilon => "eps",
            Self::Pop(_) => "pp_i",
            Self::Push(_) => "psh_i",
            Self::VPop(_) => "vpp_i",
            Self::VPush(_) => "vpsh_i",
        }
    }

    pub fn rule(&self) -> Option<CFLRuleIndex> {
        match self {
            Self::Epsilon => None,
            Self::Pop(r) => Some(*r),
            Self::Push(r) => Some(*r),
            Self::VPop(r) => Some(*r),
            Self::VPush(r) => Some(*r),
        }
    }
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
            Self::VPop(_) => panic!(),
            Self::VPush(_) => panic!(),
        }
        Ok(())
    }
}
