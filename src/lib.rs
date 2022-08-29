//! This crate provides tools to provide type safety for types that you use as identifier types.
//! In general, the identifier fields between structs aren't interchangable, even if they are the
//! same type.
//! 
//! Enter `TypedId`, a container struct that is generic over your ID type and the type that ID
//! belongs to. `TypedId` implements many useful traits to make using `TypeId` identical to using
//! the underlying ID type, but with a layer of type safety to prevent mistakes that can occur when
//! work with multiple ID types. This layer only exists at compile time, so there is no added bloat
//! or performance hit at runtime. Let's see that `TypedId` can do.
//! ```rust
//! use typed_id::TypedId;
//! // The ID type for our Customer struct
//! pub type CustomerId = TypedId<u32, Customer>;
//! // The ID type for our Order struct
//! pub type OrderId = TypedId<u32, Order>;
//! 
//! pub struct Customer {
//!     id: CustomerId,
//!     orders: Vec<OrderId>,
//!     /* Likely more fields */
//! }
//! 
//! pub struct Order {
//!     id: OrderId,
//!     /* Likely more fields */
//! }
//!
//! impl Customer {
//!     pub fn has_order(&self, o_id: OrderId) -> bool {
//!         self.orders.iter().find(|&o| *o == o_id).is_some()
//!     }
//! }
//! 
//! let id = 42;
//! 
//! // Convert the id to a typed id
//! let mut customer = Customer { id: id.into(), orders: Vec::new() };
//! let order = Order { id: id.into() };
//! 
//! customer.orders.push(order.id);
//! 
//! // We can cast typed ids back to Uuid's if needed
//! assert_eq!(id, *customer.id);
//! assert_eq!(id, *order.id);
//! 
//! // This will *not* compile
//! // We *can't* directly compare typed ids, they are different types
//! // assert_eq!(id, customer.id);
//! // assert_eq!(id, order.id);
//! // assert_eq!(order.id, customer.id);
//! 
//! // Nor can we mistake a Uuid or a different typed id for an OrderId
//! // assert!(customer.has_order(id));
//! 
//! // Instead, we must have an OrderId or explicitly convert an id to an OrderId
//! assert!(customer.has_order(order.id));
//! assert!(customer.has_order(id.into()));
//! assert!(customer.has_order(customer.id.convert()));
//! ```

#![deny(
    dead_code,
    irrefutable_let_patterns,
    missing_docs,
    unused_variables,
    unused_imports,
    unused_import_braces,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub,
)]
#![warn(rust_2018_idioms)]

use std::{fmt, hash::Hash, marker::PhantomData, ops::Deref};

#[cfg(feature = "serde")]
mod serde;

/// A generic type-checked wrapper around a generic identifier type
pub struct TypedId<I, T>(pub I, PhantomData<T>);

impl<I, T> TypedId<I, T> {
    /// Creates a new typed id with an underlying ID type of `I`
    pub fn new(id: I) -> Self {
        Self(id, PhantomData)
    }

    /// The method explictly converts between typed ids.
    /// ```rust
    /// # struct A;
    /// # struct B;
    /// use typed_id::TypedId;
    /// let a_id: TypedId<u32, A> = 42.into();
    /// let b_id: TypedId<u32, B> = a_id.convert();
    /// ```
    /// 
    /// Note, `From` can not be implemented here. We can't specify that two generic types, `A` and
    /// `B`, are distinct. If we try, this fails to compile.
    /// ```compile_fail
    /// impl<I, A, B> From<TypedId<I, A>> for TypedId<I, B> {
    ///   fn from(other: TypedId<I, A>) -> TypedId<I, B> {
    ///     other.0.into()
    ///   }
    /// }
    /// ```
    pub fn convert<B: From<I>>(self) -> B {
        B::from(self.0)
    }
}

impl<I: Default, T> Default for TypedId<I, T> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<I: fmt::Debug, T> fmt::Debug for TypedId<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypedId").field(&self.0).finish()
    }
}

impl<I: fmt::Display, T> fmt::Display for TypedId<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<I: Clone, T> Clone for TypedId<I, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<I: Copy, T> Copy for TypedId<I, T> {}

impl<I: Hash, T> Hash for TypedId<I, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<I: PartialEq, T> PartialEq for TypedId<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<I: Eq, T> Eq for TypedId<I, T> {}

impl<I: PartialOrd, T> PartialOrd for TypedId<I, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<I: Ord, T> Ord for TypedId<I, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<I, T> Deref for TypedId<I, T> {
    type Target = I;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I, T> From<I> for TypedId<I, T> {
    fn from(other: I) -> TypedId<I, T> {
        TypedId(other, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::TypedId;
    
    type CustomerId = TypedId<u32, Customer>;
    type OrderId = TypedId<u32, Order>;

    struct Customer {
        id: CustomerId,
        orders: Vec<OrderId>,
    }

    struct Order {
        id: OrderId,
    }

    impl Customer {
        fn has_order(&self, o_id: OrderId) -> bool {
            self.orders.iter().find(|&o| *o == o_id).is_some()
        }
    }

    #[test]
    fn basic_convertion() {
        let id = 42;

        let mut customer = Customer { id: id.into(), orders: Vec::new() };
        let order = Order { id: id.into() };

        customer.orders.push(order.id);

        assert_eq!(id, *customer.id);
        assert_eq!(id, *order.id);

        assert!(customer.has_order(order.id));
        assert!(customer.has_order(id.into()));
        assert!(customer.has_order(customer.id.convert()));
    }
    
    #[test]
    fn basic_strings() {
        let id = 42;
        let t_id: CustomerId = id.into();
        assert_eq!(id.to_string(), t_id.to_string());
        assert_eq!(format!("{t_id:?}"), String::from("TypedId(42)"));
    }
}
