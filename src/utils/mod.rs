use std::{error, io};
use std::fmt::{Display, Formatter};
use rocket::{Request, Response};
use rocket::http::ContentType;
use rocket::response::Responder;

#[derive(Debug)]
pub struct GenericError {
    details: String,
}

impl GenericError {
    pub fn new(msg: &str) -> GenericError {
        GenericError {
            details: msg.to_string(),
        }
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl<'r> Responder<'r, 'r> for GenericError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'r> {
        let mut response = Response::build()
            .header(ContentType::HTML)
            .status(rocket::http::Status::InternalServerError)
            .finalize();

        let html = format!(r###"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title> {0} </title>
                </head>
                <body align="center">
                    <div role="main" align="center">
                        <h1> {0} </h1>
                        <p> {1} </p>
                        <hr />
                    </div>
                    <div role="contentinfo" align="center">
                        <small>Rust Blog Engine</small>
                    </div>
                </body>
                </html>"###, rocket::http::Status::InternalServerError, &*self.details);
        response.set_sized_body(html.len(), io::Cursor::new(html));
        Ok(response)
    }
}
