mod builder;

#[proc_macro_attribute]
pub fn builder(attr: proc_macro::TokenStream ,input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let k : proc_macro::TokenStream = builder::builder(attr.into(), input.into()).unwrap_or_else(syn::Error::into_compile_error).into();
    return k;
}

#[test]
fn test_macro_quote() {
    use quote::quote;
    use prettyplease::unparse;

    let attrs = quote!{name = Builder, pass = derive(Debug, Default, Serialize, Deserialize)};
    let original = quote!{
    #[derive(Debug, Default, Clone)]
struct Table{
    #[builder(skip)]
    id: usize,
    #[builder(skip_table)]
    ident: String,

    #[builder(ty = String, pass = serde(skip_serializing_if = "String::is_empty"))]
    parent: Option<usize>,

    #[builder(init = String::from("data"))]
    data: String,

    #[builder(skip_setter)]
    complicated: String,
}};

    let out = builder::builder(
        attrs, original.clone()
    ).unwrap();

    let os = unparse(&syn::parse_file(&original.to_string()).unwrap());
    let out = unparse(&syn::parse_file(&out.to_string()).unwrap());
    println!("original:\n{os}");
    println!("parsed:\n{out}");
}



