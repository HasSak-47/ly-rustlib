use core::panic;

use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse2, parse_macro_input, spanned::Spanned, token::{Bracket, Colon, Eq, Pound}, Attribute, Data, DeriveInput, ExprGroup, Field, FieldsNamed, Ident, Path, Token, Type, Visibility};

macro_rules! proc_error {
    ($error: literal , $var: expr ) => {
        quote_spanned!($var.span() => compile_error!($error))
        
    };
}
pub fn builder(attr: TokenStream, input: TokenStream) -> TokenStream {
    proc_error!("idk man", attr)
}

// #[builder(skip | ( type & ( builder ) & pass(attrs)) )]
// #[builder(skip | ( ty = Ty & (value = val | def_impl) & attrs(#[...])) )]
// param: type,
enum Param{
    Skip,
    Build{
        name: Ident,
        ty: TokenStream,
        builder: ParamBuilder,
        attrs: Vec<Attribute>
    }
}

impl Parse for Param{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_inner(input)?;
        let _field = Field::parse_named(input)?;
        for attr in attrs {
            let _ = attr.parse_args_with(ParamBuilder::parse);
        }

        return Err(syn::Error::new(input.span(), "invalid sequence"));
    }
}

#[test]
fn test_param(){
    let _ : Param = parse2(quote! {
        #[builder(def_v = String::from("text") ty = String, pass(#[serde(skip)]))]
        param: std::Vec
    }).unwrap();
}

// should be something in specific like String::from("test") or just Default::default(),
enum ParamBuilder{
    Value(TokenStream),
    Trait,
}

impl Parse for ParamBuilder{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident : Ident = input.parse()?; 
        let ident = ident.to_string();
        if ident == "def_val"{
            let _ : Eq = input.parse()?;
            let value : TokenStream  = input.parse()?;
            return Ok(Self::Value(value))
        }
        if ident == "def_impl"{
            return Ok(Self::Trait);
        }
        return Err(syn::Error::new(input.span(), "invalid option"));
    }
}

struct Parameter{
    param: Ident,
    ty: Type,
}

/*
#[builder(name = Task,
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
)]
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    pub(crate) min_time: Duration,

    #[builder(other_type = Option<String>)]
    pub(crate) parent_task: Option<usize>,

    #[builder(other_type = Option<String>)]
    pub(crate) project    : Option<usize>,

    #[builder(skip)]
    pub(crate) id : usize,
}
*/
