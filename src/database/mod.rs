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

use mongodb::{bson, Client, Cursor, Database, options::FindOptions};
use mongodb::bson::{doc, Document};
use mongodb::options::{ClientOptions};
use mongodb::results::{InsertOneResult};
use rocket::fairing::{AdHoc};
use rocket::futures::TryStreamExt;
use crate::model::BlogEntry;

#[derive(Debug)]
pub struct MongoDB {
    database: Database,
}

impl MongoDB {
    fn new(database: Database) -> Self {
        MongoDB {
            database
        }
    }

    pub async fn fetch_all_published_blogs(&self, filter_drafts: bool) -> mongodb::error::Result<Vec<BlogEntry>> {
        let collection = self.database.collection::<BlogEntry>("blogs");
        // Query the blogs in the collection which are published.
        let filter = match filter_drafts {
            true => Some(doc! { "status": "Published" }),
            false => None
        };
        let find_options = FindOptions::builder()
            .sort(doc! { "title": 1 })
            .build(); //TODO sort by date

        let mut cursor: Cursor<BlogEntry> = collection.find(filter, find_options).await?;

        let mut blogs: Vec<BlogEntry> = Vec::new();
        while let Some(blog) = cursor.try_next().await? {
            blogs.push(blog);
        }

        Ok(blogs)
    }

    pub async fn find_blog(&self, id: String) -> mongodb::error::Result<BlogEntry> {
        let collection = self.database.collection::<BlogEntry>("blogs");
        Ok(collection.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn delete_blog(&self, id: String) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<BlogEntry>("blogs");
       collection.delete_one(doc! { "_id": id }, None).await?;
       Ok(())
    }

    pub async fn update_blog(&self, blog: &mut BlogEntry) -> mongodb::error::Result<String> {
        let collection = self.database.collection::<BlogEntry>("blogs");
        let result = collection.replace_one(doc! { "_id":  &blog._id }, blog, None).await?;
        dbg!(result);
        Ok("ok".to_string())
    }

    pub async fn add_blog(&self, blog: &mut BlogEntry) -> mongodb::error::Result<String> {
        let collection = self.database.collection::<BlogEntry>("blogs");
        blog._id = bson::oid::ObjectId::new().to_string();
        let insert: InsertOneResult = collection.insert_one(blog, None).await?;
        Ok(insert.inserted_id.to_string())
    }

    pub async fn is_authorized(&self, user_id: &String) -> mongodb::error::Result<bool> {
        let authorization = self.database.collection::<Document>("authorization");
        Ok(authorization.count_documents(doc! { "_id": user_id }, None).await? == 1)
    }
}

 pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => {
                rocket.manage(MongoDB::new(database))
            }
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let mdb_uri = std::env::var("MDB_URL").or(Err("MDB_URL environment variable missing")).unwrap();
    let client_options = ClientOptions::parse(mdb_uri).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    Ok(client.database("rust_blog_engine"))
}
