use std::borrow::ToOwned;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use actix_files as fs;
use actix_web::{get, http::header, web, Error, HttpRequest, HttpResponse, Result};

#[get("/media/{tail:.*}")]
pub async fn static_files(req: HttpRequest) -> Result<HttpResponse, Error> {
    let file_path_str = format!(".{}", req.path());
    let file_path = Path::new(&file_path_str);
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();
    let file_ext = file_path.extension().unwrap().to_str().unwrap().to_owned();
    let file_mime = fs::file_extension_to_mime(&file_ext);
    let content_type = format!("{}/{}", file_mime.type_(), file_mime.subtype());

    let res = web::block(move || -> Result<String, ()> {
        let mut f = File::open(file_path_str).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        Ok(buffer)
    })
    .await;

    match res {
        Ok(resp) => Ok(HttpResponse::Ok()
            .header(
                header::CONTENT_DISPOSITION,
                header::ContentDisposition {
                    disposition: header::DispositionType::Inline,
                    parameters: vec![header::DispositionParam::Filename(file_name.to_owned())],
                },
            )
            .content_type(content_type)
            .content_length(resp.len() as u64)
            .body(resp)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}
