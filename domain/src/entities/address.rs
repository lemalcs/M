use chrono::prelude::*;

struct Address
{
    id: i4,
    id_user: i32,
    endpoint_uri: String,
    register_date: DateTime<Utc>,
}