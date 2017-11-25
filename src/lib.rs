extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Body, VariantData};

#[proc_macro_derive(NewType)]
pub fn newtype(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_newtype_derive(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_newtype_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    match ast.body {
        Body::Struct(VariantData::Tuple(ref fields)) => if fields.len() == 1 {
            let name = &ast.ident;
            let (impl_vars, type_vars, where_clause) = ast.generics.split_for_impl();
            let contained_type = &fields.iter().next().unwrap().ty;
            quote! {
                impl#impl_vars ::std::convert::From<#contained_type> for #name#type_vars #where_clause {
                    fn from(a: #contained_type) -> Self {
                        #name(a)
                    }
                }

                impl#impl_vars ::std::ops::Deref for #name#type_vars #where_clause {
                    type Target = #contained_type;
                    fn deref(&self) -> &#contained_type {
                        &self.0
                    }
                }
            }
        } else {
            panic!(
                "NewType only supports single value newtypes, your type has {} fields.",
                fields.len()
            )
        },
        _ => panic!("NewType only supports single value newtypes."),
    }
}
