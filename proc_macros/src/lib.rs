use syn::{parse_macro_input, DeriveInput};

mod builder;


#[proc_macro_attribute]
pub fn builder(attr: proc_macro::TokenStream ,input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    return builder::builder(attr.into(), input.into()).into();
}

