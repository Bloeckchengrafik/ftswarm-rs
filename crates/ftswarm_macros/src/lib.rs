use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;


#[proc_macro_derive(Updateable)]
pub fn updateable_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Updateable for #name {
            fn handle_subscription(&mut self, message: &RPCReturnParam) {
                // No-op
            }
        }
    };
    gen.into()
}