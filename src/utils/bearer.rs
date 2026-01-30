use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde_json::{json, Value};
use tracing::info;

use crate::models::initialization::{Entity, Column};

pub async fn bearer_resolver(
    token: &str,
    db: &DatabaseConnection,
) -> Result<Value, String> {
    info!("MY TOKEN: {:?}",token);
    match Entity::find()
        .filter(Column::ApiKey.eq(token))
        .one(db)
        .await
    {
        Ok(Some(user)) => Ok(json!(user)),
        Ok(None) => Err("Invalid API key".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

