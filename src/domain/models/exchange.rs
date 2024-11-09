use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exchange {
    pub id: Uuid,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub pattern: ExchangePattern,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExchangePattern {
    InOnly,
    InOut,
}

impl Exchange {
    pub fn new(body: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            body,
            headers: HashMap::new(),
            properties: HashMap::new(),
            pattern: ExchangePattern::InOnly,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
        self.updated_at = Utc::now();
    }

    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
        self.updated_at = Utc::now();
    }
}
