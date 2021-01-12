use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::*,
    error::image::*
};
use utils::{
    fetch::{api_with_auth, api_with_auth_empty, api_upload_file}
};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use url::Url;
use web_sys::File;

pub async fn create_image(file:File, kind: ImageKind) -> Result<String, ()> {
    match _create_image_api(kind).await {
        Err(_) => { return Err(()) },
        Ok(res) => {
            let CreateResponse { id} = res;
            let id = id.0.to_string();

            let path = Upload::PATH.replace("{id}",&id);
            api_upload_file(&path, &file, Upload::METHOD)
                .await
                .map_err(|_| ())
                .map(|_| id)
        }
    }
}


async fn _create_image_api(kind: ImageKind) -> Result < <Create as ApiEndpoint>::Res, <Create as ApiEndpoint>::Err> {
    let req:<Create as ApiEndpoint>::Req = CreateRequest {
        name: "".to_string(),
        description: "".to_string(),
        is_premium: false,
        publish_at: None,
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new(),
        kind
    };

    api_with_auth(Create::PATH, Create::METHOD, Some(req)).await
}
/*

    //needs to be a function due to orphan rule
    fn category_id_from_str(id:&str) -> CategoryId {
        CategoryId(uuid_from_str(id))
    }
    //needs to be a function due to orphan rule
    fn uuid_from_str(id:&str) -> Uuid {
        Uuid::parse_str(id).unwrap_throw()
    }

    pub async fn get_all() -> Result < <Get as ApiEndpoint>::Res, <Get as ApiEndpoint>::Err> {
        let req:<Get as ApiEndpoint>::Req = GetCategoryRequest {
            ids: Vec::new(), 
            scope: Some(CategoryTreeScope::Decendants)
        };
        
        let query = serde_qs::to_string(&req).unwrap_throw();

        let path = api_url(&format!("{}?{}", Get::PATH, query)); 

        api_with_auth::<_,_,()>(&path, Get::METHOD, None).await
    }
    */
