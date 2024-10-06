

mod builder;


#[proc_macro_attribute]
pub fn builder(attr: proc_macro::TokenStream ,input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let k : proc_macro::TokenStream = builder::builder(attr.into(), input.into()).unwrap_or_else(syn::Error::into_compile_error).into();
    println!("{}", k.to_string());

    return k;
}

