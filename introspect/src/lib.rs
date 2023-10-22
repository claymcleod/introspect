//! Introspection for Rust `struct`s and `enum`s ("entities") and their respective
//! fields and variants ("members").
//!
//! **Note:** currently, only the identifier name and the optional documentation for
//! each supported entity and member are implemented. That was all that was needed at
//! the time the crate was developed. However, we will implement requests and/or accept
//! pull requests that add additional introspection—please just leave an issue on the
//! [issues page](https://github.com/claymcleod/introspect/issues)!
//!
//! ## Getting Started
//!
//! You can add `introspect` as a dependency via the Github repository.
//!
//! ```bash
//! cargo add --git https://github.com/claymcleod/introspect.git introspect
//! ```
//!
//! ## Examples
//!
//! You can then use the [`Introspected`] trait and related traits to pull out the
//! information you wish. Typically, you will do with the [`Introspect`] derive
//! macro like so.
//!
//! ```rust
//! use introspect::Entity;
//! use introspect::Introspect;
//! use introspect::IntrospectedEntity;
//! use introspect::IntrospectedMembers;
//! use introspect::Member;
//!
//! /// This is the documentation for the [`Example`] enum.
//! ///
//! /// We can add more text down here.
//! #[allow(dead_code)]
//! #[derive(Introspect)]
//! enum Example {
//!     /// The first variant.
//!     One,
//!
//!     /// The second variant.
//!     ///
//!     /// And some more text.
//!     Two,
//! }
//!
//! // Access to the top-level entity characteristics.
//! match Example::introspected_entity() {
//!     Entity::Enum(entity) => {
//!         dbg!(entity.identifier());
//!         dbg!(entity.documentation());
//!     }
//!     _ => unreachable!(),
//! }
//!
//! // Access to the members of the entity.
//! for member in Example::introspected_members() {
//!     match member {
//!         Member::Variant(member) => {
//!             dbg!(member.identifier());
//!             dbg!(member.documentation());
//!         }
//!         _ => unreachable!(),
//!     }
//! }
//! ```
//!
//! You can also take a look at the
//! [examples](https://github.com/claymcleod/introspect/tree/main/introspect/examples) to
//! get a sense of the various ways you can use the crate.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod r#enum;
pub mod r#struct;

#[cfg(feature = "derive")]
pub use introspect_proc_macros::Introspect;

pub use introspect_core::Entity;
pub use introspect_core::Enum;
pub use introspect_core::Member;
pub use introspect_core::Struct;

/// A trait that provides introspection of a Rust entity.
pub trait IntrospectedEntity {
    /// Gets the introspected entity.
    fn introspected_entity() -> Entity;
}

/// A trait that provides introspection of a Rust entity's members.
pub trait IntrospectedMembers {
    /// Gets the introspected entity's members.
    fn introspected_members() -> Vec<Member>;
}

/// A trait encompassing all introspection supported by the crate.
pub trait Introspected: IntrospectedEntity + IntrospectedMembers {}
