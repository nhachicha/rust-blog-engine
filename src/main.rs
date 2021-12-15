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
#[macro_use]
extern crate rocket;

mod model;
mod database;
mod utils;
mod routes;
mod auth;

use std::error::Error;
use routes::*;
use auth::GoogleUserInfo;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use rocket_oauth2::OAuth2;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>>{
    rocket::build()
        .attach(database::init().await) // connect to the database
        .attach(OAuth2::<GoogleUserInfo>::fairing("google")) // setup OAuth
        .attach(Template::fairing()) // setup Tera templates

        .mount("/", FileServer::from(relative!("/static"))) // serving CSS
        .mount("/", routes![blog_entries]) // public pages (blogposts)
        .mount("/auth", routes![google_login, oauth_via_google, login_success, login_failure, logout]) // authentication
        .mount("/admin", routes![admin_blog_entries, new_blog, new_blog_post, edit_blog, delete_blog]) // administration
        .launch()
        .await?;
    Ok(())
}
