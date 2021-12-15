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

use hyper::{Error as HyperError};
use rocket::form::{Context, Contextual, Form};
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Debug, Redirect};
use rocket::State;
use rocket_dyn_templates::Template;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};

use crate::auth::{authenticate, GoogleUserInfo, UserSession};
use crate::database;
use crate::database::MongoDB;
use crate::model::{BlogEntry};
use crate::utils::GenericError;

// -------------------------- P U B L I C  R O U T E S -------------------------- //
#[get("/")]
pub async fn blog_entries(database: &State<database::MongoDB>) -> Result<Template, GenericError> {
    get_blog_entries(database, None).await
}

// -------------------------- A D M I N  R O U T E S -------------------------- //
#[get("/")]
pub async fn admin_blog_entries(database: &State<database::MongoDB>, user: UserSession) -> Result<Template, GenericError> {
    get_blog_entries(database, Some(UserSessionData {
        edit_mode: true,
        logged_in_user: user.0,
    })).await
}

#[get("/new")]
pub fn new_blog(_user: UserSession) -> Template {
    Template::render("blog_entry", &Context::default())
}

#[post("/blog", data = "<form>")]
pub async fn new_blog_post(mut form: Form<Contextual<'_, BlogEditViewContext>>, database: &State<database::MongoDB>, _user: UserSession) -> Result<Redirect, Template> {
    match form.value {
        Some(ref mut blog) => {
            if blog.data._id.is_empty() {
                // add a new entry
                database.add_blog(&mut blog.data).await.ok();
            } else {
                // update existing one
                database.update_blog(&mut blog.data).await.ok();
            }
            Ok(Redirect::to("/admin"))
        }
        None => Err(Template::render("blog_entry", &form.context))
    }
}

#[get("/edit/<id>")]
pub async fn edit_blog(database: &State<database::MongoDB>, id: String, _user: UserSession) -> Result<Template, GenericError> {
    match database.find_blog(id).await {
        Ok(blog) => {
            let context = BlogEditViewContext {
                data: blog,
                session: None
            };
            Ok(Template::render("blog_entry", &context))
        }
        Err(error) => {
            Err(GenericError::new(&*format!("{:?}", error)))
        }
    }
}

#[get("/delete/<id>")]
pub async fn delete_blog(database: &State<database::MongoDB>, id: String, _user: UserSession) -> Result<Redirect, GenericError> {
    match database.delete_blog(id).await {
        Ok(_) => {
            Ok(Redirect::to("/admin"))
        }
        Err(error) => {
            Err(GenericError::new(&*format!("{:?}", error)))
        }
    }
}

async fn get_blog_entries(database: &State<MongoDB>, user_session: Option<UserSessionData>) -> Result<Template, GenericError> {
    // for authenticated users don't filter out the draft blogs so we can edit them
    let user_session = user_session.unwrap_or(UserSessionData {
        edit_mode: false,
        logged_in_user: String::new(),
    });
    match database.fetch_all_published_blogs(!user_session.edit_mode).await {
        Ok(blogs) => {
            let context = PublishedBlogEntriesViewContext {
                data: blogs,
                session: Some(user_session),
            };
            Ok(Template::render("index", &context))
        }
        Err(error) => {
            Err(GenericError::new(&*format!("{:?}", error)))
        }
    }
}

#[derive(Serialize, Deserialize, FromForm, Debug)]
struct UserSessionData {
    #[field(default = "no_token")]
    logged_in_user: String,
    edit_mode: bool, // whether the Form should be rendered as editable (modify) or as read only (view)
}

#[derive(Serialize, Deserialize, FromForm, Debug)]
struct PublishedBlogEntriesViewContext {
    data: Vec<BlogEntry>,
    session: Option<UserSessionData>,
}

#[derive(Serialize, Deserialize, FromForm, Debug)]
pub struct BlogEditViewContext {
    data: BlogEntry,
    session: Option<UserSessionData>,
}
// -------------------------- A U T H E N T I C A T I O N -------------------------- //

#[get("/login")]
pub fn google_login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["profile"]).unwrap()
}

#[get("/google")]
pub async fn oauth_via_google(
    token: TokenResponse<GoogleUserInfo>,
    cookies: &CookieJar<'_>,
    database: &State<database::MongoDB>,
) -> Result<Redirect, Debug<HyperError>> {
    let user_id = String::from(authenticate(token).await?);

    if database.is_authorized(&user_id).await.unwrap() {
        cookies.add_private(Cookie::new("user_id",  user_id));
        Ok(Redirect::to("/auth/login_success"))
    } else {
        Ok(Redirect::to("/auth/login_failure"))
    }
}

#[get("/login_success")]
pub fn login_success() -> Template {
    Template::render("login_success", &Context::default())
}

#[get("/login_failure")]
pub fn login_failure() -> Template {
    Template::render("login_failure", &Context::default())
}

#[get("/logout")]
pub fn logout(jar: &CookieJar) -> Redirect {
    jar.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}
