use core::panic;

use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::{ext::IdentExt, parse::{self, Parse, ParseStream}, parse2, spanned::Spanned, token::Comma, Attribute, Data, DeriveInput, Expr, ExprParen, Field, FieldsNamed, Ident, Meta, MetaList, MetaNameValue, PatParen, Path, Type};

pub fn builder(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let span = attr.span();
    return Err(syn::Error::new(span, "not implemented"));
}

enum Initer{
    Default,
    Other(TokenStream),
}

struct StructBuilder{
    original: DeriveInput,
    attrs: Vec<Attribute>,
    iden: Ident,
    fields: Vec<Field>,
}

struct BuilderField{
    attrs: Vec<TokenStream>,
    ident: Ident,
    ty: Type,
    init: Initer,
    skip: bool,
}

fn get_path(attr: &Attribute) -> &Path{
    match &attr.meta{
        Meta::List( MetaList { path, .. } )=> path,
        Meta::Path( path ) => path,
        Meta::NameValue( MetaNameValue { path, .. } ) => path,
    }
}

/*
 * possible builder attrs
 * #[builder(skip)]
 * #[builder(type = Ty)]
 * #[builder(pass = (..))]
 */
fn split(f: Field) -> syn::Result<(BuilderField, Field)>{
    let mut o_field = f.clone();
    let mut o_attrs = Vec::new();
    let mut b_attrs = Vec::new();
    let ident = match f.ident{
        Some(s) => s,
        None => {return Err(syn::Error::new(f.span(), "not a valid field"))}
    };

    for attr in f.attrs{
        let path = get_path(&attr);
        if path.is_ident("builder"){
            b_attrs.push(attr);
        }
        else{
            o_attrs.push(attr);
        }
    }

    o_field.attrs = b_attrs;

    let mut ty = f.ty;
    let mut skip = false;
    let mut init = Initer::Default;
    let mut attrs = Vec::new();
    for attr in o_attrs{
        if let syn::AttrStyle::Inner(_) = attr.style{
            // todo: change to err
            return Err(syn::Error::new(attr.span(), "invalid style"));
        }
        let data = match attr.meta{
            Meta::List(MetaList {tokens, ..}) => tokens,
            _ => {
                return Err(syn::Error::new(attr.meta.span(), "invalid content"));
            }
        };

        let attr : BuilderAttr = parse2(data)?;
        match attr{
            BuilderAttr::Type(typ) => {ty = typ},
            BuilderAttr::Skip => {skip = true},
            BuilderAttr::Init(int) => {init = int},
            BuilderAttr::Pass(ts) => {attrs.push(ts)},
        }
    }

    let attrs = Vec::new();

    return Ok((BuilderField{
        init, ty, skip, ident, attrs
    }, o_field));
}

enum BuilderAttr{
    Skip,
    Init(Initer),
    Pass(TokenStream),
    Type(Type),
}

impl Parse for BuilderAttr{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("{}", input.to_string());
        // this is builder
        let _ : Ident = input.parse()?;
        if input.peek(syn::token::Paren){
        }

        let ident : Ident = input.parse()?;
        if ident.to_string() == "skip"{
            return Ok(Self::Skip);
        }
        if ident.to_string() == "init"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : TokenStream = input.parse()?;
            return Ok(Self::Init(Initer::Other(tokens)));
        }
        if ident.to_string() == "pass"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : TokenStream = input.parse()?;
            return Ok(Self::Pass(tokens));
        }
        if ident.to_string() == "ty"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : Type = input.parse()?;
            return Ok(Self::Type(tokens));
        }

        let error = format!("{} unknown attr", ident.to_string());
        return Err(syn::Error::new(input.span(), error));
    }
}

#[test]
fn test_field() {
    let _ : BuilderAttr = parse2(quote! {
        builder(skip)
    }).unwrap();
    let _ : BuilderAttr = parse2(quote! {
        builder(init = String::from("test"))
    }).unwrap();
    let _ : BuilderAttr = parse2(quote! {
        builder(init = String::from("test"))
    }).unwrap();
}
