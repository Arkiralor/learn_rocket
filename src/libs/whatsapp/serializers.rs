use uuid::Uuid;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappImage {
    pub caption: String,
    pub mime_type: String,
    pub sha_256: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappReaction {
    pub msg_id: String,
    pub emoji: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappSticker {
    pub mime_type: String,
    pub sha_256: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappError {
    pub code: i64,
    pub details: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappUnknown {
    pub errors: Vec<WhatsappError>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappButton {
    pub text: String,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappInteractive {
    pub button_reply: Option<WhatsappButtonReply>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappText {
    pub body: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WhatsappMessage {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: Option<WhatsappText>,
    pub reaction: Option<WhatsappReaction>,
    pub image: Option<WhatsappImage>,
    pub sticker: Option<WhatsappSticker>,
    pub unknown: Option<WhatsappUnknown>,
    pub button: Option<WhatsappButton>,
    pub interactive: Option<WhatsappInteractive>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappProfile {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappContact {
    pub profile: WhatsappProfile,
    pub wa_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappValue {
    pub messaging_product: String,
    pub metadata: WhatsappMetadata,
    pub contacts: Vec<WhatsappContact>,
    pub messages: Vec<WhatsappMessage>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappChange {
    pub value: WhatsappValue,
    pub field: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappEntry {
    pub id: String,
    pub changes: Vec<WhatsappChange>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WhatsappRequest {
    pub object: String,
    pub entry: Vec<WhatsappEntry>,
}
