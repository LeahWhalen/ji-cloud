use crate::{db, error, extractor::TokenUser};
use actix_web::dev::ConnectionInfo;
use chrono::{Duration, Utc};
use core::settings::RuntimeSettings;
use paperclip::actix::{
    api_v2_operation,
    web::{self, Data, Json},
    CreatedJson,
};
use shared::{
    api::{endpoints::jig::player, ApiEndpoint},
    domain::jig::{
        player::{JigPlayerSession, JigPlayerSessionCode, JigPlayerSessionToken},
        JigId,
    },
};
use sqlx::PgPool;

use crate::extractor::IPAddress;
use crate::token::{create_auth_token, create_auth_token_no_cookie, generate_csrf};

/// Create a jig player session for the author, if one does not exist already.
#[api_v2_operation]
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<player::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<player::Create as ApiEndpoint>::Res>, error::JigCode> {
    let req = req.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(req.jig_id.clone())).await?;

    let index = db::jig::player::create(&db, req.jig_id, req.settings).await?;

    Ok(CreatedJson(JigPlayerSessionCode { index }))
}

/// Create a jig player session for someone who's not the author, if one doesn't already exist
/// todo: finish this route (make me async)
#[api_v2_operation]
pub fn create_player_session(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    claims: TokenUser,
    ip_addr: IPAddress,
    req: Json<<player::CreatePlayerSession as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<player::CreatePlayerSession as ApiEndpoint>::Res>, error::JigCode> {
    // Check to make sure user is authorized. Is this necessary?
    let req = req.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(req.jig_id.clone())).await?;

    let session_id =
        db::jig::player::create_user_session(&db, req.jig_id, req.settings, ip_addr).await?;
    // Create a new row in ig_player_session_instance (return instance ID)

    // Generate a short-lived access token that will authenticate the next API
    let session_duration = Duration::minutes(20);

    let csrf = generate_csrf();

    let now = Utc::now();

    let token: String = create_auth_token_no_cookie(
        &settings.token_secret,
        session_duration,
        &session_id,
        csrf.clone(),
        now,
    )?;

    // this access token contains the "instance ID" -> subject
    // Return this access token

    Ok(CreatedJson(JigPlayerSessionToken { token }))
}

/// Get the player session identified by the code, if it exists.
#[api_v2_operation]
pub async fn get(
    db: Data<PgPool>,
    path: web::Path<i16>,
) -> Result<Json<<player::Get as ApiEndpoint>::Res>, error::JigCode> {
    let code = path.into_inner();

    let res = db::jig::player::get(&*db, code)
        .await?
        .ok_or(error::JigCode::ResourceNotFound)?;

    Ok(Json(JigPlayerSession {
        jig_id: res.0,
        settings: res.1,
    }))
}

/// Fetch a jig player session code from it's jig if it exists.
#[api_v2_operation]
pub async fn get_code(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<player::GetPlayerSessionCode as ApiEndpoint>::Res>, error::JigCode> {
    let id = path.into_inner();

    let index = db::jig::player::get_code(&*db, id)
        .await?
        .ok_or(error::JigCode::ResourceNotFound)?;

    Ok(Json(JigPlayerSessionCode { index }))
}
