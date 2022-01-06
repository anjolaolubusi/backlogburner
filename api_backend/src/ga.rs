#![allow(non_snake_case)]

use crate::model;
use crate::ga;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct GAPath{
    pub fitness: f32,
    pub schedule: Vec<(f32, f32)>,
    pub newEventsIndex: Vec<usize>,
    pub EndOfCycle: f32
}

impl GAPath{
    pub fn init(&mut self, hardConstraints: &Vec<(f32, f32)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32){
        self.schedule = hardConstraints.clone();
        let ListOfFreeTime = ga::getListOfFreeTime(hardConstraints, EndOfCycle);
        let mut rng = thread_rng();
        let mut listOfNewEvents = Vec::<(f32, f32)>::new();
        self.EndOfCycle = EndOfCycle;
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
        self.fitness = ga::calculateFitness(self, EndOfCycle);
    }

    pub fn new() -> Self{
        Self{
            fitness: 0.0,
            schedule: Vec::new(),
            newEventsIndex: Vec::new(),
            EndOfCycle: 0.0
        }
    }
}

pub fn getListOfFreeTime(listOfEvents: &Vec<(f32, f32)>, EndOfCycle: f32) -> Vec<(f32, f32)>{
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

pub fn calculateFitness(genome: &GAPath, EndOfCycle: f32) -> f32{
    let mut minFitness: f32 = 999999999999999.0;
    //let temp = genome.schedule.clone();
    let ListOfFreeTime = ga::getListOfFreeTime(&genome.schedule, EndOfCycle);
    println!("{:?}", ListOfFreeTime);
    for x in ListOfFreeTime{
        println!("({0}, {1})", x.1 - x.0, minFitness);
        if x.1 - x.0 < minFitness{
            minFitness = x.1 - x.0;
        }
    }
    return minFitness;
}

pub fn crossover(genome1: &GAPath, genome2: &GAPath, model_data: &Vec<(f32, f32)>, ga_vec: &Vec<GAPath>) -> GAPath{
    let mut newGenome = GAPath::new();
    newGenome.EndOfCycle = genome1.EndOfCycle;
    let mut rng = rand::thread_rng();
    let mut newEventsList = Vec::<(f32, f32)>::new();
    let mut newSchedule = Vec::<(f32, f32)>::new();
    for i in 0..genome1.newEventsIndex.len(){
        let eventLength = genome1.schedule[genome1.newEventsIndex[i]].1 - genome1.schedule[genome1.newEventsIndex[i]].0;
        let mixScale: f32 = rng.gen();
        let currEventStart = genome1.schedule[genome1.newEventsIndex[i]].0 * mixScale + (1.0 - mixScale) * genome2.schedule[genome2.newEventsIndex[i]].0;
        let currEventEnd = currEventStart + eventLength;
        newEventsList.push((currEventStart, currEventEnd));
    }
    for x in model_data{
        newSchedule.push(*x);
    }
    for x in &newEventsList{
       newSchedule.push(*x);
    }
    newSchedule.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut newEventsIndex = Vec::<usize>::new();
    for x in newEventsList{
        let index = newSchedule.iter().position(|&r| r == x).unwrap();
        newEventsIndex.push(index);
    }

    newGenome.schedule = newSchedule.clone();
    newGenome.newEventsIndex = newEventsIndex.clone();
    newGenome.fitness = ga::calculateFitness(&newGenome, newGenome.EndOfCycle);
    return newGenome.clone();
}


pub fn mutate(genome: &mut GAPath, model_data: &Vec<(f32, f32)>){
    let mut newEventsList = Vec::<(f32, f32)>::new();
    for i in &genome.newEventsIndex{
        let mut eventTemp = ga::getNewTimings(&genome.schedule[*i]);
        while checkViolations(&eventTemp, model_data) {
            eventTemp = ga::getNewTimings(&eventTemp);
        }
        println!("{:?}", eventTemp);
        newEventsList.push(eventTemp);
    }
    
    let mut newSchedule = Vec::<(f32, f32)>::new();
    for x in model_data{
        newSchedule.push(*x);
    }
    for x in &newEventsList{
       newSchedule.push(*x);
    }
    newSchedule.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let mut newEventsIndex = Vec::<usize>::new();
    for x in newEventsList{
        let index = newSchedule.iter().position(|&r| r == x).unwrap();
        newEventsIndex.push(index);
    }

    genome.schedule = newSchedule.clone();
    genome.newEventsIndex = newEventsIndex.clone();
    genome.fitness = ga::calculateFitness(&genome, genome.EndOfCycle);
}

pub fn getNewTimings(origEvent: &(f32, f32)) -> (f32, f32){
    let mut rng = rand::thread_rng();
    let mut currEvent = *origEvent;
    let eventLength = currEvent.1 - currEvent.0;
    let mutateFactor = eventLength * rng.gen_range(-3..3) as f32;
    currEvent.0 = currEvent.0 + mutateFactor;
    currEvent.1 = currEvent.0 + eventLength;
    return currEvent;
}

pub fn checkViolations(newEvent: &(f32, f32), model_data: &Vec<(f32, f32)>) -> bool{
    let mut violated: bool = false;
    for hc in model_data{
        if (hc.0 <= newEvent.0 && newEvent.0 <= hc.1) || (hc.0 <= newEvent.1 && newEvent.1 <= hc.1) {
            violated = true;
            break;
        }
    }
    return violated;
}

/*
pub fn GeneticAlgorithm(){

}

pub fn select(){

}
*/
