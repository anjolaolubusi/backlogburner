#![allow(non_snake_case)]

use actix_web::{post, HttpResponse};
use actix_web::web::{Json};
use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::ga;
use crate::aco;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardConstraint{
    pub class: String,
    pub end: DateTime<Utc>,
    pub source: String,
    pub start: DateTime<Utc>,
    pub title: String
}

impl HardConstraint{
    pub fn new() -> Self{
        Self{
        class: "".to_string(),
        end: Utc::now(),
        source: "".to_string(),
        start: Utc::now(),
        title: "".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData{
    pub listOfEvents: Vec<HardConstraint>,
    pub monday: DateTime<Utc>,
    pub EndOfCycle: DateTime<Utc>,
    pub newEvent: Vec<RequestedEvent>
}

impl UserData{
    pub fn ConvertUserData(&self) -> Vec<(i128, i128)>{
        let mut newVec = Vec::<(i128, i128)>::new();
        for x in &self.listOfEvents{

            newVec.push((
                    (x.start.timestamp() - self.monday.timestamp()) as i128/300,
                    (x.end.timestamp() - self.monday.timestamp()) as i128/300
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

pub fn ConvertToScheduleData(newEventModelData: &Vec<Vec<(i128, i128)>>, userData: &UserData) -> Vec<Vec<HardConstraint>>{
    let mut scheduleData = Vec::<Vec<HardConstraint>>::new();
    for newEvents in newEventModelData{
        let mut eventList = Vec::<HardConstraint>::new();
        for i in 0..userData.newEvent.len(){
            let mut new_hc = HardConstraint::new();
            new_hc.start = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(
                    ((newEvents[i].0 * 300) as f32 + (userData.monday.timestamp() as f32)) as i64, 0 as u32)
                    , Utc);
            new_hc.end = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(
                    ((newEvents[i].1 * 300) as f32 + (userData.monday.timestamp() as f32)) as i64, 0 as u32)
                    , Utc);
            new_hc.class = "hc".to_string();
            new_hc.title = userData.newEvent[i].title.clone();
            new_hc.source = "A".to_string();
            eventList.push(new_hc);
        }
        scheduleData.push(eventList);
    }
    return scheduleData;
}

#[post("/model")]
async fn getNewSchedule(user_data: Json<UserData>) -> HttpResponse{
    let model_data:Vec<(i128, i128)> = user_data.ConvertUserData();
    let endValue: i128 = (user_data.EndOfCycle.timestamp() - user_data.monday.timestamp()) as i128/300;
    let freeIntervals = ga::getListOfFreeTime(&model_data, endValue as f32);
    let pool = ga::run(100, &model_data, &user_data.newEvent, endValue as f32);
    let selectedSolution = aco::run(100, &user_data.newEvent, &freeIntervals, &pool, &model_data);
    let newEvents = ConvertToScheduleData(&selectedSolution, &user_data);
    if cfg!(debug_assertions){
        println!("NEW EVENT: {:?}", newEvents);
        println!("---------------------");
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(newEvents)
}