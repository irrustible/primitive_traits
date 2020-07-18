use core::hash::Hash;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign,
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign,
    Shl, ShlAssign, Shr, ShrAssign,
    Neg, Not,
};

pub trait Float
    : Copy + Default + Sized
    + Add + AddAssign + Div + DivAssign + Mul + MulAssign
    + Rem + RemAssign + Sub + SubAssign
{
    const WIDTH: usize;
}

pub trait Integer
    : Copy + Default + Eq + Hash + Ord + Not + Sized
    + Add<Self> + AddAssign<Self> + Div<Self> + DivAssign<Self>
    + Mul<Self> + MulAssign<Self> + Sub<Self> + SubAssign<Self>
    + Rem<Self> + RemAssign<Self> 
    + BitAnd<Self> + BitAndAssign<Self> + BitOr<Self> + BitOrAssign<Self>
    + BitXor<Self> + BitXorAssign<Self>
    + Shl<Self> + ShlAssign<Self> + Shr<Self> + ShrAssign<Self>
{
    const MIN: Self;
    const MAX: Self;
    const WIDTH: usize;
}

pub trait Signed : Neg {}

pub trait Unsigned {}

pub trait AddSign : Integer + Unsigned {
    type Signed : Integer + Signed;
}

pub trait DropSign : Integer + Signed {
    type Unsigned : Integer + Unsigned;
}

pub trait ArithmeticShr : Integer + Signed {}

pub trait LogicalShr : Integer + Unsigned {}

impl<T: Integer + Signed> ArithmeticShr for T {}

impl<T: Integer + Unsigned> LogicalShr for T {}

impl Signed for f32 {}
impl Signed for f64 {}

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}
impl Signed for isize {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}

macro_rules! impl_float {
    ($type:ty = $size:expr) => {
        impl Float for $type {
            const WIDTH: usize = $size;
        }
    }
}

macro_rules! impl_integer {
    ($type:ty = $size:expr) => {
        impl Integer for $type {
            const MAX: $type = <$type>::MAX;
            const MIN: $type = <$type>::MIN;
            const WIDTH: usize = $size;
        }
    }
}

macro_rules! impl_add_sign {
    ($type:ty = $signed:ty) => {
        impl AddSign for $type {
            type Signed = $signed;
        }
    }
}
macro_rules! impl_drop_sign {
    ($type:ty = $unsigned:ty) => {
        impl DropSign for $type {
            type Unsigned = $unsigned;
        }
    }
}

impl_float!(f32 = 32);
impl_float!(f64 = 64);
impl_integer!(i8 = 8);
impl_integer!(i16 = 16);
impl_integer!(i32 = 32);
impl_integer!(i64 = 64);
impl_integer!(i128 = 128);
 #[cfg(target_pointer_width="32")]
impl_integer!(isize = 32);
 #[cfg(target_pointer_width="64")]
impl_integer!(isize = 64);

impl_integer!(u8 = 8);
impl_integer!(u16 = 16);
impl_integer!(u32 = 32);
impl_integer!(u64 = 32);
impl_integer!(u128 = 128);
#[cfg(target_pointer_width="32")]
impl_integer!(usize = 32);
#[cfg(target_pointer_width="64")]
impl_integer!(usize = 64);

impl_add_sign!(u8 = i8);
impl_add_sign!(u16 = i16);
impl_add_sign!(u32 = i32);
impl_add_sign!(u64 = i64);
impl_add_sign!(u128 = i128);
impl_add_sign!(usize = isize);

impl_drop_sign!(i8 = u8);
impl_drop_sign!(i16 = u16);
impl_drop_sign!(i32 = u32);
impl_drop_sign!(i64 = u64);
impl_drop_sign!(i128 = u128);
impl_drop_sign!(isize = usize);
