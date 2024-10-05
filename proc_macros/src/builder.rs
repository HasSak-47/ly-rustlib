use core::panic;

use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse2, parse_macro_input, spanned::Spanned, token::{Bracket, Colon, Comma, Eq, Pound}, Attribute, Data, DeriveInput, Expr, ExprGroup, Field, FieldsNamed, Ident, Meta, MetaList, MetaNameValue, Path, Stmt, Token, Type, Visibility};

macro_rules! proc_error {
    ($error: literal , $var: expr ) => {
        quote_spanned!($var.span() => compile_error!($error))
        
    };
}

pub fn builder(attr: TokenStream, input: TokenStream) -> Resul<TokenStream> {
    let str : DeriveInput = parse2(input)?;

    return Err("not implemented :3");
}

// #[builder(skip | ( type & ( builder ) & pass(attrs)) )]
// #[builder(skip | ( ty = Ty & (value = val | def_impl) & attrs(#[...])) )]
// param: type,
enum Param{
    Skip,
    Build{
        ident: Ident,
        ty: Type,
        builder: ParamBuilder,
        attrs  : Vec<Attribute>,
    }
}

enum ParamBuilder {
    Impl,
    Expr(Expr),
}

struct ParamAttrParser(Vec<(Path, Expr)>);

impl Parse for ParamAttrParser{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::ext::IdentExt;
        let mut v = Vec::new();

        while input.lookahead1().peek(Ident::peek_any) {
            let MetaNameValue {path, value, ..} = input.parse()?; 
            let r : Result<Comma, _> = input.parse();
            v.push((path, value));
            if r.is_err(){
                break;
            }
        }

        return Ok(Self(v));
    }

}

impl Parse for Param{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let Field {attrs, mut ty, ident, ..}= Field::parse_named(input)?;
        let mut ident = ident.unwrap();

        let builder_attr = attrs.iter().find(|f| if let Meta::List(MetaList{path, ..}) = &f.meta {
            path.is_ident("builder")
        } else {
            false
        });

        let p_attrs = match builder_attr{
            Some(t) => {
                t.parse_args_with(ParamAttrParser::parse)?
            },
            None => {
                ParamAttrParser(Vec::new())
            },
        };

        let mut builder = ParamBuilder::Impl;
        for attr in p_attrs.0{
            if attr.0.is_ident("skip"){
                println!("skip");
                return Ok(Self::Skip);
            }
            if attr.0.is_ident("name"){
                println!("name");
                ident = parse2(attr.1.to_token_stream())?;
                continue;
            }
            if attr.0.is_ident("def_expr"){
                println!("expr");
                builder = ParamBuilder::Expr(attr.1);
                continue;
            }
            if attr.0.is_ident("ty"){
                println!("ty");
                ty = parse2(attr.1.to_token_stream())?;
                continue;
            }
            if attr.0.is_ident("pass"){
                println!("pass");
                continue;
            }
        }

        println!("return");
        return Ok(Self::Build{
            builder,
            attrs,
            ty,
            ident,
        });
    }
}

#[test]
fn test_param(){
    let _ : Param = parse2(quote! {
        #[serde(skip)]
        #[builder(def_expr = String::from("text"), ty = String, pass = (serde(skip)) )]
        bytes: std::Vec<u8>
    }).unwrap();
    let _ : Param = parse2(quote! {
        #[builder(def_expr = String::from("text"), ty = String, pass = (serde(skip)) )]
        #[serde(skip)]
        bytes: std::Vec<u8>
    }).unwrap();
}
