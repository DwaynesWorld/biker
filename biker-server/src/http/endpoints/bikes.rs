use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::http::{self, AppContext, Result};
use crate::protos::api::v1::bikes::CreateBikeRequest;
use crate::protos::api::v1::bikes::CreateBikeResponse;

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
        .route("/api/v1/bikes", post(create_bike))
        .route("/api/v1/bikes/nearby", get(find_nearby_bikes))
}

async fn create_bike(
    c: State<AppContext>,
    Json(r): Json<CreateBikeRequest>,
) -> Result<Json<CreateBikeResponse>> {
    info!("creating new bike: {}", r.code);

    if let Err(err) = r.validate() {
        return Err(err);
    }

    let tr: BikeRow = r.into();
    let id = tr.id.clone();

    sqlx::query(
        "INSERT INTO bikes (
            id, code, model, location, created_at, updated_at
        ) VALUES ($1, $2, $3,  ST_Point($4, $5), $6, $7)",
    )
    .bind(&tr.id)
    .bind(&tr.code)
    .bind(&tr.model)
    .bind(&tr.longitude)
    .bind(&tr.latitude)
    .bind(&tr.created_at)
    .bind(&tr.updated_at)
    .execute(&c.db)
    .await?;

    Ok(Json(CreateBikeResponse { id: id.to_string() }))
}

async fn find_nearby_bikes(State(_): State<AppContext>) -> Result<()> {
    Ok(())
}

impl CreateBikeRequest {
    #[rustfmt::skip]
    fn validate(&self) -> Result<()> {
        if self.location.is_none() { 
            return Err(http::Error::unprocessable_entity([("location", "missing location params")]));
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct BikeRow {
    pub id: Uuid,
    pub code: String,
    pub model: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<BikeRow> for CreateBikeRequest {
    fn into(self) -> BikeRow {
        let now = chrono::offset::Utc::now();
        let location = self.location.unwrap();

        BikeRow {
            id: Uuid::now_v7(),
            code: self.code,
            model: self.model,
            latitude: location.latitude,
            longitude: location.longitude,
            created_at: now,
            updated_at: now,
        }
    }
}
