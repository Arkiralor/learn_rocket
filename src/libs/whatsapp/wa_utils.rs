use rocket::serde::json::Json;

use crate::libs::whatsapp::serializers::{WhatsappMessage, WhatsappRequest};

pub fn get_messages(req: Json<WhatsappRequest>) -> Vec<WhatsappMessage> {
    return req.entry[0].changes[0].value.messages.clone();
}

pub fn get_message_type(msg: WhatsappMessage) -> String {
    return msg._type;
}

pub fn get_text_body(msg: WhatsappMessage) -> String {
    let body = match msg.text {
        Some(val) => val.body,
        None => String::from("None"),
    };
    return body;
}
