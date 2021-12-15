A demo blog post engine written in Rust, using [Rocket](https://rocket.rs/) and [MongoDB](mongodb.com)

# Quick Start

- Setup a new MongoDB cluster https://cloud.mongodb.com/ create a new database and obtain the rust connection string (under Database/Connect)
  example (`mongodb+srv://<user>:<password>@XXXX.mongodb.net/myFirstDatabase?retryWrites=true&w=majority`)

- Create a database `rust_blog_engine`

- Add a collection `authorization` containing authorized users id (Google user id)
```Javascript
{
    "_id": "116710526826489061000",
    "email": "nabil.hachicha@gmail.com",
    "name": "Nabil Hachicha"
}
```

- Add a collection `blogs` which will persist our blog posts entries.
  <img src="./images/MongoDB_Collections.png" width="800">


- Setup a Google OAuth2 API and add the `client_id` and `client_secret` inside the [Rocket.toml](/Rocket.toml) file.

- Start the engine using `cargo run` and providing the MongoDB connection string
```Shell
MDB_URL="mongodb+srv://<user>:<password>@XXXX.mongodb.net/myFirstDatabase?retryWrites=true&w=majority" cargo run 
```

# Preview

### Home
<img src="./images/Home.png" width="800">

### Login
<img src="./images/OAuth_login.png" width="800">

### Admin
<img src="./images/Admin_home.png" width="800">

### New Blog
<img src="./images/New_Blog_Validation.png" width="800">

### Edit Blog
<img src="./images/Edit_Blog.png" width="800">