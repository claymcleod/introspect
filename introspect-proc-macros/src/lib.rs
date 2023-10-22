//! Procedural macros for `introspect`.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(rustdoc::broken_intra_doc_links)]

use core::panic;

use introspect_core::r#enum::Variant;
use introspect_core::r#struct::Field;
use introspect_core::Entity;
use introspect_core::Enum;
use introspect_core::Struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

/// The primary `derive` procedural macro that implements the introspection traits.
#[proc_macro_derive(Introspect)]
pub fn introspect(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(stream as Item);

    match item {
        Item::Enum(enum_) => parse_item_enum(enum_),
        Item::Struct(struct_) => parse_item_struct(struct_),
        _ => {
            quote! {
                compile_error!("Introspect can only be derived for `enum`s and `struct`s")
            }
        }
    }
    .into()
}

fn parse_item_enum(item: syn::ItemEnum) -> TokenStream {
    let ident = &item.ident;

    let enum_ = match Enum::try_from(&item) {
        Ok(enum_) => Entity::Enum(enum_),
        // SAFETY: this panic is okay because it happens during the compilation
        // process. As such, the Rust compiler will complain with this error instead
        // of happening at program runtime.
        Err(err) => panic!("error: {err}"),
    };

    let variants = item
        .variants
        .iter()
        .map(|field| match Variant::try_from(field) {
            Ok(variant) => variant,
            // SAFETY: this panic is okay because it happens during the compilation
            // process. As such, the Rust compiler will complain with this error instead
            // of happening at program runtime.
            Err(err) => panic!("error: {err}"),
        })
        .map(introspect_core::Member::Variant)
        .collect::<Vec<_>>();

    quote! {
        #[automatically_derived]
        impl ::introspect::IntrospectedEntity for #ident {

            fn introspected_entity() -> ::introspect::Entity {
                #enum_
            }
        }

        #[automatically_derived]
        impl ::introspect::IntrospectedMembers for #ident {

            fn introspected_members() -> Vec<::introspect::Member> {
                vec![
                    #(#variants),*
                ]
            }
        }

        #[automatically_derived]
        impl ::introspect::Introspected for #ident {}
    }
}

fn parse_item_struct(item: syn::ItemStruct) -> TokenStream {
    let ident = &item.ident;

    let struct_ = match Struct::try_from(&item) {
        Ok(struct_) => Entity::Struct(struct_),
        // SAFETY: this panic is okay because it happens during the compilation
        // process. As such, the Rust compiler will complain with this error instead
        // of happening at program runtime.
        Err(err) => panic!("error: {err}"),
    };

    let fields = item
        .fields
        .iter()
        .map(|field| match Field::try_from(field) {
            Ok(field) => field,
            // SAFETY: this panic is okay because it happens during the compilation
            // process. As such, the Rust compiler will complain with this error instead
            // of happening at program runtime.
            Err(err) => panic!("error: {err}"),
        })
        .map(introspect_core::Member::Field)
        .collect::<Vec<_>>();

    quote! {
        #[automatically_derived]
        impl ::introspect::IntrospectedEntity for #ident {

            fn introspected_entity() -> ::introspect::Entity {
                #struct_
            }
        }

        #[automatically_derived]
        impl ::introspect::IntrospectedMembers for #ident {

            fn introspected_members() -> Vec<::introspect::Member> {
                vec![
                    #(#fields),*
                ]
            }
        }

        #[automatically_derived]
        impl ::introspect::Introspected for #ident {}
    }
}
