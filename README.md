# Vcard Downloader
Download vcard files from browser
  
Designed to be ran so that when the link is opened in a mobile browser it displays a vcard file in the native viewer.
## Running
The executable accepts the environment variable `BIND` to select which ip and which ports the server will respond to. (i.e. `BIND=0.0.0.0:3000 ./<executable>`)
  
Defaults to `0.0.0.0:5000`
  
	
The server will look for vcards in the `cards/` folder. So a file at `cards/lukeh990.vcf` will be translated to `http://BASEURL/lukeh990`.
  
  
An API health check can be preformed at `GET /`
