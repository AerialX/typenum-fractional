use typenum::consts::{B0, B1};

pub type If<Cond, If, Else> = <Cond as Conditional<If, Else>>::Output;

pub trait Conditional<If, Else> {
    type Output;
}

impl<If, Else> Conditional<If, Else> for B0 {
    type Output = Else;
}

impl<If, Else> Conditional<If, Else> for B1 {
    type Output = If;
}
