//! Definitions for DID document types.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DidDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@context")]
    pub context: Option<Vec<String>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<Vec<VerificationMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Service>>,
}

impl DidDocument {
    pub fn get_did(&self) -> String {
        self.id.clone()
    }
    pub fn get_handle(&self) -> Option<String> {
        if let Some(also_known_as) = &self.also_known_as {
            also_known_as
                .iter()
                .find_map(|s| s.strip_prefix("at://").map(String::from))
        } else {
            None
        }
    }
    pub fn get_signing_key(&self) -> Option<(String, String)> {
        self.get_verification_material("atproto")
    }
    pub fn get_verification_material(&self, key_id: &str) -> Option<(String, String)> {
        let did = self.get_did();
        if let Some(keys) = &self.verification_method {
            keys.iter()
                .find(|key| key.id == format!("#{key_id}") || key.id == format!("{did}#{key_id}"))
                .filter(|key| key.public_key_multibase.is_some())
                .map(|key| {
                    (
                        key.r#type.clone(),
                        key.public_key_multibase.clone().unwrap(),
                    )
                })
        } else {
            None
        }
    }
    pub fn get_pds_endpoint(&self) -> Option<String> {
        self.get_service_endpoint(("#atproto_pds", Some("AtprotoPersonalDataServer")))
    }
    pub fn get_service_endpoint(&self, (id, r#type): (&str, Option<&str>)) -> Option<String> {
        if let Some(services) = &self.service {
            let did = self.get_did();
            // TODO: validate url
            services
                .iter()
                .find(|service| service.id == id || service.id == format!("{did}{id}"))
                .filter(|service| r#type.map_or(true, |t| service.r#type == t))
                .map(|service| service.service_endpoint.clone())
        } else {
            None
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_multibase: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub r#type: String,
    // TODO: enum?
    pub service_endpoint: String,
}
