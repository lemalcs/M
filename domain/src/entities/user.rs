use chrono::prelude::*;

struct User
{
    id: i32,
    token: String,
    name: String,
    fullName: String,
    creation_date: DateTime<Utc>,
}