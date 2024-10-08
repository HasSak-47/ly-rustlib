mod builder;

#[proc_macro_attribute]
pub fn builder(attr: proc_macro::TokenStream ,input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let k : proc_macro::TokenStream = builder::builder(attr.into(), input.into()).unwrap_or_else(syn::Error::into_compile_error).into();
    return k;
}

#[test]
fn test_macro_quote() {
    use quote::quote;
    builder::builder(quote!{builder}, quote!{
    struct TestStruct{
        #[builder(skip)]
        id1 : usize,
        #[builder(init = String::from("test"))]
        data : String,
        #[builder(ty = Option<i32>)]
        id2: usize,
        #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
        string: String,
    }}).unwrap();
}
