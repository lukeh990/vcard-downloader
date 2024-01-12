# VCard Downloader
Download VCard files from browser
  
Designed to be ran so that when the link is opened in a mobile browser it displays a VCard file in the native viewer.
## Bugs
- Currently all phones, emails, and addresses types are not detected by iOS.
## Running
VCard Downloader currently just supports a SQLite DB but I am working on implementing a MySQL connection as well.
### Environment
#### BIND
The executable accepts the environment variable `BIND` to select which ip and which ports the server will respond to. 
  
Example: `BIND=0.0.0.0:3000 ./vcard-downloader`
  
Default: `0.0.0.0:5000`
#### DATABASE_URL
`DATABASE_URL` specifies the file for the SQLite DB.
This should be in a `.env`

Example: `DATABASE_URL=file:my_db.db3`
  
Default: `file:card_data.db3`
### Behavior
When a request is made to the server, the first thing it does is check if the file is contained by the public directory. If it is not found in the public directory, it will then query the SQLite DB and return the VCard if found.

## Licensing 
A copyright notice is placed at `/` if you fork this project please update it according to the `LICENSE` file
SPDX-License-Identifier: GPL-3.0-only
