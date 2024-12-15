use super::data;
use super::*;

pub trait Functor: data::Functor {
    fn fmap<A, B, F>(f: F, carrier: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B;

    fn void<A>(carrier: Self::Container<A>) -> Self::Container<()> {
        Self::fmap(|_| (), carrier)
    }

    fn constant<A, B>(a: A, carrier: Self::Container<B>) -> Self::Container<A> {
        Self::fmap(move |_| a, carrier)
    }
}

impl Functor for OptionFunctor {
    fn fmap<A, B, F>(f: F, carrier: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        carrier.map(f)
    }
}

impl<E> Functor for ResultFunctor<E> {
    fn fmap<A, B, F>(f: F, carrier: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        carrier.map(f)
    }
}

pub trait Applicative: data::Applicative {
    fn pure<A>(a: A) -> Self::Container<A>;

    fn zip_map<A, B, F>(fs: Self::Container<F>, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        Self::zip_with(|f, a| f(a), fs, fa)
    }

    fn zip_with<A, B, C, F>(
        f: F,
        a: Self::Container<A>,
        b: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C;
}

impl Applicative for OptionFunctor {
    fn pure<A>(a: A) -> Self::Container<A> {
        Some(a)
    }

    fn zip_with<A, B, C, F>(
        f: F,
        a: Self::Container<A>,
        b: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C,
    {
        a.zip(b).map(|(a, b)| f(a, b))
    }
}
