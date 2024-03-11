pub trait A8<A> { fn a8(arg8: A) -> Self; }

pub trait A7<A> { fn a7(arg7: A) -> Self; }

pub trait A6<A> { fn a6(arg6: A) -> Self; }

pub trait A5<A> { fn a5(arg5: A) -> Self; }

pub trait A4<A> { fn a4(arg4: A) -> Self; }

pub trait A3<A> { fn a3(arg3: A) -> Self; }

pub trait A2<A> { fn a2(arg2: A) -> Self; }

pub trait A1<A> { fn a1(arg1: A) -> Self; }

pub trait Call<Return> where Self: Sized {
    fn call(args: Vec<Self>) -> Return;
}

// I will leave this in here, for reference and later used.
// pub enum Arg_3<A, B, C> {
//     ARG_3(A),
//     ARG_2(B),
//     ARG_1(C),
//
// }
//
// impl<A, B, C> A3<A> for Arg_3<A, B, C> {
//     fn a3(arg3: A) -> Self { Self::ARG_3(arg3) }
// }
//
// impl<A, B, C> A2<B> for Arg_3<A, B, C> {
//     fn a2(arg2: B) -> Self { Self::ARG_2(arg2) }
// }  
//
// impl<A, B, C> A1<C> for Arg_3<A, B, C> {
//     fn a1(arg1: C) -> Self { Self::ARG_1(arg1) }
// }  
//
// impl<A, B, C> Arg_3<A, B, C> {
//     pub fn generate<Return>() -> Type_3<A, B, C, Return, Self> {
//         Type_3::new(PhantomData::default(), Vec::new())
//     }
// }
