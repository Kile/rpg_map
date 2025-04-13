use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

#[proc_macro_attribute]
pub fn stubgen(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    // Wrap the item with #[cfg_attr(feature = "stubgen", ...)]
    let output = match input {
        Item::Struct(s) => {
            quote! {
                #[cfg_attr(feature = "stubgen", pyo3_stub_gen::derive::gen_stub_pyclass)]
                #s
            }
        }
        Item::Impl(i) => {
            quote! {
                #[cfg_attr(feature = "stubgen", pyo3_stub_gen::derive::gen_stub_pymethods)]
                #i
            }
        }
        Item::Fn(f) => {
            quote! {
                #[cfg_attr(feature = "stubgen", pyo3_stub_gen::derive::gen_stub_pyfunction)]
                #f
            }
        }
        Item::Enum(e) => {
            quote! {
                #[cfg_attr(feature = "stubgen", pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
                #e
            }
        }
        _ => {
            quote! {
                #input
            }
        }
    };

    output.into()
}