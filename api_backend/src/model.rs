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

pub fn ConvertToScheduleData(newEventModelData: &Vec<(f32, f32)>, userData: &UserData) -> Vec<HardConstraint>{
    let mut newEvents = Vec::<HardConstraint>::new();
    for i in 0..userData.newEvent.len(){
        let mut new_hc = HardConstraint::new();
        new_hc.start = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(
                ((newEventModelData[i].0 * 3600.0) as f32 + (userData.monday.timestamp() as f32)) as i64, 0 as u32)
                , Utc);
        new_hc.end = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(
                ((newEventModelData[i].1 * 3600.0) as f32 + (userData.monday.timestamp() as f32)) as i64, 0 as u32)
                , Utc);
        new_hc.class = "hc".to_string();
        new_hc.title = userData.newEvent[i].title.clone();
        new_hc.source = "A".to_string();
        newEvents.push(new_hc);
    }
    return newEvents;
}

#[post("/model")]
async fn getNewSchedule(user_data: Json<UserData>) -> HttpResponse{
    let model_data:Vec<(f32, f32)> = user_data.ConvertUserData();
    let endValue: f32 = (user_data.EndOfCycle.timestamp() - user_data.monday.timestamp()) as f32/3600.0;
    let freeIntervals = ga::getListOfFreeTime(&model_data, endValue);
    /*let pool = ga::run(5, &model_data, &user_data.newEvent, endValue);
    let mut selectedSolution = Vec::<(f32, f32)>::new();
    for i in &pool[0].newEventsIndex{
        selectedSolution.push(pool[0].schedule[*i])
    }
    let newEvents = ConvertToScheduleData(&selectedSolution, &user_data);
    println!("{:?}", newEvents);
    println!("---------------------");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(newEvents)*/
    let mut acoGraph : aco::Graph = aco::Graph{
        nodes: Vec::<Vec<aco::Node>>::new()
    };
    acoGraph.init(&user_data.newEvent, &freeIntervals);
    println!("{:?}", &acoGraph);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(model_data)
}