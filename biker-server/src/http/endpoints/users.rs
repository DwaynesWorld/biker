use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};

use crate::http::{self, AppContext, Result};
use crate::protos::api::v1::users::{CreateUserRequest, CreateUserResponse, User, UserStatus};
use crate::protos::types::PaymentMethod;

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
    .route("/api/v1/users", post(create_user))
    .route("/api/v1/users/:id", get(get_user))
}

async fn create_user(c: State<AppContext>, Json(r): Json<CreateUserRequest>) -> Result<Json<CreateUserResponse>> {
    info!("creating new user: {}", r.name);

    if let Err(err) = r.validate() {
        return Err(err);
    }

    let ur: UserRow = r.into();
    let id = ur.id.clone();
    
    sqlx::query(
        "INSERT INTO users (
            id,
            name,
            nickname,
            email,
            email_verified,
            phone,
            picture_uri,
            status,
            default_payment_method,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(&ur.id)
    .bind(&ur.name)
    .bind(&ur.nickname)
    .bind(&ur.email)
    .bind(&ur.email_verified)
    .bind(&ur.phone)
    .bind(&ur.picture_uri)
    .bind(&ur.status)
    .bind(&ur.default_payment_method)
    .bind(&ur.created_at)
    .bind(&ur.updated_at)
    .execute(&c.db)
    .await?;

    Ok(Json(CreateUserResponse { id }))
}

async fn get_user(c: State<AppContext>, Path(user_id): Path<String>) -> Result<Json<User>> {
    info!("fetching user with id: {}", user_id);
    
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT
            id,
            name,
            nickname,
            email,
            email_verified,
            phone,
            picture_uri,
            status,
            default_payment_method,
            created_at,
            updated_at
        FROM Users
        WHERE id = $1"
    )
    .bind(&user_id)
    .fetch_one(&c.db)
    .await?
    .into();

    Ok(Json(user))
}

impl CreateUserRequest {
    #[rustfmt::skip]
    fn validate(&self) -> Result<()> {
        if self.name.is_empty() { 
            return Err(http::Error::unprocessable_entity([("name", "missing name")]));
        }

        if self.name.is_empty() { 
            return Err(http::Error::unprocessable_entity([("name", "missing name")]));
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct UserRow {
    pub id: String,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub email_verified: bool,
    pub phone: String,
    pub picture_uri: String,
    pub status: String,
    pub default_payment_method: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<UserRow> for CreateUserRequest {
    fn into(self: CreateUserRequest) -> UserRow {
        let now = chrono::offset::Utc::now();
        let status = self.status().as_str_name().to_string();
        let default_payment_method = self.default_payment_method().as_str_name().to_string();

        UserRow {
            id: self.id,
            name: self.name,
            nickname: self.nickname,
            email: self.email,
            email_verified: self.email_verified,
            phone: self.phone,
            picture_uri: self.picture_uri,
            status,
            default_payment_method,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        Self {
            id: r.id.into(),
            name: r.name.into(),
            nickname: r.nickname.into(),
            email: r.email.into(),
            email_verified: r.email_verified,
            phone: r.phone.into(),
            picture_uri: r.picture_uri.into(),
            status: UserStatus::from_str_name(&r.status).unwrap() as i32,
            default_payment_method: PaymentMethod::from_str_name(&r.default_payment_method).unwrap() as i32,
            created_at: r.created_at.timestamp(),
            updated_at: r.updated_at.timestamp(),
        }
    }
}