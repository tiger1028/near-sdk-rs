use crate::info_extractor::ItemTraitInfo;
use quote::quote;
use syn::export::TokenStream2;

impl ItemTraitInfo {
    /// Generate code that wrapps external calls.
    pub fn wrapped_module(&self) -> TokenStream2 {
        let mut result = TokenStream2::new();
        for method in &self.methods {
            result.extend(method.method_wrapper());
        }
        let mod_name = &self.mod_name;
        quote! {
            mod #mod_name {
                #result
            }
        }
    }
}

// Rustfmt removes comas.
#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use syn::ItemTrait;
    use quote::quote;
    use crate::info_extractor::ItemTraitInfo;

    #[test]
    fn standard() {
        let mut t: ItemTrait = syn::parse2(
            quote!{
                    pub trait ExternalCrossContract {
                        fn merge_sort(&self, arr: Vec<u8>) -> PromiseOrValue<Vec<u8>>;
                        fn merge(
                            &self,
                            #[callback]
                            #[serializer(borsh)]
                            data0: Vec<u8>,
                            #[callback]
                            #[serializer(borsh)]
                            data1: Vec<u8>,
                        ) -> Vec<u8>;
                    }
            }
        ).unwrap();
        let info = ItemTraitInfo::new(&mut t, None).unwrap();
        let actual = info.wrapped_module();

        let expected = quote! {
            mod external_cross_contract {
                pub fn merge_sort<T: ToString>(
                    arr: Vec<u8>,
                    __account_id: &T,
                    __balance: near_bindgen::Balance,
                    __gas: near_bindgen::Gas
                ) -> near_bindgen::Promise {
                    #[derive(serde :: Deserialize, serde :: Serialize)]
                    struct Input {
                        arr: Vec<u8>,
                    }
                    let args = Input { arr, };
                    let args = serde_json::to_vec(&args)
                        .expect("Failed to serialize the cross contract args using JSON.");
                    near_bindgen::Promise::new(__account_id.to_string()).function_call(
                        b"merge_sort".to_vec(),
                        args,
                        __balance,
                        __gas,
                    )
                }
                pub fn merge<T: ToString>(__account_id: &T, __balance: near_bindgen::Balance, __gas: near_bindgen::Gas) -> near_bindgen::Promise {
                    let args = vec![];
                    near_bindgen::Promise::new(__account_id.to_string()).function_call(
                        b"merge".to_vec(),
                        args,
                        __balance,
                        __gas,
                    )
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }
}
