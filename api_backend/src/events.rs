use actix_web::{web, get, post, HttpResponse, Responder, Result};
use actix_web::web::{Json, Path};
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
pub struct Events{
    pub listOfEvents: Vec<HardConstraint>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEvent{
    pub class: String,
    pub length: f32,
    pub selected_date: DateTime<Utc>,
    pub source: String,
    pub title: String    
}

#[post("/model")]
async fn model(req_body: web::Json<Events>) -> Result<String>{
    println!("{:?}", req_body.listOfEvents);
    Ok(format!("{:?}", req_body))
}