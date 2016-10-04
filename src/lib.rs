#![doc(html_logo_url = "",
       html_favicon_url = "",
       html_root_url = "")]
//! This crate contains utilities to work with an api server & client.
//! These utiltilites are used by the data type objects (dto) server, and client.
//! Currently it only has an address representation, but it will have more as i mprove it.
//! Each item will be explained in its own documentation so it can be easily used by third
//! parties.
//!
//! Using it is as simple as including this in the crate:
//!
//! ```
//! extern crate rest_api_data_utils;
//! ```
#![forbid(missing_docs, warnings)]
#![deny(deprecated, improper_ctypes, non_shorthand_field_patterns, overflowing_literals,
    plugin_as_library, private_no_mangle_fns, private_no_mangle_statics, stable_features,
    unconditional_recursion, unknown_lints, unused, unused_allocation, unused_attributes,
    unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused, unused_extern_crates, unused_import_braces,
    unused_qualifications, unused_results, variant_size_differences)]

extern crate rustc_serialize;


pub mod location;
pub use location::Address;
