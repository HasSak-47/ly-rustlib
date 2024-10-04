use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Ident, Type};

#[proc_macro_derive(Builder)]
pub fn builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let DeriveInput {ident, data, ..}= parse_macro_input!(input);
    let builder = match data {
        syn::Data::Struct(s) => match s.fields{
            syn::Fields::Named(named) => build_fields(&ident, named),
            _ => quote!{},
        }
        _ => quote!{},
    };
    println!("{builder}");
    return builder.into();
}

fn build_fields(name: &Ident, f: FieldsNamed) -> TokenStream {
    let setters = f.named.iter().map(|f|{
        let (f_name, f_ty) = (&f.ident, &f.ty);
        let builder_tags = f.attrs.iter().map(|tag| {
            use syn::Meta as sm;
            match &tag.meta{
                sm::List(l) => {
                    let path = &l.path;
                    let names: Vec<_> = path.segments.iter().map(|p|{&p.ident}).collect();
                },
                _ => {},
            };
        });
        quote!{
            pub fn #f_name(mut self, #f_name: #f_ty) -> Self{
                self.#f_name= #f_name;
                return self;
            }
        }
    });
    let k = quote!{impl #name {
        #(#setters)*
    }};
    return k.into();
}

fn build_field(ident: &Option<Ident>, ty: &Type) -> TokenStream{
    quote!{
        pub fn #ident(mut self, #ident: #ty) -> Self{
            self.#ident= #ident;
            return self;
        }
    }
}
