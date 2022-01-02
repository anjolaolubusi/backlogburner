#![allow(non_snake_case)]

use actix_web::{post, Result};
use actix_web::web::{Json};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardConstraint{
    pub class: String,
    pub end: DateTime<Utc>,
    pub source: String,
    pub start: DateTime<Utc>,
    pub title: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData{
    pub listOfEvents: Vec<HardConstraint>,
    pub monday: DateTime<Utc>,
    pub newEvent: RequestedEvent
}

impl UserData{
    pub fn ConvertUserData(&self) -> Vec<ModelHardConstraint>{
        let mut newVec = Vec::<ModelHardConstraint>::new();
        for x in &self.listOfEvents{
            newVec.push(ModelHardConstraint::new(x, self.monday));
        }
        return newVec;
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestedEvent{
    pub class: String,
    pub length: f32,
    pub selectedDate: DateTime<Utc>,
    pub source: String,
    pub title: String    
}

#[derive(Clone, Copy, Debug)]
pub struct ModelHardConstraint{
    pub end: f32,
    pub start: f32,
}

impl ModelHardConstraint{
    pub fn new(hc: &HardConstraint, monday: DateTime<Utc>) -> Self{
        Self{
            end: (hc.end.timestamp() - monday.timestamp()) as f32/3600.0,
            start: (hc.start.timestamp() - monday.timestamp()) as f32/3600.0,
        }
    }
}

#[post("/model")]
async fn getNewSchedule(user_data: Json<UserData>) -> Result<String>{
    let model_data:Vec<ModelHardConstraint> = user_data.ConvertUserData();
    Ok(format!("{:?}", model_data))
    // Add Genetic Algorithm
    // Add ACO
}