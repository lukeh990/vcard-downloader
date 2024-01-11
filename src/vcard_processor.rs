// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2023 Luke Harding

use sp_vcard::rfc6350::parameters::BaseType;
use sp_vcard::rfc6350::values::{Category, Email, FullName, IGender, Role, Title};
use sp_vcard::rfc6350::VCard40;

pub fn create_vcard(_id: String) -> VCard40 {
    let mut vc = VCard40::new();

    vc.full_names.add(
        FullName::new()
            .set_value("Hello World")
            .set_language(Some("en"))
            .add_base_type(BaseType::HOME)
            .add_base_type(BaseType::WORK),
    );
    vc.gender.set(IGender::Male);
    vc.emails.add(
        Email::new()
            .set_value("user@example.com")
            .add_base_type(BaseType::WORK),
    );
    vc.categories.add(
        Category::new()
            .add_category("Rust")
    );
    vc.titles.add(
        Title::new()
            .set_value("Rust Developer")
            .add_base_type(BaseType::HOME),
    );
    vc.roles.add(Role::new().set_value("Story Teller"));

    vc
}
