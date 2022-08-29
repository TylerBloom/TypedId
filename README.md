[![Crates.io](https://img.shields.io/crates/v/typed_id.svg)](https://crates.io/crates/typed_id)
[![Documentation](https://docs.rs/typed_id/badge.svg)](https://docs.rs/typed_id/)
![GitHub Workflows](https://github.com/TylerBloom/TypedId/actions/workflows/ci.yml/badge.svg)
[![Coverage Status](https://codecov.io/gh/TylerBloom/TypedId/branch/main/graph/badge.svg)](https://codecov.io/gh/TylerBloom/TypedId)
![Maintenance](https://img.shields.io/badge/Maintenance-Actively%20Developed-brightgreen.svg)

## About
`TypedId` introduces a single type, aptly named `TypedId`. This is a generic
wrapper any type, often types that you would use as an identifier. However,
each instance of a `TypedId` has a free-generic parameter that you can use to
associate that ID type to a given struct (or collection of structs).

This allows you to have an added layer of type-checked safety at compile time
that disappears at run time! You can have all your types use the same
underlying structure for thier identifiers while never have to worry about
swapping them around.

## How it Works
`TypedId`s are very strange forward.
The easiest way to use them is why declaring your own `type`s that use `TypedId`.
For example,
```rust
use typed_id::TypedId;

pub stuct Customer {
    id: CustomerId,
    /* Likely other fields */
}

pub type CustomerId = TypedId<u32, Customer>;
```

It's that simple! If you have other types that you need ids for, simply add
another `type` and `TypedId` will handle all the boilerplate for you!

Note, that `TypedId` has an optional dependency on `serde`. When enabled, this
adds an opinionated (de)serization implementation. This implementation
(de)serizalizes a `TypedId` as its underlying type.

## Why use 
Rust has a very powerful type system with many amazing properties. This
leverages that system to prevent simple typos, such as passing in the wrong
type to a function.

## Contribution
If you want to contribute to or improve upon this library, please do so.
Fork this project or submit an issue or a pull request for a
feature/fix/change/etc. All that I ask is for derived/iterative
libraries to be open and free to use and ideally with the same license
(LGPL v2.1). Any other application or library that uses this library can
use any license.

