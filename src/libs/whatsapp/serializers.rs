use uuid::Uuid;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappText {
    pub body: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappMessage {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<WhatsappText>,
    pub _type: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappProfile {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappContact {
    pub profile: WhatsappProfile,
    pub wa_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappValue {
    pub messaging_product: String,
    pub metadata: WhatsappMetadata,
    pub contacts: Vec<WhatsappContact>,
    pub messages: Vec<WhatsappMessage>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappChange {
    pub value: WhatsappValue,
    pub field: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappEntry {
    pub id: String,
    pub changes: Vec<WhatsappChange>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WhatsappRequest {
    pub object: String,
    pub entry: Vec<WhatsappEntry>,
}
