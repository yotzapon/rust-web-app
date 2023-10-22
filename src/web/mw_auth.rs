use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use crate::ctx::Ctx;

pub async fn mw_required_auth<B>(
    ctx: Result<Ctx>, // or you can use ->  ctx: Option<Ctx>,
    req: Request<B>,
    next: Next<B>) -> Result<Response>{
    println!("->> {:12} - mw_required_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// region: --- ctx Extractor
#[async_trait]
impl <S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        println!("->> {:12} - Ctx", "EXTRACTOR");

        // User cookies extractor.
        let cookies  = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token.
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO: Token components validation

        Ok(Ctx::new(user_id))
    }
}
// endregion: --- ctx Extractor

/// Parse a token of formant `user-[user-id].[expiration].[signature]
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string() ))
}