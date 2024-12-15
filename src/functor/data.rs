use super::control;
use super::*;

pub trait Functor {
    type Container<A>;
    fn dmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B;

    fn void<A>(fa: Self::Container<A>) -> Self::Container<()> {
        Self::dmap(|_| (), fa)
    }

    fn constant<A: Clone, B>(a: A, fb: Self::Container<B>) -> Self::Container<A> {
        Self::dmap(|_| a.clone(), fb)
    }
}

impl Functor for IdentityFunctor {
    type Container<A> = A;

    fn dmap<A, B, F>(mut f: F, a: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        f(a)
    }
}

impl Functor for OptionFunctor {
    type Container<A> = Option<A>;

    fn dmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl<E> Functor for ResultFunctor<E> {
    type Container<A> = Result<A, E>;

    fn dmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl Functor for VecFunctor {
    type Container<A> = Vec<A>;

    fn dmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

impl Functor for ZipVecFunctor {
    type Container<A> = Vec<A>;

    fn dmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

pub trait Applicative: Functor {
    fn dpure<A: Clone>(a: A) -> Self::Container<A>;

    fn d_zip_map<A, B, F>(fs: Self::Container<F>, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        Self::d_zip_with(|mut f, a| f(a), fs, fa)
    }

    fn d_zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C;
}

impl Applicative for IdentityFunctor {
    fn dpure<A: Clone>(a: A) -> Self::Container<A> {
        a
    }
    fn d_zip_with<A, B, C, F>(
        mut f: F,
        a: Self::Container<A>,
        b: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        f(a, b)
    }
}

impl Applicative for OptionFunctor {
    fn dpure<A: Clone>(a: A) -> Self::Container<A> {
        Some(a)
    }
    fn d_zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        fa.zip(fb).map(|(a, b)| f(a, b))
    }
}

impl<E> Applicative for ResultFunctor<E> {
    fn dpure<A: Clone>(a: A) -> Self::Container<A> {
        Ok(a)
    }
    fn d_zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        fa.and_then(|a| fb.map(|b| f(a, b)))
    }
}

pub trait Traversable: Functor {
    fn traverse<F, A, B, G>(f: G, fa: Self::Container<A>) -> F::Container<Self::Container<B>>
    where
        B: Clone,
        F: control::Applicative,
        G: FnMut(A) -> F::Container<B>;
}

impl Traversable for IdentityFunctor {
    fn traverse<F, A, B, G>(mut f: G, fa: Self::Container<A>) -> F::Container<Self::Container<B>>
    where
        B: Clone,
        F: Applicative,
        G: FnMut(A) -> F::Container<B>,
    {
        f(fa)
    }
}

impl Traversable for OptionFunctor {
    fn traverse<F, A, B, G>(mut f: G, fa: Self::Container<A>) -> F::Container<Self::Container<B>>
    where
        B: Clone,
        F: control::Applicative,
        G: FnMut(A) -> F::Container<B>,
    {
        match fa {
            Some(a) => F::fmap(Some, f(a)),
            None => F::pure(None),
        }
    }
}

impl<E: Clone> Traversable for ResultFunctor<E> {
    fn traverse<F, A, B, G>(mut f: G, fa: Self::Container<A>) -> F::Container<Self::Container<B>>
    where
        B: Clone,
        F: control::Applicative,
        G: FnMut(A) -> F::Container<B>,
    {
        match fa {
            Ok(a) => F::fmap(Ok, f(a)),
            Err(e) => F::pure(Err(e)),
        }
    }
}

impl Traversable for VecFunctor {
    fn traverse<F, A, B, G>(mut f: G, fa: Self::Container<A>) -> F::Container<Self::Container<B>>
    where
        B: Clone,
        F: control::Applicative,
        G: FnMut(A) -> F::Container<B>,
    {
        let mut result = F::pure(Vec::new());
        for a in fa {
            let b = f(a);
            result = F::zip_with(
                |mut v, b| {
                    v.push(b);
                    v
                },
                result,
                b,
            );
        }
        result
    }
}
