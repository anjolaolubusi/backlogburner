#![allow(non_snake_case)]

use actix_web::{post, Result};
use actix_web::web::{Json};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::ga;

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
    pub EndOfCycle: DateTime<Utc>,
    pub newEvent: Vec<RequestedEvent>
}

impl UserData{
    pub fn ConvertUserData(&self) -> Vec<(f32, f32)>{
        let mut newVec = Vec::<(f32, f32)>::new();
        for x in &self.listOfEvents{

            newVec.push((
                    (x.start.timestamp() - self.monday.timestamp()) as f32/3600.0,
                    (x.end.timestamp() - self.monday.timestamp()) as f32/3600.0
            ));
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

#[post("/model")]
async fn getNewSchedule(user_data: Json<UserData>) -> Result<String>{
    let model_data:Vec<(f32, f32)> = user_data.ConvertUserData();
    let endValue: f32 = (user_data.EndOfCycle.timestamp() - user_data.monday.timestamp()) as f32/3600.0;
    let mut chromosone: ga::GAPath = ga::GAPath::new();
    chromosone.init(&model_data, &user_data.newEvent, endValue);

    println!("{:?}", chromosone);
    println!("New Event");
    for i in &chromosone.newEventsIndex{
        println!("{:?}", chromosone.schedule[*i]);
    }
    println!("New Event after Mutation");
    ga::mutate(&mut chromosone, &model_data);
    
    println!("---------------------");
    Ok(format!("{:?}", model_data))
    // Add Genetic Algorithm
    // Add ACO
}