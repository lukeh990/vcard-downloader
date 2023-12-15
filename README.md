# Vcard Downloader
Download vcard files from browser
  
Designed to be ran so that when the link is opened in a mobile browser it displays a vcard file in the native viewer.
## Running
### Enviroment
#### BIND
The executable accepts the environment variable `BIND` to select which ip and which ports the server will respond to. 
  
Example: `BIND=0.0.0.0:3000 ./vcard-downloader`
  
Default: `0.0.0.0:5000`
#### DB_PATH
`DB_PATH` specifies the file for the SQLite DB.
  
Example: `DB_PATH=./my_db.db3 ./vcard-downloader`
  
Default: `card_data.db3`
### Behavior
The server will look for vcards in the `cards/` folder. So a file at `cards/lukeh990.vcf` will be translated to `http://BASEURL/lukeh990`.
  
A copyright notice is placed at `/` if you fork this project please update it according to the `LICENSE` file
