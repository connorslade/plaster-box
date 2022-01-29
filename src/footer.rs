use std::fs;

use afire::{
    middleware::{MiddleResponse, Middleware},
    Request, Response,
};

lazy_static! {
    static ref FOOTER: String = fs::read_to_string("web/components/footer.html").unwrap();
}

pub struct Footer;

impl Middleware for Footer {
    fn post(&mut self, _req: Request, res: Response) -> MiddleResponse {
        let text = match String::from_utf8(res.data.clone()) {
            Ok(i) => i,
            Err(_) => return MiddleResponse::Continue,
        };
        let text = text.replace("{{FOOTER}}", &*FOOTER);

        MiddleResponse::Add(res.text(text))
    }
}
