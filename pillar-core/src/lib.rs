#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(never_type)]

pub mod layer;
pub mod ready;
pub mod service;
pub mod timeout;

// pub struct UnreadyService<F, S> {
//     inner: S,
//     f: F,
// }

// impl<RS1, RS2, S1, S2, Req1, Req2> Layer<S1, S2, RS2> for RS1
// where
//     S1: Service<Req1>,
//     S2: Service<Req2>,
//     RS1: Ready<S1>,
//     RS2: Ready<S2>,
// {
//     fn layer<F>(self, f: F) -> RS2
//     where
//         F: FnOnce(S1) -> S2,
//     {
//         todo!()
//     }
// }
