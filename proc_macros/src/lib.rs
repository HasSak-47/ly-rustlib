mod builder;

#[proc_macro_attribute]
pub fn builder(attr: proc_macro::TokenStream ,input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let k : proc_macro::TokenStream = builder::builder(attr.into(), input.into()).unwrap_or_else(syn::Error::into_compile_error).into();
    return k;
}

#[test]
fn test_macro_quote() {
    use quote::quote;
    use prettyplease;
    let out = builder::builder(
        quote!{Name},
    quote!{
    struct TestStruct{
        #[builder(skip)]
        #[builder(init = 10)]
        id1 : usize,
        #[builder(init = String::from("test"))]
        data : String,
        #[builder(ty = Option<i32>)]
        id2: usize,
        #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
        string: String,
    }}).unwrap();

    let out = syn::parse_file(&out.to_string()).unwrap();
}
