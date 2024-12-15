use super::data;
use super::*;

pub trait Functor: data::Functor {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B;

    fn void<A>(fa: Self::Container<A>) -> Self::Container<()> {
        Self::fmap(|_| (), fa)
    }

    fn constant<A, B>(a: A, fb: Self::Container<B>) -> Self::Container<A> {
        Self::fmap(move |_| a, fb)
    }
}

impl Functor for IdentityFunctor {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        f(fa)
    }
}

impl Functor for OptionFunctor {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.map(f)
    }
}

impl<E> Functor for ResultFunctor<E> {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.map(f)
    }
}

pub trait Applicative: data::Applicative + Functor {
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

impl Applicative for IdentityFunctor {
    fn pure<A>(a: A) -> Self::Container<A> {
        a
    }

    fn zip_with<A, B, C, F>(
        f: F,
        a: Self::Container<A>,
        b: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C,
    {
        f(a, b)
    }
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

impl<E> Applicative for ResultFunctor<E> {
    fn pure<A>(a: A) -> Self::Container<A> {
        Ok(a)
    }

    fn zip_with<A, B, C, F>(
        f: F,
        a: Self::Container<A>,
        b: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C,
    {
        a.and_then(|a| b.map(|b| f(a, b)))
    }
}

pub trait Monad: Applicative {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>;
}

impl Monad for IdentityFunctor {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        f(fa)
    }
}

impl Monad for OptionFunctor {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}

impl<E> Monad for ResultFunctor<E> {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}
