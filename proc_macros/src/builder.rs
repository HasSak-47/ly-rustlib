use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse2, spanned::Spanned, token::{Comma}, Data, DeriveInput, Expr, Field, Ident, Meta, MetaList, MetaNameValue, Path, Type};

pub fn builder(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let span = input.span();
    let mut derived : DeriveInput = parse2(input.clone())?;
    match &mut derived.data{
        Data::Struct(d) => {
            for field in &mut d.fields{
                field.attrs = field.attrs.clone().into_iter().filter(|p| {
                    match &p.meta{
                        Meta::Path (p)  => {
                            if p.is_ident("builder_skip") {
                                return false; 
                            }
                        }
                        Meta::List(l)  => {
                            if l.path.is_ident("builder_pass") || l.path.is_ident("builder"){
                                return false;
                            }
                        }
                        _ => { }
                    }
                    return true;
                }).collect();
            }
        },
        _ => {
            return Err(syn::Error::new(span, "expected data struct"));
        }

    }
    let DeriveInput {mut ident, data, attrs, ..} = parse2(input)?;

    // parse the attr of the struct
    let attr_fields : FieldAttrParser = parse2(attr)?;
    for field in attr_fields.0{
        if field.0.is_ident("name") {
            ident = parse2(field.1.into_token_stream())?;
        }
    }

    // get the field of the struct
    let fields = match data{
        Data::Struct(d) => {
            d.fields
        },
        _ => {
            return Err(syn::Error::new(span, "expected data struct"));
        }
    };

    let mut v = Vec::new();

    for field in fields{
        // parse the field and the attrs
        let new_field : FieldOptions = parse2( field.to_token_stream() )?;
        match &new_field{
            FieldOptions::Skip => { continue; },
            _ => {},
        }
        v.push(new_field);
    }
    let (fields, initers) : (Vec<_>, Vec<_>)= v.into_iter().map(|field| return match field {
        FieldOptions::Build { ident, ty, initer, attrs } => {
            (NewField{attrs, ty, ident: ident.clone()}, Init{initer, ident})
        },
        _ => unreachable!(),
    }).unzip();

    let sets : Vec<_> = fields.iter().map(FieldSet::from).collect();

    return Ok(quote!{
        #derived
        #(#attrs)*
        pub struct #ident{
            #(#fields),*
        }

        impl #ident{
            pub fn new() -> Self{
                Self{
                    #(#initers),*
                }
            }

            #(#sets)*
        }
    });
}

struct NewField{
    attrs  : Vec<TokenStream>,
    ident  : Ident,
    ty     : Type,
}

impl ToTokens for NewField{
    fn to_token_stream(&self) -> TokenStream {
        let attr = &self.attrs;
        let ident = &self.ident;
        let ty    = &self.ty;
        let _k : Vec<_> = attr.iter().map(|f| {return f.to_token_stream().to_string()} ).collect();
        quote!(
            #(#[#attr])*
            #ident: #ty
        )
    }

    fn into_token_stream(self) -> TokenStream{
        self.to_token_stream()
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = self.to_token_stream();
        tokens.extend(ts);
    }
}

// #[builder(skip | ( type & ( builder ) & pass(attrs)) )]
// #[builder(skip | ( ty = Ty & (value = val | def_impl) & attrs(#[...])) )]
// param: type,
enum FieldOptions{
    Skip,
    Build {
        ident: Ident,
        ty: Type,
        initer: FieldIniter,
        attrs  : Vec<TokenStream>,
    }
}

struct Init{
    ident: Ident,
    initer: FieldIniter,
}

enum FieldIniter {
    Impl,
    Expr(Expr),
}

struct FieldSet{
    ident: Ident,
    ty: Type,
}

impl ToTokens for Init{
    fn to_token_stream(&self) -> TokenStream {
        let init = match &self.initer{
            FieldIniter::Impl => quote!{ Default::default() },
            FieldIniter::Expr(expr) => quote!{ #expr }
        };
        let ident = &self.ident;

        quote!(
            #ident: #init
        )
    }

    fn into_token_stream(self) -> TokenStream{
        self.to_token_stream()
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = self.to_token_stream();
        tokens.extend(ts);
    }
}

impl From<&NewField> for FieldSet{
    fn from(value: &NewField) -> Self {
        Self{
            ident: value.ident.clone(),
            ty: value.ty.clone(),
        }
    }
}

impl ToTokens for FieldSet{
    fn to_token_stream(&self) -> TokenStream {
        let ident = &self.ident;
        let ty = &self.ty;

        quote!(
            pub fn #ident(mut self, #ident: #ty) -> Self{
                self.#ident = #ident;
                return self;
            }
        )
    }

    fn into_token_stream(self) -> TokenStream{
        self.to_token_stream()
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = self.to_token_stream();
        tokens.extend(ts);
    }
}

// in charge of seprarating all the options in #[builder(opt, ...)]
struct FieldAttrParser(Vec<(Path, Expr)>);

impl Parse for FieldAttrParser{
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

// parse #[attr] ident: ty
impl Parse for FieldOptions{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let Field {attrs, mut ty, ident, ..}= Field::parse_named(input)?;
        let mut ident = ident.unwrap();

        // if #[builder_skip] skip
        // note it will be changed at some point to #[builder(skip)]
        if attrs.iter().find(|f|
            if let Meta::Path(path) = &f.meta {
                path.is_ident("builder_skip")
            } else {
                false
        }).is_some(){
            return Ok(FieldOptions::Skip);
        }

        // parse all #[builder(...)]
        let builder_attr = attrs.iter().find(|f|
            if let Meta::List(MetaList{path, ..}) = &f.meta {
                path.is_ident("builder")
            } else {
                false
        });

        // all the option = stmt stuff inside #[builder(...)]
        let p_attrs = match builder_attr{
            Some(t) => {
                t.parse_args_with(FieldAttrParser::parse)?
            },
            None => {
                FieldAttrParser(Vec::new())
            },
        };

        let mut builder = FieldIniter::Impl;
        for attr in p_attrs.0{
            if attr.0.is_ident("name"){
                ident = parse2(attr.1.to_token_stream())?;
                continue;
            }
            if attr.0.is_ident("def_expr"){
                builder = FieldIniter::Expr(attr.1);
                continue;
            }
            if attr.0.is_ident("ty"){
                ty = parse2(attr.1.to_token_stream())?;
                continue;
            }
        }

        // parse all #[builder_pass]
        let attrs = attrs.into_iter()
            .filter_map(|attr| {
                if let Meta::List(MetaList{path, tokens, ..}) = attr.meta{
                    if ! path.is_ident("builder_pass"){
                        return None;
                    }
                    return Some(tokens); 
                }
                return None;
            }).collect();

        return Ok(Self::Build{
            initer: builder,
            attrs,
            ty,
            ident,
        });
    }
}

#[test]
fn test_param(){
    let _ : FieldOptions = parse2(quote! {
        #[serde(skip)]
        #[builder(def_expr = String::from("text"), ty = String, pass = ((serde(skip))) )]
        bytes: std::Vec<u8>
    }).unwrap();
    let _ : FieldOptions = parse2(quote! {
        #[builder(def_expr = String::from("text"), ty = String, pass = ((serde(skip))) )]
        #[serde(skip)]
        bytes: std::Vec<u8>
    }).unwrap();
}
