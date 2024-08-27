use crate::parser::schema::reference::Reference;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefinitionPropertyType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Array,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DefinitionType {
    Object {
        #[serde(flatten)]
        properties: HashMap<String, DefinitionProperty>,
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Array {
        items: Box<DefinitionType>,
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    String {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Number {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Integer {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Boolean {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
}

#[derive(Debug, Deserialize)]
pub struct DefinitionProperty {
    #[serde(flatten)]
    pub schema: DefinitionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub definition_property_type: Option<DefinitionPropertyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    #[serde(flatten)]
    pub schema: DefinitionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<Reference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}