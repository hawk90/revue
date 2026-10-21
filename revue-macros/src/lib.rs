//! Proc macros for Revue framework
//!
//! This crate provides derive macros for Revue traits.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the `Store` trait
///
/// Automatically implements the `Store` trait for structs.
///
/// # Requirements
///
/// Your struct must derive `Default`. The macro will implement the `Store` trait
/// using the struct's type name as the store identifier.
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
/// use revue_macros::Store;
///
/// #[derive(Store, Default)]
/// struct CounterStore {
///     count: Signal<i32>,
/// }
///
/// impl CounterStore {
///     fn new() -> Self {
///         Self {
///             count: signal(0),
///         }
///     }
///
///     fn increment(&self) {
///         self.count.update(|c| *c += 1);
///     }
/// }
///
/// // Use the store
/// let counter = use_store::<CounterStore>();
/// counter.increment();
/// ```
///
/// # Generated Implementation
///
/// The macro generates:
/// - `id()` method that returns a fixed ID from the type's full path (`revue::reactive::store::StoreId::from_path`)
/// - `name()` method that returns the struct name
/// - `get_state()` and `get_getters()` methods (basic implementations)
#[proc_macro_derive(Store)]
pub fn derive_store(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // A store type has one id, hashed from its path: a generic type would be
    // many types under one id, and a union has no fields to read state from
    if !input.generics.params.is_empty() {
        return syn::Error::new_spanned(
            &input.generics,
            "`#[derive(Store)]` does not take generic parameters: a store type has one id, and \
             each instance of a generic type would share it",
        )
        .to_compile_error()
        .into();
    }
    if let syn::Data::Union(data) = &input.data {
        return syn::Error::new_spanned(
            data.union_token,
            "`#[derive(Store)]` works on structs and enums, not unions",
        )
        .to_compile_error()
        .into();
    }
    let struct_name = &input.ident;
    let struct_name_string = struct_name.to_string();

    let expanded = quote! {
        // Implement Store trait
        impl ::revue::reactive::store::Store for #struct_name {
            fn id(&self) -> ::revue::reactive::store::StoreId {
                // The type's full path, hashed at compile time: the module
                // path is only known where the derive expands, not here
                // (#1191).
                const ID: ::revue::reactive::store::StoreId =
                    ::revue::reactive::store::StoreId::from_path(::core::concat!(
                        ::core::module_path!(),
                        "::",
                        #struct_name_string
                    ));
                ID
            }

            fn name(&self) -> &str {
                #struct_name_string
            }

            fn get_state(&self) -> ::std::collections::HashMap<String, String> {
                ::std::collections::HashMap::new()
            }

            fn get_getters(&self) -> ::std::collections::HashMap<String, String> {
                ::std::collections::HashMap::new()
            }
        }
    };

    TokenStream::from(expanded)
}
