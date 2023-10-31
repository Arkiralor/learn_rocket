use rocket::serde::json::Json;

use crate::libs::whatsapp::serializers::{WhatsappMessage, WhatsappRequest};

pub fn parse_body(req: WhatsappRequest) -> Vec<WhatsappMessage> {
    return req.entry[0].changes[0].value.messages.clone();
}
