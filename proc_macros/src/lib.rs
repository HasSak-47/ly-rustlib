use std::iter::zip;

use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, DeriveInput, FieldsNamed, Ident, Type};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let DeriveInput {ident, data, ..}= parse_macro_input!(input);

    let builder = match data {
        syn::Data::Struct(s) => match s.fields{
            syn::Fields::Named(named) => {
                let k = BuilderBuilder::new(ident.clone(), named);
                match k{
                    Pair::B(b) => {
                        return b.into();
                    },
                    _ => {},
                }
                quote!{}
            },
            _ => quote!{},
        }
        _ => quote!{},
    };
    println!("{builder}");
    return builder.into();
}

#[derive(Debug, Default)]
enum DefaultOption{
    #[default]
    DefaultTrait,
    SpecifiedFunction(Ident),
    SpecifiedValue(Ident),
}

// god I should not code drunk
struct BuilderBuilder{
    name  : Ident,
    fields: Vec<(Option<Ident>, Type, DefaultOption)>,
}

enum Pair<A, B>{
    A(A),
    B(B)
}

impl BuilderBuilder{
    fn prosses_attr(attr: Attribute) -> Pair<Option<DefaultOption>, TokenStream>{
        let tokens: Vec<TokenTree> = match attr.parse_args::<TokenStream>(){
            Ok(k) => k,
            _ => return Pair::B(quote_spanned!(attr.span() => "could not parse args")),
        }.into_iter().collect();
        match tokens.len(){
            1 => {
                if tokens[0].to_string() == "skip"{
                    Pair::A(None)
                }
                else{
                    Pair::B(quote_spanned!(attr.span() => "invalid argument"))
                }
            }
            2 => {
                Pair::B(quote_spanned!(attr.span() => "I don't know what you are trying to say"))
            }
            3 => {
                match (&tokens[0], &tokens[2]){
                    (TokenTree::Ident(option), TokenTree::Ident(function)) => {
                        if option.to_string() == "default"{
                            Pair::A(Some(DefaultOption::SpecifiedFunction(function.clone())))
                        }
                        else{
                            Pair::B(quote_spanned!(attr.span() => "unknown option"))
                        }
                    },
                    _ => Pair::B(quote_spanned!(attr.span() => "invalid argument")),
                }
            }
            _ => return Pair::B(quote_spanned!(attr.span() => "too many tokens!")),
        }
    }
    fn new(name: Ident, f: FieldsNamed) -> Pair<Self, TokenStream>{
        let mut fields = Vec::new();
        'field_iter : for field in &f.named{
            let option = DefaultOption::DefaultTrait;
            fields.push((field.ident.clone(), field.ty.clone(), option));
        }


        return Pair::A(Self{
            name,
            fields,
        })
    }
}
/*
fn build_fields(name: &Ident, f: FieldsNamed) -> TokenStream {
    let (function, inits) : (Vec<_>, Vec<_>)= f.named.iter().filter(|f|{
        let mut skip = true;
        for attr in f.attrs.iter(){
            let args : proc_macro2::TokenStream = attr.parse_args().unwrap();
            let parts : Vec<_> = args.into_iter().collect();

            if &parts[0].to_string() == "skip"{
                skip = false;
                break;
            }
        }
        skip
    }).map(|f|{
        let (f_name, f_ty) = (&f.ident, &f.ty);
        let mut def_opts =  DefaultOption::DefaultTrait;
        for attr in f.attrs.iter() {
            let args : proc_macro2::TokenStream = attr.parse_args().unwrap();
            let parts : Vec<_> = args.into_iter().collect();
            use TokenTree as TT;
            match (&parts[0], &parts[2]){
                (TT::Ident(def), TT::Ident(data)) => {
                    if def.to_string() == "default"{
                        def_opts = DefaultOption::SpecifiedFunction(data.clone())
                    }
                },
                _ => {},
            };
        }
        (build_field(f_name, f_ty), build_new(f_name, f_ty, def_opts))
    }).unzip();

    let name = format_ident!("{}Builder", name);
    
    let definition = quote!{
    },

    let initer = quote!{
        fn new() -> Self{
            Self{
                #(#inits),*
            }
        }
    };


    let implementations = quote!{impl #name {
        #initer
        #(#function)*
    }};

    return implementations.into();
}

fn build_field(ident: &Option<Ident>, ty: &Type) -> TokenStream{
    quote!{
        pub fn #ident(mut self, #ident: #ty) -> Self{
            self.#ident= #ident;
            return self;
        }
    }
}

fn build_new(ident: &Option<Ident>, ty: &Type, opt: DefaultOption) -> TokenStream{
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
        DefaultOption::SpecifiedValue(val) => quote!{
            #ident: #val.clone()
        },
    }
}
*/
