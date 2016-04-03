extern crate algebra;

use algebra::ops::Mul;

pub trait Magma
{
    fn op(&self, rhs:&Self) -> Self;
}

impl<A> Magma for MgM<A>
    where
        A: Magma,
{
    #[inline]
    fn op(&self, rhs: &MgM<A>) -> MgM<A> {
        let &MgM(ref lhs) = self;
        let &MgM(ref rhs) = rhs;
        MgM(lhs.op(rhs))
    }
}


impl<A> Mul<MgM<A>> for MgM<A>
    where
        A: Magma,
{
    #[inline]
	type Output = MgM<A>;
    fn mul(self, rhs: MgM<A>) -> MgM<A> {
        self.op(&rhs)
    }
}


pub trait MagmaMultiplicative
    : Magma
    + Mul<Self> where Self: Sized {}

pub trait Semigroup: Magma
{
    #[inline]
    fn app(&self, rhs:&Self) -> Self where Self: Sized {
        self.op(rhs)
    }
}

pub trait Monoid: Semigroup
{
    fn nil() -> Self;
}


#[derive(Clone)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
pub struct MgM<A>(pub A);