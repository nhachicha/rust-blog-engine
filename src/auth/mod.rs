/*
 * Copyright 2021 Nabil Hachicha.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rocket::response::{Debug};
use serde::{Deserialize};
use rocket::request::{self, FromRequest};
use hyper::{Client as HyperClient, Method, Request as HyperRequest, Body as HyperBody, Error as HyperError};
use rocket::{Request};
use hyper_tls::HttpsConnector;
use rocket::outcome::IntoOutcome;
use rocket_oauth2::{TokenResponse};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| UserSession(id))
            .or_forward(())
    }
}

pub async fn authenticate(token: TokenResponse<GoogleUserInfo>) -> Result<String, Debug<HyperError>> {
    let https = HttpsConnector::new();
    let client = HyperClient::builder().build::<_, hyper::Body>(https);
    // Use the token to retrieve the user's Google account information.
    let req = HyperRequest::builder()
        .method(Method::GET)
        .uri("https://www.googleapis.com/oauth2/v3/userinfo")
        .header("Authorization", format!("Bearer {}", token.access_token().to_string()))
        .header("Accept", "application/json")
        .body(HyperBody::empty()).unwrap();

    let resp = client.request(req).await?;
    let body = hyper::body::to_bytes(resp.into_body()).await?;
    let user: GoogleAuthUser = serde_json::from_slice(&body).unwrap();

    Ok(user.sub)
}


#[derive(serde::Deserialize)]
pub struct GoogleUserInfo;

#[derive(Debug)]
pub struct UserSession(pub String);

#[derive(Deserialize, Debug)]
struct GoogleAuthUser {
    sub: String, // Google ID
    name: String,
    given_name: String,
    family_name: String,
    picture: String
}
