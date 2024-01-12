// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2023 Luke Harding

use crate::*;
use models::VCard;
use sp_vcard::rfc6350::parameters::{BaseType, TelType};
use sp_vcard::rfc6350::values::{
    Address, Email, FullName, IGender, Name, NickName, Note, Organization, Role, Tel, Title, URL,
};
use sp_vcard::rfc6350::VCard40;

pub fn create_vcard(card: VCard) -> VCard40 {
    let mut vc = VCard40::new();

    let mut full_name: String = card.prefix.to_owned();

    if card.firstname.ne("") && card.prefix.eq("") {
        full_name.push_str(&card.firstname.to_string());
    } else {
        full_name.push_str(&format!(" {}", card.firstname));
    }

    if card.middlename.ne("") {
        full_name.push_str(&format!(" {}", card.middlename));
    }

    if card.lastname.ne("") {
        full_name.push_str(&format!(" {}", card.lastname));
    }

    if card.suffix.ne("") {
        full_name.push_str(&format!(" {}", card.suffix));
    }

    vc.full_names.add(FullName::new().set_value(&full_name));

    vc.name.set(
        Name::new()
            .add_given_name(&card.firstname)
            .add_additional_name(&card.middlename)
            .add_family_name(&card.lastname)
            .add_honorific_prefix(&card.prefix)
            .add_honorific_suffix(&card.suffix),
    );

    vc.orgs
        .add(Organization::new().set_value(&card.organization));

    vc.tels.add(
        Tel::new()
            .add_base_type(BaseType::WORK)
            .add_tel_type(TelType::VOICE)
            .set_value(&card.w_phone),
    );

    vc.tels.add(
        Tel::new()
            .add_base_type(BaseType::HOME)
            .add_tel_type(TelType::VOICE)
            .set_value(&card.h_phone),
    );

    vc.tels.add(
        Tel::new()
            .add_base_type(BaseType::HOME)
            .add_tel_type(TelType::CELL)
            .set_value(&card.c_phone),
    );

    vc.tels.add(
        Tel::new()
            .add_base_type(BaseType::HOME)
            .add_tel_type(TelType::VOICE)
            .set_value(&card.p_phone),
    );

    vc.tels.add(
        Tel::new()
            .add_base_type(BaseType::HOME)
            .add_tel_type(TelType::FAX)
            .set_value(&card.h_fax),
    );

    vc.tels.add(
        Tel::new()
            .add_base_type(BaseType::WORK)
            .add_tel_type(TelType::FAX)
            .set_value(&card.w_fax),
    );

    vc.titles.add(Title::new().set_value(&card.title));

    vc.urls.add(URL::new().set_value(&card.url));
    vc.urls.add(URL::new().set_value(&card.workurl));

    vc.notes.add(Note::new().set_value(&card.note));

    vc.nicknames
        .add(NickName::new().add_nickname(&card.nickname));

    let gender = match card.gender.as_ref() {
        "M" => IGender::Male,
        "F" => IGender::Female,
        "O" => IGender::Other,
        _ => IGender::Unknown,
    };
    vc.gender.set(gender);

    vc.roles.add(Role::new().set_value(&card.role));

    vc.emails.add(
        Email::new()
            .add_base_type(BaseType::HOME)
            .set_value(&card.h_email),
    );

    vc.emails.add(
        Email::new()
            .add_base_type(BaseType::WORK)
            .set_value(&card.w_email),
    );

    let h_addr = match decode_address(card.h_address) {
        Ok(addr) => Address::new()
            .street(&addr.street)
            .locality(&addr.locality)
            .region(&addr.region)
            .code(&addr.code)
            .country(&addr.country),
        Err(_) => Address::new(),
    };
    vc.addresses.add(h_addr.add_base_type(BaseType::HOME));

    let w_addr = match decode_address(card.w_address) {
        Ok(addr) => Address::new()
            .street(&addr.street)
            .locality(&addr.locality)
            .region(&addr.region)
            .code(&addr.code)
            .country(&addr.country),
        Err(_) => Address::new(),
    };
    vc.addresses.add(w_addr.add_base_type(BaseType::WORK));

    vc
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StoredAddress {
    street: String,
    locality: String,
    region: String,
    code: String,
    country: String,
}

fn decode_address(encoded: String) -> Result<StoredAddress> {
    match serde_json::from_str::<StoredAddress>(&encoded) {
        Ok(addr) => Ok(addr),
        Err(e) => Err(e.into()),
    }
}
