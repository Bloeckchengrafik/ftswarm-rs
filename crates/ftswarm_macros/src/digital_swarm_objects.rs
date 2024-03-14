use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{LitBool, Token};

struct DigitalSwarmObjectParsed {
    typename: Ident,
    _colon: Token![,],
    has_toggle: LitBool,
}

impl syn::parse::Parse for DigitalSwarmObjectParsed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(DigitalSwarmObjectParsed {
            typename: input.parse()?,
            _colon: input.parse()?,
            has_toggle: input.parse()?,
        })
    }
}

pub fn digital_swarm_object_impl(input: TokenStream) -> TokenStream {
    let parsed: DigitalSwarmObjectParsed = syn::parse(input).unwrap();
    let typename = parsed.typename;
    let has_toggle = parsed.has_toggle.value;

    let toggle_type = if has_toggle {
        quote! {
            impl #typename {
                pub async fn get_toggle(&self) -> Result<ToggleType, String> {
                    return self.run_command(RpcFunction::GetToggle, vec![])
                        .await
                        .and_then(|param| param.as_int().ok_or("Invalid toggle value".to_string()))
                        .and_then(|param| Ok(ToggleType::from(param)))
                }
            }
        }
    } else {
        quote! {}
    };

    return quote! {
        #[derive(Clone)]
        pub struct #typename {
            pub name: String,
            pub value: bool,
            normally_open: NormallyOpen,
            swarm: FtSwarm,
        }

        impl_bool_updateable!(#typename);
        impl_swarm_object!(#typename, NormallyOpen);

        impl NewSwarmObject<NormallyOpen> for #typename {
            default_new_swarm_object_impls!();

            fn new(name: &str, swarm: FtSwarm, normally_open: NormallyOpen) -> Box<Self> {
                Box::new(#typename {
                    name: name.to_string(),
                    value: false,
                    normally_open,
                    swarm,
                })
            }

            fn init(&mut self) -> impl Future<Output=()> {
                async move {
                    self.run_command(
                        RpcFunction::SetSensorType,
                        vec![Argument::SensorType(SensorType::#typename), self.normally_open.clone().into()],
                    ).await.unwrap();

                    self.run_command(
                        RpcFunction::Subscribe,
                        vec![Argument::Int(0i64)],
                    ).await.unwrap();

                    self.value = self.run_command(RpcFunction::GetValue, vec![])
                        .await.ok()
                        .and_then(|param| param.as_int())
                        .unwrap_or(0) == 1;
                }
            }
        }

        #toggle_type
    }.into()
}