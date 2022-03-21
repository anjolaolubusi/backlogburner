#![allow(non_snake_case)]

//Library Imports
//Actic Web handles the http post calls
//create::ga is our genetic algorithm implementation
//crate::aco is out ant colony optimization implementation
use actix_web::{post, HttpResponse};
use actix_web::web::{Json};
use chrono::{DateTime, Utc, NaiveDateTime, Local};
use serde::{Serialize, Deserialize};
use crate::ga;
use crate::aco;
use rsgenetic::pheno::*;

//Abstracted Hard Constraint
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardConstraint{
    pub class: String,
    pub end: DateTime<Utc>,
    pub source: String,
    pub start: DateTime<Utc>,
    pub title: String
}

//Allows creates new HardConstaint object
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

//Object that represents the JSON data sent from the VueJS Application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData{
    pub listOfEvents: Vec<HardConstraint>,
    pub monday: DateTime<Utc>,
    pub EndOfCycle: DateTime<Utc>,
    pub newEvent: Vec<RequestedEvent>
}

//Coverts the schedule data into a format that the model can use
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

//Represents the requested hobby to schedule
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestedEvent{
    pub class: String,
    pub length: f32,
    pub selectedDate: DateTime<Utc>,
    pub source: String,
    pub title: String,
    pub recurType: String    
}

//Represents the schedule data as the model sees it
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleData{
    pub start: i128,
    pub end: i128,
    pub class: String,
    pub title: String,
    pub source: String
}

//Allows creates new ScheduleData object
impl ScheduleData{
    pub fn new() -> Self{
        Self{
        class: "".to_string(),
        end: 0,
        source: "".to_string(),
        start: 0,
        title: "".to_string()
        }
    }
}

//Converts the event from the model format to a more human reable format
//Pre-conditions: None
//Post-conditions: Returns a object represents the scheduled hobby
pub fn ConvertToScheduleData(newEventModelData: &Vec<Vec<(i128, i128)>>, userData: &UserData) -> Vec<HardConstraint>{
    println!("newEventModelData {:?}", newEventModelData);
    let mut scheduleData = Vec::<HardConstraint>::new();
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
            new_hc.class = "sc".to_string();
            new_hc.title = userData.newEvent[i].title.clone();
            new_hc.source = "A".to_string();
            scheduleData.push(new_hc);
        }
        //scheduleData.push(eventList);
    }
    return scheduleData;
}

//Sends out the schedule hobby
//Pre-condition: The user has sent their data to the api endpoint.
//Post-condition: The scheduled hobby is sent to the user in a json form
#[post("/model")]
async fn getNewSchedule(user_data: Json<UserData>) -> HttpResponse{
    println!("{:?}", user_data);
    let mut model_data:Vec<(i128, i128)> = user_data.ConvertUserData();
    model_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let endValue: i128 = (user_data.EndOfCycle.timestamp() - user_data.monday.timestamp()) as i128/300;
    let freeIntervals = ga::getListOfFreeTime(&model_data, endValue as f32);
    let pool = ga::run(100, &model_data, &user_data.newEvent, endValue as f32);
    println!("Starting Events: ");
    for x in &pool.0{
        println!("");
        for index in &x.newEventsIndex{
            print!("{:?} ", x.schedule[*index]);
        }
        print!(" Fitness: {}", x.fitness() );
    }
    let selectedSolution = aco::run(27, &user_data.newEvent, &freeIntervals, &pool.0, &model_data, pool.1);
    let newEvents = ConvertToScheduleData(&selectedSolution, &user_data);
    if cfg!(debug_assertions){
        println!("New Events Count: {:?}", newEvents.len());
        println!("---------------------");
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(newEvents)
}