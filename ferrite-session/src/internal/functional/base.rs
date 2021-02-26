use super::type_app::*;

// An implementation of Functor, Applicative, and Monad
// without resorting to HKT or GAT.

pub trait Functor: TypeAppGeneric
{
  fn fmap<A, B>(
    fa : Applied<Self, A>,
    mapper : impl Fn(A) -> B,
  ) -> Applied<Self, B>
  where
    A : Send + 'static,
    B : Send + 'static;
}

pub trait Applicative: Functor
{
  fn apply<A, B, Func>(
    fab : Applied<Self, Func>,
    fa : Applied<Self, A>,
  ) -> Applied<Self, B>
  where
    Func : Fn(A) -> B,
    A : Send + 'static,
    B : Send + 'static;
}

pub trait Monad: Applicative
{
  fn bind<A, B>(
    fa : Applied<Self, A>,
    cont : impl Fn(A) -> Applied<Self, B>,
  ) -> Applied<Self, B>
  where
    A : Send + 'static,
    B : Send + 'static;
}

// NaturalTransformation f1 f2 = forall x. f1 x -> f2 x
pub trait NaturalTransformation<F1, F2>
where
  F1 : TyCon,
  F2 : TyCon,
{
  fn lift<A>(
    &self,
    fa : Applied<F1, A>,
  ) -> Applied<F2, A>
  where
    A : Send + 'static;
}