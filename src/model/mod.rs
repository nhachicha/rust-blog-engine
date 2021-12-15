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
use rocket::form::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromForm, Deserialize, Clone)]
pub struct BlogEntry {
    // using String (instead of bson::oid::ObjectId) so it can easily be used as
    // a FromFormField without creating a custom transformer like (https://github.com/SergioBenitez/Rocket/issues/602)
    pub _id: String,
    #[field(validate = len(2..))]
    pub title: String,
    #[field(validate = len(10..))]
    pub content: String,
    #[field(validate = len(2..))]
    pub author: String,
    #[field(default = "Today")]
    pub last_edit_date: String,
    pub status: Status,
}

#[derive(Debug, FromFormField, Serialize, Deserialize, Clone)]
pub enum Status {
    Draft,
    Published,
}
