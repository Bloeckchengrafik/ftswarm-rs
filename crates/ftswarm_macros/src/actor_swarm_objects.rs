use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{LitBool, Token};

struct ActorSwarmObject {
    typename: Ident,
    _comma: Token![,],
    digital: bool,
}

impl syn::parse::Parse for ActorSwarmObject {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let typename: Ident = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let normally_open: LitBool = input.parse()?;

        Ok(ActorSwarmObject {
            typename,
            _comma,
            digital: normally_open.value,
        })
    }
}

pub fn actor_swarm_object_impl(input: TokenStream) -> TokenStream {
    let ActorSwarmObject { typename, _comma: _, digital } = syn::parse_macro_input!(input as ActorSwarmObject);

    let impl_block = if digital {
        quote! {
            impl #typename {
                pub async fn set(&self, value: ValueState) -> Result<(), String> {
                    self.run_command(
                        RpcFunction::SetSpeed,
                        vec![Argument::Int(value.into())]
                    ).await.map(|_| ())
                }
            }
        }
    } else {
        quote! {
            impl #typename {
                pub async fn set(&self, value: i32) -> Result<(), String> {
                    let value = value.max(-255).min(255);

                    self.run_command(
                        RpcFunction::SetPosition,
                        vec![Argument::Int(value as i64)]
                    ).await.map(|_| ())
                }
            }
        }
    };

    return quote! {
        #[derive(Clone, Updateable)]
        pub struct #typename {
            pub name: String,
            swarm: FtSwarm
        }

        impl_swarm_object!(#typename, ());

        impl NewSwarmObject<()> for #typename {
            default_new_swarm_object_impls!();

            fn new(name: &str, swarm: FtSwarm, _: ()) -> Box<Self> {
                Box::new(#typename {
                    name: name.to_string(),
                    swarm
                })
            }

            fn init(&mut self) -> impl Future<Output = ()> {
                async move {
                    self.run_command(
                        RpcFunction::SetActorType,
                        vec![Argument::ActorType(ActorType::#typename)]
                    ).await.unwrap();
                }
            }
        }

        #impl_block
    }.into()
}