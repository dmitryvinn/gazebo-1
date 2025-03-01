/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Derivations for the [Gazebo library](https://docs.rs/gazebo/).
//! Usually you would use these derivations via exports from that library.

#[allow(unused_extern_crates)] // proc_macro is very special
extern crate proc_macro;

mod any_lifetime;
mod clone;
mod coerce;
mod copy;
mod default;
mod dupe;
mod maybe_eq;
mod util;
mod variant;

/// Derive the `Dupe` trait.
#[proc_macro_derive(Dupe)]
pub fn derive_dupe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dupe::derive_dupe(input)
}

/// Derive the `Dupe` trait, but without requiring all type arguments to implement `Dupe`.
#[proc_macro_derive(Dupe_)]
pub fn derive_dupe_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dupe::derive_dupe_(input)
}

/// Derive the [`Clone` trait](Clone), but without requiring all type arguments to implement [`Clone`](Clone).
#[proc_macro_derive(Clone_)]
pub fn derive_clone_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    clone::derive_clone_(input)
}

/// Derive the [`Copy` trait](Copy), but without requiring all type arguments to implement [`Copy`](Copy).
#[proc_macro_derive(Copy_)]
pub fn derive_copy_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    copy::derive_copy_(input)
}

/// Derive the [`Default` trait](Default), but without requiring all type arguments to implement [`Default`](Default).
#[proc_macro_derive(Default_)]
pub fn derive_default_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    default::derive_default_(input)
}

/// Derive the `AnyLifetime` trait. Requires the type has no type arguments, no constant arguments,
/// and at most one lifetime argument.
#[proc_macro_derive(AnyLifetime)]
pub fn derive_any_lifetime(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // We say we derive `AnyLifetime`, but we actually derive `ProvidesStaticType`.
    any_lifetime::derive_provides_static_type(input)
}

/// Derive the `MaybeEq` trait.
#[proc_macro_derive(MaybeEq)]
pub fn derive_maybe_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    maybe_eq::derive_maybe_eq(input, true)
}

/// Derive the `MaybeEq` trait, but so that it is never comparable.
#[proc_macro_derive(MaybeEq_Never)]
pub fn derive_not_maybe_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    maybe_eq::derive_maybe_eq(input, false)
}

/// Derive the `VariantName` trait.
#[proc_macro_derive(VariantName)]
pub fn derive_variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    variant::derive_variant_names(input)
}

// Derive the `Variants` functions.
#[proc_macro_derive(UnpackVariants)]
pub fn derive_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    variant::derive_unpack_variants(input)
}

// Derive the `Coerce` trait.
#[proc_macro_derive(Coerce)]
pub fn derive_coerce(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    coerce::derive_coerce(input)
}
