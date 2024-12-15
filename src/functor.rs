use std::marker::PhantomData;

pub mod control;
pub mod data;

pub enum OptionFunctor {}
pub struct ResultFunctor<E> {
    phantom: PhantomData<E>,
}
pub enum VecFunctor {}
pub enum ZipVecFunctor {}

pub enum IdentityFunctor {}
