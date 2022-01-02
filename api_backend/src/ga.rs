#![allow(non_snake_case)]

use crate::model;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct GAPath{
    pub fitness: f64,
    pub schedule: Vec<(f32, f32)>
}

impl GAPath{
    pub fn init(&mut self, hardConstraints: &Vec<(f32, f32)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32){
        self.schedule = hardConstraints.clone();
        let ListOfFreeTime = self.getListOfFreeTime(hardConstraints, EndOfCycle);
        let mut rng = thread_rng();
        for x in listOfRequestedEvents{
            let mut freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            while freeTime.1 - freeTime.0 < x.length{
                freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            }
            let freetime_length = freeTime.1 - freeTime.0;
            let multiple: i32 = rng.gen_range(0 .. (freetime_length/x.length) as i32);
            let start = freeTime.0 + multiple as  f32 * x.length;
            let sample_schedule: (f32, f32) = (start, start + x.length);
            self.schedule.push(sample_schedule);
        }
        self.schedule.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn getListOfFreeTime(&mut self, listOfEvents: &Vec<(f32, f32)>, EndOfCycle: f32) -> Vec<(f32, f32)>{
        let mut ListOfFreeTime = Vec::new();
        let mut lastTime: f32 = 0.0;
        for x in listOfEvents{
            ListOfFreeTime.push((lastTime, x.0));
            lastTime = x.1;
        }
        ListOfFreeTime.push((lastTime, EndOfCycle));
        return ListOfFreeTime;
    }

    pub fn new() -> Self{
        Self{
            fitness: 0.0,
            schedule: Vec::new()
        }
    }
}

