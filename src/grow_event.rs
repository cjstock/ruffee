use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(FromRow, Debug)]
pub struct GrowEvent {
    pub grow_event_id: Uuid,
    pub timestamp: Option<DateTime<Utc>>,
    pub farm_id: Option<Uuid>,
    pub farmer_id: Option<Uuid>,
    pub variety_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data for creating a new grow event.
#[derive(Debug)]
pub struct NewGrowEvent {
    pub timestamp: Option<DateTime<Utc>>,
    pub farm_id: Option<Uuid>,
    pub farmer_id: Option<Uuid>,
    pub variety_id: Option<Uuid>,
}
