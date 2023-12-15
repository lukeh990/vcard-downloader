/*
Luke Harding (@lukeh990) - Main entrypoint of program
Copyright (C) 2023 Luke Harding

Licensing information can be found in the "LICENSE" file in the root directory

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use rouille::{Response, router};
use std::env;
use std::fs::File;
use mime_guess;

mod vcard;

fn main() {
    let bind: String = match env::var("BIND") {
        Ok(port) => port,
        Err(_e) => "0.0.0.0:5000".to_string()
    };

    println!("Starting server on {}", bind);

    rouille::start_server(bind, move |request| {
        router!(request,
            (GET) (/) => {
                if let Ok(file) = File::open("public/index.html") {
                    return Response::from_file("text/html", file);
                }

                Response::text("404 Not Found").with_status_code(404)
            },

            (GET) (/{id: String}) => {
                if let Ok(public) = File::open(format!("public/{}", id)) {
                    let mime_type = match mime_guess::from_path(format!("public/{}", id)).first() {
                        Some(x) => x.to_string(),
                        None => {
                             return Response::text("500 Server Error").with_status_code(500);
                        }
                    };
                    return Response::from_file(mime_type, public);
                } if let Ok(card) = File::open(format!("cards/{}.vcf", id)) {
                    let data = vcard::create_vcard(id).to_string();
                    return Response::from_data("text/x-vcard", data);
                }

                Response::text("404 Not Found").with_status_code(404)
            },

            _ => Response::text("404 Not Found").with_status_code(404)
        )
    });
}
