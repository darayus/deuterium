
use predicate::{Predicate, ToAbstractPredicate, RcPredicate};
use expression::{ToExpression};

use expression::{RawExpr};
use field::{StringField, OptionalStringField};
use sql::{ToPredicateValue};

#[deriving(Clone)]
pub struct LikePredicate<F, T> {
    pub field: F,
    pub value: T,
    pub case_sensitive: bool
}

pub trait ToLikePredicate<F, T> {
    fn like(&self, val: T) -> RcPredicate;
    fn ilike(&self, val: T) -> RcPredicate;
}

macro_rules! is_methods(
    ($v:ty) => (
        fn like(&self, val: $v) -> RcPredicate {
            LikePredicate {
                field: self.clone(),
                value: val,
                case_sensitive: true
            }.upcast()
        }

        fn ilike(&self, val: $v) -> RcPredicate {
            LikePredicate {
                field: self.clone(),
                value: val,
                case_sensitive: false
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: ToExpression<$v> + ToPredicateValue + Clone> Predicate for LikePredicate<$field, T> { }
        impl<T: ToExpression<$v> + ToPredicateValue + Clone> ToLikePredicate<$field, T> for $field {
            is_methods!(T) 
        }
    )
)

impl_for!(StringField, String)
impl_for!(OptionalStringField, String)

impl_for!(RawExpr, String)