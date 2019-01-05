extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Generics, GenericParam, Ident,
    parse_macro_input, parse_quote,
};

#[proc_macro_derive(AsTuple)]
pub fn derive_as_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut generics = input.generics.clone();
    generics.params.push(parse_quote!('__as_tuple));
    let generics = add_bounds(generics);
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let tuple_type = tuple_gen(&input.data, TupleOptions::TypeSig { mutable: false });
    let tuple_type_mut = tuple_gen(&input.data, TupleOptions::TypeSig { mutable: true });
    let destructure_expr = destructure(&name, &input.data, false);
    let destructure_expr_mut = destructure(&name, &input.data, true);
    let expanded = quote! {
        impl #impl_generics ::as_tuple::AsTuple<'__as_tuple> for #name #ty_generics #where_clause {
            type Tuple = #tuple_type;
            type TupleMut = #tuple_type_mut;

            fn as_tuple(&'__as_tuple self) -> Self::Tuple {
                #destructure_expr
            }

            fn as_tuple_mut(&'__as_tuple mut self) -> Self::TupleMut {
                #destructure_expr_mut
            }
        }
    };
    //panic!("{}", expanded.to_string());
    proc_macro::TokenStream::from(expanded)
}

fn add_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        match *param {
            GenericParam::Type(ref mut type_param) => {
                type_param.bounds.push(parse_quote!('__as_tuple));
            },
            GenericParam::Lifetime(ref mut lifetime_param) => {
                lifetime_param.bounds.push(parse_quote!('__as_tuple));
            },
            _ => (),
        }
    }
    generics
}

#[derive(Debug)]
enum TupleOptions {
    TypeSig { mutable: bool },
    Binding,
    Ref { mutable: bool },
}

fn tuple_gen(data: &Data, options: TupleOptions) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields
                        .named
                        .iter()
                        .map(|f| match options {
                            TupleOptions::TypeSig { mutable } => if !mutable {
                                let ty = &f.ty;
                                quote! { &'__as_tuple #ty }
                            } else {
                                let ty = &f.ty;
                                quote! { &'__as_tuple mut #ty }
                            },
                            TupleOptions::Binding => {
                                let name = f.ident.as_ref().unwrap();
                                quote! { #name }
                            },
                            _ => unimplemented!(),
                        });
                    quote! { (#(#recurse),*) }
                },
                Fields::Unnamed(ref fields) => {
                    let recurse = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, f)| match options {
                            TupleOptions::TypeSig { mutable } => if !mutable {
                                let ty = &f.ty;
                                quote! { &'__as_tuple #ty }
                            } else {
                                let ty = &f.ty;
                                quote! { &'__as_tuple mut #ty }
                            },
                            TupleOptions::Binding => {
                                quote! { field_#i }
                            },
                            TupleOptions::Ref { mutable } => if !mutable {
                                quote! { &self.#i }
                            } else {
                                quote! { &mut self.#i }
                            },
                        });
                    quote! { (#(#recurse),*) }
                },
                Fields::Unit => quote!{ () },
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn destructure(name: &Ident, data_root: &Data, mutable: bool) -> TokenStream {
    match *data_root {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| if !mutable {
                        let name = f.ident.as_ref().unwrap();
                        quote! { ref #name }
                    } else {
                        let name = f.ident.as_ref().unwrap();
                        quote! { ref mut #name }
                    });
                    let tuple_bind = tuple_gen(data_root, TupleOptions::Binding);
                    quote! {
                        let #name { #(#recurse),* } = self;
                        #tuple_bind
                    }
                },
                Fields::Unnamed(_) => {
                    let tuple = tuple_gen(data_root, TupleOptions::Ref { mutable });
                    quote! { #tuple }
                },
                Fields::Unit => quote!{ () },
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
