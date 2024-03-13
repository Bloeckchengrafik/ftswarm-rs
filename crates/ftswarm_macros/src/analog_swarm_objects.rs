use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::Token;

struct AnalogSwarmObjectParsed {
    typename: Ident,
}

impl syn::parse::Parse for AnalogSwarmObjectParsed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(AnalogSwarmObjectParsed {
            typename: input.parse()?
        })
    }
}

pub fn analog_swarm_object_impl(input: TokenStream) -> TokenStream {
    let parsed: AnalogSwarmObjectParsed = syn::parse(input).unwrap();
    let typename = parsed.typename;

    return quote! {
        #[derive(Clone)]
        pub struct #typename {
            pub name: String,
            pub hysteresis: Hysteresis,
            pub value: i32,
            swarm: FtSwarm
        }

        impl_int_updateable!(#typename);
        impl_swarm_object!(#typename, Hysteresis);

        impl NewSwarmObject<Hysteresis> for #typename {
            default_new_swarm_object_impls!();

            fn new(name: &str, swarm: FtSwarm, hysteresis: Hysteresis) -> Box<Self> {
                Box::new(#typename {
                    name: name.to_string(),
                    hysteresis,
                    value: 0,
                    swarm
                })
            }

            fn init(&mut self) -> impl Future<Output = ()> {
                async move {
                    self.run_command(
                        RpcFunction::SetSensorType,
                        vec![Argument::SensorType(SensorType::#typename), NormallyOpen::Open.into()]
                    ).await.unwrap();

                    self.run_command(
                        RpcFunction::Subscribe,
                        vec![Argument::Int(self.hysteresis.0.clone() as i64)]
                    ).await.unwrap();

                    self.value = self.run_command(RpcFunction::GetValue, vec![])
                        .await.ok()
                        .and_then(|param| param.as_int())
                        .unwrap_or(0);
                }
            }
        }
    }.into()
}