use std::marker::PhantomData;

use crate::args::{Call, A1, A2, A3, A4, A5, A6, A7, A8};

pub struct Type8<A, B, C, D, E, F, G, H, Return, Args> {
    types: PhantomData<(A, B, C, D, E, F, G, H, Return, )>,
    data: Vec<Args>,
}

pub struct Type7<A, B, C, D, E, F, G, Return, Args> {
    types: PhantomData<(A, B, C, D, E, F, G, Return, )>,
    data: Vec<Args>,
}

pub struct Type6<A, B, C, D, E, F, Return, Args> {
    types: PhantomData<(A, B, C, D, E, F, Return, )>,
    data: Vec<Args>,
}

pub struct Type5<A, B, C, D, E, Return, Args> {
    types: PhantomData<(A, B, C, D, E, Return, )>,
    data: Vec<Args>,
}

pub struct Type4<A, B, C, D, Return, Args> {
    types: PhantomData<(A, B, C, D, Return, )>,
    data: Vec<Args>,
}

pub struct Type3<A, B, C, Return, Args> {
    types: PhantomData<(A, B, C, Return, )>,
    data: Vec<Args>,
}

pub struct Type2<A, B, Return, Args> {
    types: PhantomData<(A, B, Return,)>,
    data: Vec<Args>,
}

pub struct Type1<A, Return, Args> {
    types: PhantomData<(A, Return, )>,
    data: Vec<Args>,
}

impl<A, B, C, D, E, F, G, H, Return, Args> Type8<A, B, C, D, E, F, G, H, Return, Args>
where Args: A8<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type7<B, C, D, E, F, G, H, Return, Args> {
        let mut data = self.data;
        data.push(Args::a8(arg));
        Type7 { types: PhantomData::default(), data }
    }
}

impl<A, B, C, D, E, F, G, Return, Args> Type7<A, B, C, D, E, F, G, Return, Args>
where Args: A7<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type6<B, C, D, E, F, G, Return, Args> {
        let mut data = self.data;
        data.push(Args::a7(arg));
        Type6 { types: PhantomData::default(), data }
    } 
}

impl<A, B, C, D, E, F, Return, Args> Type6<A, B, C, D, E, F, Return, Args>
where Args: A6<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type5<B, C, D, E, F, Return, Args> {
        let mut data = self.data;
        data.push(Args::a6(arg));
        Type5 { types: PhantomData::default(), data }
    }
}

impl<A, B, C, D, E, Return, Args> Type5<A, B, C, D, E, Return, Args>
where Args: A5<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type4<B, C, D, E, Return, Args> {
        let mut data = self.data;
        data.push(Args::a5(arg));
        Type4 { types: PhantomData::default(), data }
    }
}

impl<A, B, C, D, Return, Args> Type4<A, B, C, D, Return, Args> 
where Args: A4<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type3<B, C, D, Return, Args> {
        let mut data = self.data;
        data.push(Args::a4(arg));
        Type3 { types: PhantomData::default(), data }
    }
}

impl<A, B, C, Return, Args> Type3<A, B, C, Return, Args>
where Args: A3<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type2<B, C, Return, Args> {
        let mut data = self.data;
        data.push(Args::a3(arg));
        Type2 { types: PhantomData::default(), data }
    } 
}

impl<A, B, Return, Args> Type2<A, B, Return, Args>
where Args: A2<A>
{
    pub fn new() -> Self { Self { types: PhantomData::default(), data: Vec::new()} }

    pub fn apply(self, arg: A) -> Type1<B, Return, Args> {
        let mut data = Vec::from(self.data);
        data.push(Args::a2(arg));
        Type1 { types: PhantomData::default(), data }
    }
}

impl<A, Return, Args> Type1<A, Return, Args> 
where Args: A1<A> + Call<Return>
{
    pub fn apply(self, arg: A) -> Return {
        let mut args = Vec::from(self.data);
        args.push(Args::a1(arg));
        Args::call(args)
    }
}
