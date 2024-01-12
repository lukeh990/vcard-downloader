// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2023 Luke Harding

use crate::*;
use models::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StoredAddress {
    street: String,
    locality: String,
    region: String,
    code: String,
    country: String,
}

enum BaseType {
    Home,
    Work,
}

impl BaseType {
    fn as_str(&self) -> &'static str {
        match self {
            BaseType::Home => "HOME",
            BaseType::Work => "WORK",
        }
    }
}

enum TelType {
    Voice,
    Cell,
    Fax,
}

impl TelType {
    fn as_str(&self) -> &'static str {
        match self {
            TelType::Voice => "VOICE",
            TelType::Cell => "CELL",
            TelType::Fax => "FAX",
        }
    }
}

pub fn create_vcard(card: VCard) -> String {
    let mut vcard_string: String = "BEGIN:VCARD\nVERSION:3.0\n".to_owned();

    let mut formatted_name: String = "".to_owned();
    if card.prefix.ne("") {
        formatted_name.push_str(&card.prefix)
    }
    if card.firstname.ne("") {
        formatted_name.push_str(&format!(" {}", card.firstname));
    }
    if card.middlename.ne("") {
        formatted_name.push_str(&format!(" {}", card.middlename));
    }
    if card.lastname.ne("") {
        formatted_name.push_str(&format!(" {}", card.lastname));
    }
    if card.suffix.ne("") {
        formatted_name.push_str(&format!(" {}", card.suffix));
    }

    vcard_string.push_str(&format!("FN;CHARSET=UTF-8:{}\n", formatted_name));

    vcard_string.push_str(&format!(
        "N;CHARSET=UTF-8:{};{1};{2};{3};{4}\n",
        card.lastname, card.firstname, card.middlename, card.prefix, card.suffix
    ));

    match decode_address(card.h_address) {
        Ok(addr) => vcard_string.push_str(&address_string(addr, BaseType::Home)),
        Err(_) => (),
    }
    match decode_address(card.w_address) {
        Ok(addr) => vcard_string.push_str(&address_string(addr, BaseType::Work)),
        Err(_) => (),
    }

    if card.w_phone.ne("") {
        vcard_string.push_str(&tel_string(
            card.w_phone,
            TelType::Voice,
            Some(BaseType::Work),
        ));
    }

    if card.h_phone.ne("") {
        vcard_string.push_str(&tel_string(
            card.h_phone,
            TelType::Voice,
            Some(BaseType::Home),
        ));
    }

    if card.c_phone.ne("") {
        vcard_string.push_str(&tel_string(card.c_phone, TelType::Cell, None));
    }

    if card.p_phone.ne("") {
        vcard_string.push_str(&tel_string(card.p_phone, TelType::Voice, None));
    }

    if card.h_fax.ne("") {
        vcard_string.push_str(&tel_string(card.h_fax, TelType::Fax, Some(BaseType::Home)));
    }

    if card.w_fax.ne("") {
        vcard_string.push_str(&tel_string(card.w_fax, TelType::Fax, Some(BaseType::Work)));
    }

    if card.organization.ne("") {
        vcard_string.push_str(&format!("ORG;CHARSET=UTF-8:{}\n", card.organization));
    }

    if card.title.ne("") {
        vcard_string.push_str(&format!("TITLE;CHARSET=UTF-8:{}\n", card.title));
    }

    /*
    TODO:
    - [ ] URLS
    - [ ] Notes
    - [ ] Nickname
    - [ ] Role
    - [ ] Email
    - [X] UID
    - [X] Addresses
    - [X] Phones
    - [X] Names
    */

    vcard_string.push_str(&format!("UID;CHARSET=UTF-8:{}\n", card.uuid));

    vcard_string.push_str("END:VCARD");
    vcard_string
}

fn decode_address(encoded: String) -> Result<StoredAddress> {
    match serde_json::from_str::<StoredAddress>(&encoded) {
        Ok(addr) => Ok(addr),
        Err(e) => Err(e.into()),
    }
}

fn address_string(addr: StoredAddress, addr_type: BaseType) -> String {
    format!(
        "ADR;CHARSET=UTF-8;TYPE={}:;;{1};{2};{3};{4};{5}\n",
        addr_type.as_str(),
        addr.street,
        addr.locality,
        addr.region,
        addr.code,
        addr.country
    )
}

fn tel_string(number: String, tel_type: TelType, base_type: Option<BaseType>) -> String {
    let mut type_string = "".to_owned();
    type_string.push_str(tel_type.as_str());

    match base_type {
        Some(base_type) => type_string.push_str(&format!(",{}", base_type.as_str())),
        None => (),
    }

    format!("TEL;TYPE={}:{1}\n", type_string, number)
}
