#![allow(non_snake_case)]

use crate::model;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct GAPath{
    pub fitness: f32,
    pub schedule: Vec<(f32, f32)>,
    pub newEventsIndex: Vec<usize>
}

impl GAPath{
    pub fn init(&mut self, hardConstraints: &Vec<(f32, f32)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32){
        self.schedule = hardConstraints.clone();
        let ListOfFreeTime = self.getListOfFreeTime(hardConstraints, EndOfCycle);
        let mut rng = thread_rng();
        let mut listOfNewEvents = Vec::<(f32, f32)>::new();
        for x in listOfRequestedEvents{
            let mut freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            while freeTime.1 - freeTime.0 < x.length{
                freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            }
            let freetime_length = freeTime.1 - freeTime.0;
            let multiple: i32 = rng.gen_range(0 .. (freetime_length/x.length) as i32);
            let start = freeTime.0 + multiple as  f32 * x.length;
            let sample_schedule: (f32, f32) = (start, start + x.length);
            println!("{:?}", sample_schedule);
            self.schedule.push(sample_schedule);
            listOfNewEvents.push(sample_schedule);
        }
        self.schedule.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for x in listOfNewEvents{
            let index = self.schedule.iter().position(|&r| r == x).unwrap();
            self.newEventsIndex.push(index);
        }
        self.fitness = self.calculateFitness(EndOfCycle);
    }

    pub fn getListOfFreeTime(&mut self, listOfEvents: &Vec<(f32, f32)>, EndOfCycle: f32) -> Vec<(f32, f32)>{
        let mut ListOfFreeTime = Vec::new();
        let mut lastTime: f32 = 0.0;
        for x in listOfEvents{
            ListOfFreeTime.push((lastTime, x.0));
            lastTime = x.1;
        }
        ListOfFreeTime.push((lastTime, EndOfCycle));
        println!("lastTime: {0} EndOfCycle: {1}", lastTime, EndOfCycle);
        return ListOfFreeTime;
    }

    pub fn new() -> Self{
        Self{
            fitness: 0.0,
            schedule: Vec::new(),
            newEventsIndex: Vec::new()
        }
    }

    pub fn calculateFitness(&mut self, EndOfCycle: f32) -> f32{
        let mut minFitness: f32 = 999999999999999.0;
        let temp = self.schedule.clone();
        let ListOfFreeTime = self.getListOfFreeTime(&temp, EndOfCycle);
        println!("{:?}", ListOfFreeTime);
        for x in ListOfFreeTime{
            println!("({0}, {1})", x.1 - x.0, minFitness);
            if x.1 - x.0 < minFitness{
                minFitness = x.1 - x.0;
            }
        }
        return minFitness;
    }
}


