use std::{collections::HashMap, io::Write};

use super::DiscoveryDocument;
use crate::{codegen::utils::discover_specific_apis, error::Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct GoogleServicesAPIBuilder {
    client: reqwest::Client,
    services: Vec<String>,
}

impl Default for GoogleServicesAPIBuilder {
    fn default() -> Self {
        GoogleServicesAPIBuilder {
            client: reqwest::Client::new(),
            services: vec![],
        }
    }
}

impl GoogleServicesAPIBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_service_api(self, service: &str) -> Self {
        let mut services = self.services;
        services.push(service.to_string());
        GoogleServicesAPIBuilder {
            client: self.client,
            services,
        }
    }

    fn generate_service_module(&self, doc: &DiscoveryDocument) -> TokenStream {
        let module_name = format_ident!("{}", doc.name.to_lowercase());
        let client_struct = self.generate_client_struct(doc);
        let client_trait_impl = self.generate_impl_google_service_client(doc);
        // let client_resource_defs = self.generate_client_resource_defs(doc); 

        quote! {
            mod #module_name {
                use crate::oauth::AccessToken;

                #client_struct
                
                #client_trait_impl

                // #client_resource_defs
            }
        }
    }
    
    fn generate_client_struct(&self, doc: &DiscoveryDocument) -> TokenStream {
        let client_struct_name = format_ident!("{}Client", doc.name);
        let root_url = &doc.root_url;
        let service_path = &doc.service_path;
        let version = &doc.version;

        // let client_root_method_impl = self.generate_client_root_method_impl(doc);
    
        quote! {
            pub struct #client_struct_name {
                client: reqwest::Client,
                access_token: String,
                scopes: Vec<String>,
                root_url: String,
                service_path: String,
                version: String
            }
    
            impl #client_struct_name {
                pub fn new(access_token: AccessToken, scopes: Vec<String>) -> Self {
                    Self {
                        client: reqwest::Client::new(),
                        access_token,
                        scopes,
                        ..Default::default()
                    }
                }

                // #client_root_method_impl
            }

            impl Default for #client_struct_name {
                /// Use default service account to get and auth token for the service
                fn default() -> Self {
                    Self {
                        client: reqwest::Client::new(),
                        access_token: AccessToken::default(),
                        scopes: Self::get_scopes(),
                        root_url: #root_url.to_string(),
                        service_path: #service_path.to_string(),
                        version: #version.to_string()
                    }
                }
            }
        }
    }

    fn generate_impl_google_service_client(&self, doc: &DiscoveryDocument) -> TokenStream {
        let client_struct_name = format_ident!("{}Client", doc.name);

        // Assume scopes are defined somewhere in the DiscoveryDocument, for this example we use a placeholder
        let scopes: Vec<String> = doc.extract_all_scopes();

        // Implement the GoogleServiceClient trait
        let implementation = quote! {
            impl GoogleServiceClient for #client_struct_name {
                fn set_access_token(&mut self, token: AccessToken) {
                    self.access_token = token;
                }

                fn get_scopes(&self) -> Vec<String> {
                    vec![#(#scopes),*]
                }

                fn discover(&self) -> crate::error::Result<DiscoveryDocument> {
                    let url = format!("{}discovery/v1/apis/{}/{}", self.root_url, #client_struct_name::API_NAME, self.version);
                    let resp = self.client.get(&url).bearer_auth(&self.access_token).send()?;
                    let doc = resp.json::<DiscoveryDocument>()?;
                    Ok(doc)
                }
            }
        };

        implementation
    }

    pub async fn build(self) -> Result<()> {
        // Obtain API discovery documents
        let items = discover_specific_apis(&self.client, self.services.clone()).await?;
        let mut docs: Vec<DiscoveryDocument> = Vec::new();
        for item in items {
            let doc = item.get_discovery_document(&self.client).await?;
            docs.push(doc);
        }

        // print general info
        println!("Number of APIs: {}", docs.len());
        let mut mods: HashMap<String, TokenStream> = HashMap::new();
        for (i, doc) in docs.iter().enumerate() {
            print_basic_profile(&i, &doc);
            let module = self.generate_service_module(&doc);
            mods.insert(doc.name.clone(), module);
        }

        //print first module
        let first_module = mods.values().next().unwrap();
        println!("{}", first_module);

        // Write to file
        let mut file = std::fs::File::create("src/services/mod.rs")?;
        for module in mods.values() {
            file.write_all(module.to_string().as_bytes())?;
        }



        Ok(())
    }
    
    // fn generate_client_root_method_impl(&self, doc: &DiscoveryDocument) -> TokenStream {
    //     todo!()
    // }
    
    // fn generate_client_resource_defs(&self, doc: &DiscoveryDocument) -> TokenStream {
    //     let mut resource_defs = Vec::new();
    //     for (resource_name, resource) in doc.resources.iter() {
    //         let resource_name = format_ident!("{}", resource_name);
    //         let resource_fields = self.generate_client_resource_fields(resource);
    //         let resource_methods = self.generate_client_resource_methods(resource);
    //         let resource_def = quote! {
    //             pub struct #resource_name {
    //                 #resource_fields
    //             }

    //             impl #resource_name {
    //                 #resource_methods
    //             }
                
    //         };
    //         resource_defs.push(resource_def);
    //     }
    //     let combined = quote! {
    //         #(#resource_defs)*
    //     };
    //     combined
    // }
    
    // fn generate_client_resource_fields(&self, resource: &super::Resource) -> TokenStream {
    //     todo!()
    // }
    
    // fn generate_client_resource_methods(&self, resource: &super::Resource) -> TokenStream {
    //     todo!()
    // }
}

fn print_basic_profile(index: &usize, doc: &DiscoveryDocument) {
    println!("-----------------------------------");
    println!("API {}: {} aka \"{}\"", index, doc.name, doc.title);
    println!("-----------------------------------");
    println!("Root Methods: {:?}", doc.methods.keys().collect::<Vec<_>>());

    // Print resources and methods
    let resource_keys = doc.resources.keys().collect::<Vec<_>>();
    println!("Resources: {:?}", resource_keys);
    for k in resource_keys {
        let methods = doc
            .resources
            .get(k)
            .unwrap()
            .methods
            .keys()
            .collect::<Vec<_>>();
        println!("- Methods for '{}': {:?}", k, methods);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_google_service_api_builder() {
        let builder = GoogleServicesAPIBuilder::new()
        .with_service_api("keep");

        builder.build().await.unwrap();
    }
}
