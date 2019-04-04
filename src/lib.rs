#![feature(specialization)]
pub use impl_inheritance_macros::*;

//todo: replace with ! when stable and != Infallible
pub type Placeholder = std::convert::Infallible;

pub trait SuperBorrow<T> 
where T: ? Sized
{
    fn super_ref(&self) -> & T;
    fn super_ref_mut(& mut self) -> & mut T;
}

pub trait IsSuperBorrowableTo<T>
where T : ?Sized {
    fn get_part(x : &T) -> &Self;
    fn get_part_mut(x :& mut T) -> & mut Self;
}

impl <T,X> IsSuperBorrowableTo<T> for X 
where T : SuperBorrow<X> {
    fn get_part(x : &T) -> &X {
        x.super_ref()
    }
    fn get_part_mut(x :& mut T) -> & mut X
    {
        x.super_ref_mut()
    }
}

pub trait SuperType {
    type SupType;
}

impl SuperType for Placeholder {
    type SupType = Placeholder;
}

expand_constraits_def!();

