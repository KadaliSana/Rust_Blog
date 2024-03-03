#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::thread;

// User model
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

//Schedule post model
#[derive(Debug, Serialize, Deserialize)]
struct SchedulePostRequest {
    title: String,
    content: String,
    publishing_date_nd_time: String,
}

//JWT Token generation model
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Blog post model
#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlogPost {
    title: String,
    content: String,
    publishing_date_nd_time: Option<String>,
}

// Global state to store user sessions (in-memory for simplicity, use a database in a real app)
static mut USERS: Vec<User> = Vec::new();
static mut POSTS: Vec<BlogPost> = Vec::new();

// Routes
#[post("/register", data = "<user>")]
fn register(user: Json<User>) -> &'static str {
    unsafe {
        USERS.push(user.into_inner());
    }
    "User registered successfully!"
}

#[post("/login", data = "<user>")]
fn login(user: Json<User>) -> Result<String, String> {
    let user = user.into_inner();

    // Check if the user is valid (you might want to replace this with your authentication logic)
    if is_valid_user(&user) {
        // Generate JWT
        match generate_jwt(&user.username) {
            Ok(token) => Ok(token),
            Err(err) => Err(format!("JWT Generation failed: {}", err)),
        }
    } else {
        Err("Invalid username or password".to_string())
    }
}

fn is_valid_user(user: &User) -> bool {
    // Replace this with your actual authentication logic (e.g., querying a database)
    unsafe {
        USERS.iter().any(|u| u.username == user.username && u.password == user.password)
    }
}

fn generate_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration_time = Utc::now() + Duration::seconds(3600); // 1 hour expiration time
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration_time.timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret("your_secret_key".as_bytes());
    encode(&header, &claims, &encoding_key)
}

#[post("/schedule_post", data = "<post>")]
fn schedule_post(post: Json<SchedulePostRequest>) -> &'static str {
    let post = post.into_inner();
    let new_post = BlogPost {
        title: post.title,
        content: post.content,
        publishing_date_nd_time: Some(post.publishing_date_nd_time),
    };
    unsafe {
        POSTS.push(new_post);
    }
    "Post scheduled successfully!"
}

fn publish_scheduled_posts() {
    loop {
        // Check for scheduled posts and publish them
        let current_time = Utc::now().to_rfc3339();
        unsafe {
            for (index, post) in POSTS.iter_mut().enumerate() {
                if let Some(publishing_date_nd_time) = &post.publishing_date_nd_time {
                    if publishing_date_nd_time <= &current_time {
                        println!("Publishing scheduled post: {}", post.title);
                        post.publishing_date_nd_time = None; // Marking the post as published
                    }
                }
            }
        }

        // Sleep for a while before checking again
        std::thread::sleep(std::time::Duration::from_secs(60)); // Sleep for 60 seconds
    }
}

//Post read route
#[get("/posts")]
fn get_posts() -> Json<Vec<BlogPost>> {
    unsafe {
        Json(POSTS.clone())
    }
}

//Post create route
#[post("/posts", data = "<post>")]
fn create_post(post: Json<BlogPost>) -> &'static str {
    unsafe {
        POSTS.push(post.into_inner());
    }
    "Post created successfully!"
}

//Update post route
#[put("/posts/<index>", data = "<post>")]
fn update_post(index: usize, post: Json<BlogPost>) -> &'static str {
    unsafe {
        if let Some(existing_post) = POSTS.get_mut(index) {
            *existing_post = post.into_inner();
            "Post updated successfully!"
        } else {
            "Post not found"
        }
    }
}

//Delete route
#[delete("/posts/<index>")]
fn delete_post(index: usize) -> &'static str {
    unsafe {
        if POSTS.len() > index {
            POSTS.remove(index);
            "Post deleted successfully!"
        } else {
            "Post not found"
        }
    }
}

#[rocket::main]
async fn main() {
    // Start the thread for publishing scheduled posts
    thread::spawn(|| {
        publish_scheduled_posts();
    });

    // Launch the Rocket application
    rocket::build()
        .mount("/", routes![
            register, login, schedule_post,
            get_posts, create_post, update_post, delete_post,
        ])
        .launch()
        .await;
}