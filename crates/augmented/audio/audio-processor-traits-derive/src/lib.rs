// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use proc_macro::TokenStream;

use proc_macro2::{Punct, Spacing, Span};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Attribute, Data, DeriveInput, Meta, NestedMeta};

struct CommaSeparatedTokenStreams(Vec<proc_macro2::TokenStream>);

impl ToTokens for CommaSeparatedTokenStreams {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for (i, token) in self.0.iter().enumerate() {
            if i > 0 {
                tokens.append(Punct::new(',', Spacing::Alone));
            }
            token.to_tokens(tokens);
        }
    }
}

fn find_attribute_key_value(attr: &Attribute, key: &str) -> Option<proc_macro2::TokenStream> {
    match attr.parse_meta().unwrap() {
        Meta::List(meta_list) => meta_list.nested.iter().find_map(|meta| match meta {
            NestedMeta::Meta(Meta::NameValue(name_value)) if name_value.path.is_ident(key) => {
                let lit = name_value.lit.clone();
                Some(quote! { #lit })
            }
            _ => None,
        }),
        _ => None,
    }
}

fn expand_handle(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let mut count: usize = 0;
    let parameters = match &ast.data {
        Data::Struct(data_struct) => data_struct
            .fields
            .iter()
            .filter_map(|field| {
                let field_name = field.ident.clone().unwrap().to_string();
                field.attrs.iter().find_map(|attr| {
                    if attr.path.is_ident("parameter") {
                        count += 1;
                        let ident = find_attribute_key_value(attr, "name").unwrap_or({
                            let name = field_name.clone();
                            quote! { #name }
                        });
                        let (min, max) = (
                            find_attribute_key_value(attr, "min").unwrap_or({
                                quote! { 0.0 }
                            }),
                            find_attribute_key_value(attr, "max").unwrap_or({
                                quote! { 1.0 }
                            }),
                        );
                        let step = find_attribute_key_value(attr, "step")
                            .map(|s| {
                                quote! { Some(#s) }
                            })
                            .unwrap_or({
                                quote! { None }
                            });

                        Some((
                            field_name.clone(),
                            quote! {
                                ::audio_processor_traits::parameters::ParameterSpec::new(
                                    #ident.into(),
                                    ::audio_processor_traits::parameters::ParameterType::Float(
                                        ::audio_processor_traits::parameters::FloatType {
                                            range: (#min, #max),
                                            step: #step,
                                        }
                                    )
                                )
                            },
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect(),
        _ => vec![],
    };
    let _parameter_spec_list =
        CommaSeparatedTokenStreams(parameters.iter().cloned().map(|t| t.1).collect());
    let parameter_spec_getters = CommaSeparatedTokenStreams(
        parameters
            .iter()
            .cloned()
            .map(|t| t.1)
            .enumerate()
            .map(|(index, spec)| {
                quote! { #index => #spec }
            })
            .collect(),
    );
    let parameter_getters = CommaSeparatedTokenStreams(
        parameters
            .iter()
            .cloned()
            .map(|t| t.0)
            .enumerate()
            .map(|(index, field_name)| {
                let name = proc_macro2::Ident::new(&field_name, Span::call_site());
                quote! { #index => Some(self.#name.get().into()) }
            })
            .collect(),
    );

    let parameter_setters = CommaSeparatedTokenStreams(
        parameters
            .iter()
            .cloned()
            .map(|t| t.0)
            .enumerate()
            .map(|(index, field_name)| {
                let name = proc_macro2::Ident::new(&field_name, Span::call_site());
                quote! { #index => if let Ok(value) = request.try_into() { self.#name.set(value) } }
            })
            .collect(),
    );

    quote! {
        impl ::audio_processor_traits::parameters::AudioProcessorHandle for #name {
            fn parameter_count(&self) -> usize {
                #count
            }

            fn get_parameter_spec(&self, index: usize) -> ::audio_processor_traits::parameters::ParameterSpec {
                match index {
                    #parameter_spec_getters,
                    _ => ::audio_processor_traits::parameters::ParameterSpec::new(
                        "Invalid".into(),
                        ::audio_processor_traits::parameters::ParameterType::Float(
                            ::audio_processor_traits::parameters::FloatType {
                                range: (0.0, 1.0),
                                step: None,
                            }
                        )
                    )
                }
            }

            fn get_parameter(&self, index: usize) -> Option<::audio_processor_traits::parameters::ParameterValue> {
                match index {
                    #parameter_getters,
                    _ => None
                }
            }

            fn set_parameter(&self, index: usize, request: ::audio_processor_traits::parameters::ParameterValue) {
                match index {
                    #parameter_setters,
                    _ => {}
                }
            }
        }
    }
}

#[proc_macro_derive(AudioProcessorHandle, attributes(parameter))]
pub fn audio_processor_handle(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    expand_handle(&ast).into()
}
