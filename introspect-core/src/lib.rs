//! A crate containing the core functionality used for `introspect` and supporting
//! crates.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(rustdoc::broken_intra_doc_links)]

mod entity;
pub mod r#enum;
mod member;
pub mod r#struct;

pub use entity::Entity;
pub use member::Member;

pub use r#enum::Enum;
pub use r#struct::Struct;
