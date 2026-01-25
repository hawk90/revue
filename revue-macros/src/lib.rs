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
/// - `id()` method that returns a unique ID based on the type name
/// - `name()` method that returns the struct name
/// - `get_state()` and `get_getters()` methods (basic implementations)
#[proc_macro_derive(Store)]
pub fn derive_store(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let struct_name_string = struct_name.to_string();

    // Use a simple deterministic ID based on string length and first chars
    // This is just a placeholder - a real implementation would use a proper hash
    let name_bytes = struct_name_string.as_bytes();
    let mut id: u64 = 0;
    for (i, &byte) in name_bytes.iter().take(8).enumerate() {
        id |= (byte as u64) << (i * 8);
    }

    let expanded = quote! {
        // Implement Store trait
        impl ::revue::reactive::store::Store for #struct_name {
            fn id(&self) -> ::revue::reactive::store::StoreId {
                // Use a compile-time constant ID based on type name
                ::revue::reactive::store::StoreId(#id)
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
