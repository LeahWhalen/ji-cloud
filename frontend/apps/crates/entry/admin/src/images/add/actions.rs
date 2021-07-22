use shared::{
    domain::image::*,
    error::*,
    api::{ApiEndpoint, endpoints},
};
use utils::{
    prelude::*,
    routes::*
};
use dominator::clone;
use super::state::*;
use std::rc::Rc;
use web_sys::File;

pub fn on_change(state: Rc<State>, value: String) {
    match value.as_ref() {
        "sticker" => {
            *state.kind.borrow_mut() = ImageKind::Sticker;
        },
        "canvas" => {
            *state.kind.borrow_mut() = ImageKind::Canvas;
        },
        _ => {
            log::info!("unknown value [{}]", value);
        }
    }
}

pub fn on_file(state: Rc<State>, file: File) {
    state.loader.load(clone!(state => async move {
        let req = ImageCreateRequest {
            name: "".to_string(),
            description: "".to_string(),
            is_premium: false,
            publish_at: None,
            tags: Vec::new(),
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            categories: Vec::new(),
            kind: state.kind.borrow().clone()
        };

        match api_with_auth::<CreateResponse, MetadataNotFound, _>(endpoints::image::Create::PATH, endpoints::image::Create::METHOD, Some(req)).await {
            Ok(resp) => {
                let CreateResponse { id} = resp;

                let req = ImageUploadRequest {
                    file_size: file.size() as usize
                };

                let path = endpoints::image::Upload::PATH.replace("{id}",&id.0.to_string());

                match api_with_auth::<ImageUploadResponse, EmptyError, _>(&path, endpoints::image::Upload::METHOD, Some(req)).await {
                    Ok(resp) => {
                        let ImageUploadResponse {session_uri} = resp;
                        
                        match upload_file_gcs(&session_uri, &file).await {
                            Ok(_) => {
                                log::info!("file uploaded!");
                            },
                            Err(_) => {
                            },
                        }

                    },

                    Err(err) => {
                        log::error!("error getting image upload uri!")
                    }
                }
            },
            Err(err) => {
                log::error!("error creating image id!")
            }
        }
    }))
}

async fn upload_file(file: File, id: ImageId, session_uri: &str) {
    log::info!("Uploading {} to {}", id.0.to_string(), session_uri);
}
