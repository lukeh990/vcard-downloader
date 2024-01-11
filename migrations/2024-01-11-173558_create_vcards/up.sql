-- Your SQL goes here
CREATE TABLE IF NOT EXISTS vcards (
    uuid TEXT NOT NULL UNIQUE,
	email TEXT NOT NULL,
	firstname TEXT NOT NULL,
	lastname TEXT NOT NULL,
	middlename TEXT NOT NULL,
	organization TEXT NOT NULL,
	w_phone TEXT NOT NULL,
	title TEXT NOT NULL,
	url TEXT NOT NULL,
	workurl TEXT NOT NULL,
	note TEXT NOT NULL,
	nickname TEXT NOT NULL,
	prefix TEXT NOT NULL,
	suffix TEXT NOT NULL,
	gender TEXT NOT NULL,
	role TEXT NOT NULL,
	h_phone TEXT NOT NULL,
	c_phone TEXT NOT NULL,
	p_phone TEXT NOT NULL,
	h_fax TEXT NOT NULL,
	w_fax TEXT NOT NULL,
	h_email TEXT NOT NULL,
	w_email TEXT NOT NULL,
	alias TEXT NOT NULL,
	h_address TEXT NOT NULL,
	w_address TEXT NOT NULL,
	PRIMARY KEY (uuid)
) WITHOUT ROWID