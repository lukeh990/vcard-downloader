// @generated automatically by Diesel CLI.

diesel::table! {
    vcards (uuid) {
        uuid -> Text,
        email -> Text,
        firstname -> Text,
        lastname -> Text,
        middlename -> Text,
        organization -> Text,
        w_phone -> Text,
        title -> Text,
        url -> Text,
        workurl -> Text,
        note -> Text,
        nickname -> Text,
        prefix -> Text,
        suffix -> Text,
        gender -> Text,
        role -> Text,
        h_phone -> Text,
        c_phone -> Text,
        p_phone -> Text,
        h_fax -> Text,
        w_fax -> Text,
        h_email -> Text,
        w_email -> Text,
        alias -> Text,
        h_address -> Text,
        w_address -> Text,
    }
}
