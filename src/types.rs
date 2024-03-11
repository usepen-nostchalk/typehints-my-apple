use std::{marker::PhantomData};

use crate::args::{Call, A1, A2, A3};

pub struct Type_8<A, B, C, D, E, F, G, H, Args> {
    types: PhantomData<(A, B, C, D, E, F, G, H)>,
    data: Vec<Args>,
}

pub struct Type_7<A, B, C, D, E, F, G, Args> {
    types: PhantomData<(A, B, C, D, E, F, G)>,
    data: Vec<Args>,
}

pub struct Type_6<A, B, C, D, E, F, Args> {
    types: PhantomData<(A, B, C, D, E, F)>,
    data: Vec<Args>,
}

pub struct Type_5<A, B, C, D, E, Args> {
    types: PhantomData<(A, B, C, D, E)>,
    data: Vec<Args>,
}

pub struct Type_4<A, B, C, D, Args> {
    types: PhantomData<(A, B, C, D)>,
    data: Vec<Args>,
}

pub struct Type_3<A, B, C, Return, Args> {
    types: PhantomData<(A, B, C, Return)>,
    data: Vec<Args>,
}

pub struct Type_2<A, B, Return, Args> {
    types: PhantomData<(A, B, Return)>,
    data: Vec<Args>,
}

pub struct Type_1<A, Return, Args> {
    types: PhantomData<(A, Return)>,
    data: Vec<Args>,
}

impl<A, B, C, Return, Args> Type_3<A, B, C, Return, Args>
where Args: A3<A>
{
    pub fn new(types: PhantomData<(A, B, C, Return)>, data: Vec<Args>) -> Self {
        Self { types, data }
    }

    pub fn apply(self, arg: A) -> Type_2<B, C, Return, Args> {
        let mut args = Vec::from(self.data);
        let x = Args::a3(arg);
        args.push(x);
        Type_2 { types: PhantomData::default(), data: args}
    } 
}

impl<A, B, Return, Args> Type_2<A, B, Return, Args>
where Args: A2<A>
{
    pub fn apply(self, arg: A) -> Type_1<B, Return, Args> {
        let mut args = Vec::from(self.data);
        args.push(Args::a2(arg));
        Type_1 { types: PhantomData::default(), data: args}
    }
}

impl<A, Return, Args> Type_1<A, Return, Args> 
where Args: A1<A> + Call<Return>
{
    pub fn apply(self, arg: A) -> Return {
        let mut args = Vec::from(self.data);
        args.push(Args::a1(arg));
        Args::call(args)
    }
}
