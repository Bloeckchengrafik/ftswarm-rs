use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{DeriveInput, Expr, Token};
use quote::quote;
use syn::parse::Parse;


/// This macro generates an empty `Updateable` trait implementation for a given struct.
#[proc_macro_derive(Updateable)]
pub fn updateable_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        use ftswarm_proto::message_parser::rpc::RPCReturnParam;

        impl Updateable for #name {
            fn handle_subscription(&mut self, message: &RPCReturnParam) {
                // No-op
            }
        }
    };
    gen.into()
}

/// This macro generates a sane default implementation for the `Updateable` trait.
#[proc_macro]
pub fn impl_int_updateable(src: TokenStream) -> TokenStream {
    let ast: Ident = syn::parse(src).unwrap();
    let gen = quote! {
        impl Updateable for #ast {
            fn handle_subscription(&mut self, message: &RPCReturnParam) {
                if let RPCReturnParam::Int(value) = message {
                    self.value = *value;
                }
            }
        }

        impl #ast {
            pub async fn get_value(&self) -> i32 {
                self.value
            }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn impl_bool_updateable(src: TokenStream) -> TokenStream {
    let ast: Ident = syn::parse(src).unwrap();
    let gen = quote! {
        impl Updateable for #ast {
            fn handle_subscription(&mut self, message: &RPCReturnParam) {
                if let RPCReturnParam::Int(value) = message {
                    self.value = *value == 1;
                }
            }
        }

        impl #ast {
            pub async fn get_value(&self) -> bool {
                self.value
            }
        }
    };
    gen.into()
}

struct TwoInput {
    a: Expr,
    _comma: Token![,],
    b: Expr,
}

impl Parse for TwoInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            a: input.parse()?,
            _comma: input.parse()?,
            b: input.parse()?,
        })
    }
}

/// This macro generates a sane default implementation for the `NewSwarmObject` trait.
#[proc_macro]
pub fn impl_swarm_object(src: TokenStream) -> TokenStream {
    // parse a, b
    let TwoInput { a: name, _comma: _, b: param } = syn::parse_macro_input!(src as TwoInput);

    let gen = quote! {
        impl SwarmObject<#param> for #name {
        }
    };
    gen.into()
}

/// When using the `default_new_swarm_object_impls` macro, the `name` and `swarm` fields are automatically implemented.
#[proc_macro]
pub fn default_new_swarm_object_impls(_: TokenStream) -> TokenStream {
    return quote! {
        fn name(&self) -> &str {
            &self.name
        }

        fn swarm(&self) -> &FtSwarm {
            &self.swarm
        }
    }.into();
}
