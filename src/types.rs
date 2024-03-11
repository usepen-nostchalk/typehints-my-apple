use std::marker::PhantomData;

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

pub struct Type_3<A, B, C, Args> {
    types: PhantomData<(A, B, C, )>,
    data: Vec<Args>,
}

pub struct Type_2<A, B, Args> {
    types: PhantomData<(A, B, )>,
    args: Vec<Args>,
}

pub struct Type_1<A, Args> {
    types: PhantomData<(A, )>,
    args: Vec<Args>,
}

