// Copyright (c) Microsoft. All rights reserved.

use std::fmt;

use failure::Fail;

#[derive(Clone, Copy, Debug, serde_derive::Deserialize, PartialEq, serde_derive::Serialize)]
pub enum AuthType {
    None,
    Sas,
    X509,
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            AuthType::None => "None",
            AuthType::Sas => "Sas",
            AuthType::X509 => "X509",
        };
        write!(f, "{}", s)
    }
}

pub trait Identity {
    fn module_id(&self) -> &str;
    fn managed_by(&self) -> &str;
    fn generation_id(&self) -> &str;
    fn auth_type(&self) -> AuthType;
}

pub struct IdentitySpec {
    module_id: String,
    generation_id: Option<String>,
    managed_by: Option<String>,
}

impl IdentitySpec {
    pub fn new(module_id: String) -> Self {
        IdentitySpec {
            module_id,
            generation_id: None,
            managed_by: None,
        }
    }

    pub fn module_id(&self) -> &str {
        &self.module_id
    }

    pub fn generation_id(&self) -> Option<&str> {
        self.generation_id.as_ref().map(AsRef::as_ref)
    }

    pub fn with_generation_id(mut self, generation_id: String) -> Self {
        self.generation_id = Some(generation_id);
        self
    }

    pub fn managed_by(&self) -> Option<&str> {
        self.managed_by.as_ref().map(AsRef::as_ref)
    }

    pub fn with_managed_by(mut self, managed_by: String) -> Self {
        self.managed_by = Some(managed_by);
        self
    }
}

#[async_trait::async_trait]
pub trait IdentityManager {
    type Identity: Identity;
    type Error: Fail;

    async fn create(&mut self, id: IdentitySpec) -> Result<Self::Identity, Self::Error>;
    async fn update(&mut self, id: IdentitySpec) -> Result<Self::Identity, Self::Error>;
    async fn list(&self) -> Result<Vec<Self::Identity>, Self::Error>;
    async fn get(&self, id: IdentitySpec) -> Result<Option<Self::Identity>, Self::Error>;
    async fn delete(&mut self, id: IdentitySpec) -> Result<(), Self::Error>;
}

// Useful for error contexts
#[derive(Clone, Debug)]
pub enum IdentityOperation {
    CreateIdentity(String),
    DeleteIdentity(String),
    GetIdentity(String),
    ListIdentities,
    UpdateIdentity(String),
}

impl fmt::Display for IdentityOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentityOperation::CreateIdentity(name) => {
                write!(f, "Could not create identity {}", name)
            }
            IdentityOperation::DeleteIdentity(name) => {
                write!(f, "Could not delete identity {}", name)
            }
            IdentityOperation::GetIdentity(name) => write!(f, "Could not get identity {}", name),
            IdentityOperation::ListIdentities => write!(f, "Could not list identities"),
            IdentityOperation::UpdateIdentity(name) => {
                write!(f, "Could not update identity {}", name)
            }
        }
    }
}
