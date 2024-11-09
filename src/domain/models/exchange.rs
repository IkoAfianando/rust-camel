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
    pub processing_history: Vec<ProcessingStep>,
    pub metadata: ExchangeMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingStep {
    pub processor_name: String,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: i64,
    pub success: bool,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeMetadata {
    pub source_system: String,
    pub correlation_id: Option<String>,
    pub priority: String,
    pub retry_count: u32,
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
            processing_history: Vec::new(),
            metadata: ExchangeMetadata {
                source_system: "create".to_string(),
                correlation_id: Uuid::new_v4().to_string().into(),
                priority: "normal".to_string(),
                retry_count: 0,
            },
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
    
    pub fn add_processing_step(&mut self, processor_name: &str, duration_ms: i64, success: bool, notes: Option<String>) {
        self.processing_history.push(ProcessingStep {
            processor_name: processor_name.to_string(),
            timestamp: Utc::now(),
            duration_ms,
            success,
            notes,
        });
        self.updated_at = Utc::now();
    }
}
