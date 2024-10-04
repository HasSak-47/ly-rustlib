use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, DeriveInput, FieldsNamed, Ident, Type};

#[proc_macro_attribute]
pub fn builder(attr: proc_macro::TokenStream ,input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let attr : TokenStream = attr.into();
    let DeriveInput {mut ident, data, attrs, ..} = parse_macro_input!(input);
    let attr : Vec<_> = attr.into_iter().collect();
    if attr.len() != 0{
        match &attr[0]{
            TokenTree::Ident(idnt) => {
                ident = idnt.clone();
            },
            _ => {},
        }
    }
    else{
        ident = format_ident!("{ident}Builder");
    }

    let builder = match data {
        syn::Data::Struct(s) => match s.fields{
            syn::Fields::Named(named) => {
                new(ident.clone(), named, attrs)
            },
            _ => quote!{},
        }
        _ => quote!{},
    };
    return builder.into();
}

#[derive(Debug, Default)]
enum DefaultOption{
    #[default]
    DefaultTrait,
    Skip,
    SpecifiedFunction(Ident),
    _SpecifiedValue(Ident),
}

enum Pair<A, B>{
    A(A),
    B(B)
}

macro_rules! proc_error {
    ($error: literal , $var: expr ) => {
        quote_spanned!($var.span() => compile_error!($error))
        
    };
}

fn prosses_attr(attr: &Attribute) -> Pair<DefaultOption, TokenStream>{
    let tokens: Vec<TokenTree> = match attr.parse_args::<TokenStream>(){
        Ok(k) => k,
        _ => return Pair::B(proc_error!("could not parse args", attr)),
    }.into_iter().collect();
    match tokens.len(){
        1 => {
            if tokens[0].to_string() == "skip"{
                Pair::A(DefaultOption::Skip)
            }
            else{
                Pair::B(proc_error!("Builder: Unknown argument", tokens[0].span() ))
            }
        }
        2 => {
            Pair::B(proc_error!("I don't know what you are trying to say", attr))
        }
        3 => {
            match (&tokens[0], &tokens[2]){
                (TokenTree::Ident(option), TokenTree::Ident(function)) => {
                    if option.to_string() == "default"{
                        Pair::A(DefaultOption::SpecifiedFunction(function.clone()))
                    }
                    else{
                        Pair::B(proc_error!("Unknown option", option))
                    }
                },
                _ => Pair::B(proc_error!("Invalid argument", attr)),
            }
        }
        _ => return Pair::B(proc_error!("Too many tokens!", attr)),
    }
}
fn new(name: Ident, f: FieldsNamed, attrs: Vec<Attribute>) -> TokenStream{
    let mut fields = Vec::new();
    for field in &f.named{
        let mut option = DefaultOption::DefaultTrait;
        for attr in &field.attrs{
            if attr.meta.path().to_token_stream().to_string() != "builder" {
                continue;
            }
            option = match prosses_attr(attr){
                Pair::A(a) => a,
                Pair::B(b) => return b,
            };
            break;
        }
        if let  DefaultOption::Skip = option {
            continue;
        }
        fields.push((field.ident.clone(), field.ty.clone(), option));
    }

    let (names, types) : (Vec<_>, Vec<_>)= fields.iter().map(|f| (f.0.clone(), f.1.clone())).unzip();

    let (sets, inits) : (Vec<_>, Vec<_>) = fields.into_iter().map(|f| (build_field_set(&f.0, &f.1), build_init(&f.0, &f.1, f.2))).unzip();

    let struct_declaration = quote!{
        struct #name{
            #(pub #names : #types),*
        }
    };

    let implementation = quote!{
        impl #name {
            pub fn new() -> Self{ Self{
                #(#inits),*
            }}
            #(#sets)*
        }
    };


    return quote!(#(#attrs)* #struct_declaration #implementation);
}

fn build_init(ident: &Option<Ident>, ty: &Type, opt: DefaultOption) -> TokenStream{
    match opt{
        DefaultOption::DefaultTrait => {
            let typ = match ty.clone(){
                Type::Path(p) => {
                    let tokens : Vec<_> = p.into_token_stream().into_iter().collect();
                    tokens[0].to_token_stream()
                }
                _ => {
                    quote!{
                        <#ty>
                    }
                }
            };
            quote!{
                #ident: #typ::default()
            }
        },
        DefaultOption::SpecifiedFunction(func) => quote!{
            #ident: #func()
        },
        DefaultOption::_SpecifiedValue(val) => quote!{
            #ident: #val.clone()
        },
        DefaultOption::Skip => unreachable!(),
    }
}

fn build_field_set(ident: &Option<Ident>, ty: &Type) -> TokenStream{
    quote!{
        pub fn #ident(mut self, #ident: #ty) -> Self{
            self.#ident= #ident;
            return self;
        }
    }
}
