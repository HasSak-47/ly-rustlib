use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse2, spanned::Spanned, Attribute, Data, DeriveInput, Field, Fields, Ident, Meta, MetaList, MetaNameValue, Path, Type};

enum BuilderAttrs{
    Name(Ident),
    Pass(TokenStream),
    SkipInit,
}

impl Parse for BuilderAttrs{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?.to_string();
        if ident == "name"{
            let _ : syn::token::Eq = input.parse()?;
            let name : Ident = input.parse()?;
            return Ok(Self::Name(name));
        }
        if ident == "skip_init"{
            return Ok(Self::SkipInit);
        }
        if ident == "pass"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : TokenStream = input.parse()?;
            return Ok(Self::Pass(tokens));
        }

        let error = format!("{} unknown attr", ident.to_string());
        return Err(syn::Error::new(input.span(), error));
    }
}

pub fn parse_attr(attr: TokenStream) -> syn::Result<Ident>{
    parse2(attr)
}

pub fn builder(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let mut di : DeriveInput = parse2(input)?;
    let struct_name = parse_attr(attr)?;

    let (b_fields, o_fields) : (Vec<_>, Vec<_>,) = match di.data.clone(){
        Data::Struct(s) =>{
            let mut v = Vec::new();
            for field in s.fields{
                v.push(split(field)?);
            }
            v
        },
        _ => { return Err(syn::Error::new(di.span(), "not implemented for this data type")); }
    }.into_iter().unzip();
    let b_fields = b_fields.into_iter().filter(|f| !f.skip);


    if let Data::Struct(v) = &mut di.data {
        if let Fields::Named(named) = &mut v.fields{
            named.named.clear();
            for field in o_fields{
                named.named.push_value(field);
                named.named.push_punct(syn::token::Comma::default());
            }
        }
    }

    // there is a better way to do this bullshit
    // i don't want to think lmao
    let (setter, initer_field ): (Vec<_>, Vec<(_, _)> ) = b_fields
        .into_iter()
        .map(|f| (f.make_setter(), (f.make_initer(), f.make_field())))
        .unzip();
    let (initer, field) : (Vec<_>, Vec<_>,)= initer_field.into_iter().unzip();
    


    let q = quote! {
        #di
        struct #struct_name{
            #(#field),*
        }

        impl #struct_name {
            pub fn new() -> Self{
                Self{ #(#initer)* }
            }

            #(#setter)*
        }
    };
    return Ok(q);
}

enum Initer{
    Default,
    Other(TokenStream),
}

impl ToTokens for Initer{
    fn to_token_stream(&self) -> TokenStream{
        match self{
            Self::Default =>
                quote! { Default::default() },
            Self::Other(ts)=>
                quote! { #ts },
        }
    }

    fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(self.to_token_stream()) }
    fn into_token_stream(self) -> TokenStream { self.to_token_stream() }

}

struct BuilderField{
    attrs: Vec<TokenStream>,
    ident: Ident,
    ty: Type,
    init: Initer,
    skip: bool,
}

impl BuilderField{
    fn make_initer(&self) -> TokenStream{
        if self.skip { return TokenStream::new(); }
        let ident = &self.ident;
        let init  = &self.init;
        quote! {
            #ident : #init,
        }
    }
    fn make_setter(&self) -> TokenStream{
        if self.skip { return TokenStream::new(); }
        let ident = &self.ident;
        let ty    = &self.ty;
        quote! {
            pub fn #ident(mut self, #ident: #ty) -> Self{
                self.#ident = #ident;
                return self;
            }
        }
    }
    fn make_field(&self) -> TokenStream{
        if self.skip { return TokenStream::new(); }
        let attrs = &self.attrs;
        let ident = &self.ident;
        let ty    = &self.ty;
        quote! {
            #(#[#attrs])*
            pub #ident : #ty
        }
    }
}

/**
 * gets #[path(...)] #[path] #[path = ...]
 */ 
fn get_path(attr: &Attribute) -> &Path{
    match &attr.meta{
        Meta::List( MetaList { path, .. } )=> path,
        Meta::Path( path ) => path,
        Meta::NameValue( MetaNameValue { path, .. } ) => path,
    }
}

/**
 * it splits the field into the field of the builder and the original one
 * the original one is the same as the one passed to the function
 * but without builder attrs
 * possible builder attrs
 * #[builder(skip)]
 * #[builder(type = Ty)]
 * #[builder(pass = (..))]
 *
 * returns (BuilderField, Original Field)
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

    o_field.attrs = o_attrs;

    let mut ty = f.ty;
    let mut skip = false;
    let mut init = Initer::Default;
    let mut attrs = Vec::new();

    for attr in b_attrs{
        if let syn::AttrStyle::Inner(_) = attr.style{
            // todo: change to err
            // ^-- what does this mean??
            return Err(syn::Error::new(attr.span(), "invalid style"));
        }
        match &attr.meta{
            Meta::List(_) => {},
            _ => {
                return Err(syn::Error::new(attr.meta.span(), "invalid content"));
            }
        };
        let attr : BuilderFieldAttrs = attr.parse_args()?;
        match attr{
            BuilderFieldAttrs::Type(typ) => {ty = typ},
            BuilderFieldAttrs::Skip => {skip = true},
            BuilderFieldAttrs::Init(int) => {init = int},
            BuilderFieldAttrs::Pass(ts) => {attrs.push(ts)},
        }
    }

    return Ok((BuilderField{
        init, ty, skip, ident, attrs
    }, o_field));
}

enum BuilderFieldAttrs{
    Skip,
    Init(Initer),
    Pass(TokenStream),
    Type(Type),
}

impl Parse for BuilderFieldAttrs{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?.to_string();
        if ident == "skip"{
            return Ok(Self::Skip);
        }
        if ident == "init"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : TokenStream = input.parse()?;
            return Ok(Self::Init(Initer::Other(tokens)));
        }
        if ident == "pass"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : TokenStream = input.parse()?;
            return Ok(Self::Pass(tokens));
        }
        if ident == "ty"{
            let _ : syn::token::Eq = input.parse()?;
            let tokens : Type = input.parse()?;
            return Ok(Self::Type(tokens));
        }

        let error = format!("{} unknown attr", ident.to_string());
        return Err(syn::Error::new(input.span(), error));
    }
}

#[test]
fn test_attr_args() {
    let _ : BuilderFieldAttrs = parse2(quote! {
        skip
    }).unwrap();
    let _ : BuilderFieldAttrs = parse2(quote! {
        init = String::from("test")
    }).unwrap();
    let _ : BuilderFieldAttrs = parse2(quote! {
        ty = Option<i32>
    }).unwrap();
    let _ : BuilderFieldAttrs = parse2(quote! {
        pass = serde(skip_serializing_if = "String::is_empty")
    }).unwrap();
}
