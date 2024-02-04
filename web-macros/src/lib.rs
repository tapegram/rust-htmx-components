use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, Fields,
    FieldsNamed, Ident, ItemStruct, Token, ParenthesizedGenericArguments,
};

const ATTRS_LEN: usize = 47;
const HTML_ELEMENT_ATTRS: [&str; ATTRS_LEN] = [
    "id",
    "class",
    "onclick",
    "role",
    "aria-orientation",
    "aria-labelledby",
    "tabindex",
    "name",
    "autocomplete",
    "value",
    "placeholder",

    // https://htmx.org/reference/#attributes
    "hx-boost",
    "hx-get",
    "hx-post",
    "hx-on",
    "hx-push-url",
    "hx-select",
    "hx-select-oob",
    "hx-swap",
    "hx-swap-oob",
    "hx-target",
    "hx-trigger",
    "hx-vals",

    // https://htmx.org/reference/#attributes-additional
    "hx-confirm",
    "hx-delete",
    "hx-disable",
    "hx-disabled-elt",
    "hx-disinherit",
    "hx-encoding",
    "hx-ext",
    "hx-headers",
    "hx-history",
    "hx-history-elt",
    "hx-include",
    "hx-indicator",
    "hx-params",
    "hx-patch",
    "hx-preserve",
    "hx-prompt",
    "hx-put",
    "hx-replace-url",
    "hx-request",
    "hx-sse",
    "hx-sync",
    "hx-validate",
    "hx-vars",
    "hx-ws",
];

#[proc_macro_attribute]
pub fn html_element(_: TokenStream, input: TokenStream) -> TokenStream {
    let html_element = parse_macro_input!(input as HtmlElementStruct);
    quote! { #html_element }.to_token_stream().into()
}

struct HtmlElementStruct {
    name: Ident,
    item: ItemStruct,
}

impl Parse for HtmlElementStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = input.parse::<ItemStruct>()?;
        let name = item.ident.clone();

        Ok(HtmlElementStruct { name, item })
    }
}

impl ToTokens for HtmlElementStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let original_item: &ItemStruct = &self.item;
        let item = self.item.clone();

        let original_fields = match item.fields {
            Fields::Named(named) => named.named,
            _ => panic!("not named fields"),
        };

        let attr_idents = create_attr_idents();

        let fields = quote! {
            {
                #original_fields

                #(
                    #[builder(setter(into), default)]
                    #attr_idents: String,
                )*

                #[builder(default)]
                attrs: ::web_client::server::attrs::Attrs,
            }
        };
        let fields: FieldsNamed = syn::parse_quote! { #fields };
        let fields = Fields::Named(fields);

        let item = &ItemStruct {
            fields,
            ..original_item.clone()
        };

        tokens.extend(quote! {
            #[props]
            #[derive(Clone)]
            #item
        });

        let attr_keys = HTML_ELEMENT_ATTRS;
        tokens.extend(quote! {
            impl #name {
                fn html_attrs_to_hashmap(&self) -> std::collections::HashMap<&'static str, String> {
                    let mut map = std::collections::HashMap::new();

                    #(
                        map.insert(#attr_keys, web_client::concat_attribute(&self.#attr_idents, self.attrs.get(#attr_keys)));
                    )*

                    // Check for special case html attributes that are not part of HtmlElementProps
                    if let Some(for_input) = self.attrs.get("for") {
                        map.insert("for", for_input.to_string());
                    }
                    if let Some(for_input) = self.attrs.get("type") {
                        map.insert("type", for_input.to_string());
                    }          

                    let attrs = vec![#(#attr_keys),*];
                    map.extend(self.attrs.to_hashmap_excluding(attrs));

                    map
                }
            }

            impl From<#name> for ::web_client::server::attrs::Attrs {
                fn from(html_props: #name) -> Self {
                    ::web_client::server::attrs::Attrs::from(html_props.html_attrs_to_hashmap())
                }
            }
        });
    }
}

fn create_attr_idents() -> [Ident; ATTRS_LEN] {
    HTML_ELEMENT_ATTRS.map(|attr| {
        let attr = attr.replace('-', "_");
        let attr = attr.as_str();

        Ident::new(attr, Span::call_site())
    })
}

// Right now we only support omit transform in spread_attr!
enum Transformer {
    Omit(ParenthesizedGenericArguments),
}

type TransformerFn = (Ident, ParenthesizedGenericArguments);

#[derive(Debug)]
struct AttrsSpread {
    props: Ident,
    transforms: Vec<TransformerFn>,
}

impl Parse for AttrsSpread {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut transforms: Vec<TransformerFn> = Vec::new();

        let props: Ident = input.parse()?;
    
        while !input.is_empty() { 
            input.parse::<Token![|]>()?;

            let fn_name: Ident = input.parse()?;    
            let fn_args: ParenthesizedGenericArguments = input.parse()?;

            transforms.push((fn_name, fn_args));
        }

        Ok(AttrsSpread { props, transforms })
    }
}

#[proc_macro]
pub fn spread_attrs(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as AttrsSpread);
    let AttrsSpread { props, transforms } = ast;

    let mut attr_keys = HTML_ELEMENT_ATTRS.clone().to_vec();
    let mut attr_idents = create_attr_idents().to_vec();

    for (fn_name, fn_args) in transforms {
        let transform = match fn_name.to_string().as_str()  {
            "omit" => Transformer::Omit(fn_args),
            _ => panic!("Unrecognized pipe transfomer: `{}`. Valid pipe transformers: `omit`", fn_name),
        };

        transform_attrs(transform, &mut attr_keys, &mut attr_idents);
    }

    let gen = quote! {
        {
            let mut map = std::collections::HashMap::new();
            #(
                map.insert(#attr_keys, web_client::concat_attribute(&#props.#attr_idents, #props.attrs.get(#attr_keys)));
            )*

            let attrs = vec![#(#attr_keys),*];
            map.extend(#props.attrs.to_hashmap_excluding(attrs));

            ::web_client::server::attrs::Attrs::from(map)
        }
    };

    gen.into()
}

fn transform_attrs(transform: Transformer, attr_keys: &mut Vec<&str>, attr_idents: &mut Vec<Ident>) {
    match transform {
        Transformer::Omit(args) => {
            for arg in args.inputs {
                match arg {
                    syn::Type::Path(path) => {
                        let ident = path.path.get_ident().unwrap();
                        if !HTML_ELEMENT_ATTRS.contains(&ident.to_string().replace('_', "-").as_str()) {
                            panic!(
                                "Cannot omit field {}. It doesn't exit in HtmlElementProps.",
                                ident,
                            );
                        }
                        attr_keys.retain(|f| f != &ident.to_string());
                        attr_idents.retain(|f| ident != &f.to_string());
                    },
                    _ => panic!("Expected a path"),
                }
            }
        },
    }
}
