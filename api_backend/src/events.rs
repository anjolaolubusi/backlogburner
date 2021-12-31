use actix_web::{post, Result};
use actix_web::web::{Json};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HardConstraint{
    pub class: String,
    pub end: DateTime<Utc>,
    pub source: String,
    pub start: DateTime<Utc>,
    pub title: String
}

impl HardConstraint {
    pub fn new(class: String, end: DateTime<Utc>, source: String, start: DateTime<Utc>, title: String) -> Self{
        Self{
            class,
            end,
            source,
            start,
            title
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData{
    pub listOfEvents: Vec<HardConstraint>,
    pub monday: DateTime<Utc>,
    pub newEvent: RequestedEvent
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedEvent{
    pub class: String,
    pub length: f32,
    pub selectedDate: DateTime<Utc>,
    pub source: String,
    pub title: String    
}

impl RequestedEvent{
    pub fn new(class: String, length: f32, selectedDate: DateTime<Utc>, source: String, title: String) -> Self{
        Self{
            class,
            length,
            selectedDate,
            source,
            title
        }
    }
}

#[post("/model")]
async fn model(user_data: Json<UserData>) -> Result<String>{
    println!("{:?}", user_data);
    Ok(format!("{:?}", user_data))
    // Add Genetic Algorithm
    // Add ACO
}