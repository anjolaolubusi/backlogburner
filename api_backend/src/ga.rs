#![allow(non_snake_case)]

use crate::model;
use crate::ga;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct GAPath{
    pub fitness: f32,
    pub schedule: Vec<(i128, i128)>,
    pub newEventsIndex: Vec<usize>,
    pub EndOfCycle: f32
}

impl GAPath{
    pub fn init(&mut self, hardConstraints: &Vec<(i128, i128)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32){
        self.schedule = hardConstraints.clone();
        let ListOfFreeTime = ga::getListOfFreeTime(hardConstraints, EndOfCycle);
        let mut rng = thread_rng();
        let mut listOfNewEvents = Vec::<(i128, i128)>::new();
        self.EndOfCycle = EndOfCycle;
        for x in listOfRequestedEvents{
            let mut freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            while freeTime.1 - freeTime.0 < x.length as i128{
                freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            }
            let mut freetime_length = freeTime.1 - freeTime.0;
            let mut multiple: i128 = rng.gen_range(0 .. (freetime_length as f32/(x.length * 12.0)) as i128);
            let mut start = freeTime.0 + (multiple * (x.length * 12.0)  as i128);
            let mut sample_schedule: (i128, i128) = (start, start + (x.length * 12.0) as i128);
            while ga::checkViolations(&sample_schedule, hardConstraints){
                freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
                while freeTime.1 - freeTime.0 < x.length as i128{
                    freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
                }
                freetime_length = freeTime.1 - freeTime.0;
                multiple = rng.gen_range(0 .. (freetime_length as f32/(x.length * 12.0)) as i128);
                start = freeTime.0 + (multiple * (x.length * 12.0)  as i128);
                sample_schedule = (start, start + (x.length * 12.0) as i128);
            }
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

    pub fn getNewEventsList(&self) -> Vec<(i128, i128)>{
        let mut newEventList: Vec<(i128, i128)> = Vec::new();
        for index in &self.newEventsIndex{
            newEventList.push(self.schedule[*index]);
        }
        return newEventList;
    }
}

impl PartialEq for GAPath{
    fn eq(&self, other: &GAPath) -> bool{
        (&self.schedule, &self.newEventsIndex, &self.EndOfCycle) == (&other.schedule, &other.newEventsIndex, &other.EndOfCycle)
    }
}

pub fn getListOfFreeTime(listOfEvents: &Vec<(i128, i128)>, EndOfCycle: f32) -> Vec<(i128, i128)>{
    let mut ListOfFreeTime = Vec::new();
    let mut lastTime: i128 = 0;
    for x in listOfEvents{
        ListOfFreeTime.push((lastTime, x.0));
        lastTime = x.1;
    }
    ListOfFreeTime.push((lastTime, EndOfCycle as i128));
    return ListOfFreeTime;
}

pub fn calculateFitness(genome: &GAPath, EndOfCycle: f32) -> f32{
    let mut minFitness: f32 = 999999999999999.0;
    //let temp = genome.schedule.clone();
    let ListOfFreeTime = ga::getListOfFreeTime(&genome.schedule, EndOfCycle);
    for x in ListOfFreeTime{
        if (x.1 - x.0).abs() as f32 - minFitness < 0.0{
            minFitness = (x.1 - x.0).abs() as f32;
        }
    }
    return minFitness;
}

pub fn crossover(genome1: &GAPath, genome2: &GAPath, model_data: &Vec<(i128, i128)>) -> GAPath{
    //TODO: Fix bug where program is stuck finding a new corssover point
    //TODO: Change how crossover is handled

    let mut newGenome = GAPath::new();
    newGenome.EndOfCycle = genome1.EndOfCycle;
    let mut rng = rand::thread_rng();
    let mut newEventsList = Vec::<(i128, i128)>::new();
    let mut newSchedule = Vec::<(i128, i128)>::new();
    for i in 0..genome1.newEventsIndex.len(){
        let eventLength = genome1.schedule[genome1.newEventsIndex[i]].1 - genome1.schedule[genome1.newEventsIndex[i]].0;
        let mut mixScale: i128 = rng.gen_range(0..2);
        let mut currEventStart = genome1.schedule[genome1.newEventsIndex[i]].0  * mixScale + (1 - mixScale) * genome2.schedule[genome2.newEventsIndex[i]].0;
        let mut currEventEnd = currEventStart + eventLength;
        let mut nEvent = (currEventStart, currEventEnd);
        while checkViolations(&nEvent, model_data) {
            mixScale = rng.gen_range(0..2);
            currEventStart = genome1.schedule[genome1.newEventsIndex[i]].0  * mixScale + (1 - mixScale) * genome2.schedule[genome2.newEventsIndex[i]].0;
            currEventEnd = currEventStart + eventLength;
            nEvent = (currEventStart, currEventEnd);
        }
        newEventsList.push(nEvent);
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

pub fn mutate(genome: &mut GAPath, model_data: &Vec<(i128, i128)>){
    let mut newEventsList = Vec::<(i128, i128)>::new();
    for i in &genome.newEventsIndex{
        let mut eventTemp = ga::getNewTimings(&genome.schedule[*i]);
        while checkViolations(&eventTemp, model_data) {
            eventTemp = ga::getNewTimings(&eventTemp);
        }
        newEventsList.push(eventTemp);
    }
    
    let mut newSchedule = Vec::<(i128, i128)>::new();
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

pub fn getNewTimings(origEvent: &(i128, i128)) -> (i128, i128){
    let mut rng = rand::thread_rng();
    let mut currEvent = *origEvent;
    let eventLength = currEvent.1 - currEvent.0;
    let mutateFactor = eventLength * rng.gen_range(-24.0 .. 24.0) as i128;
    currEvent.0 = currEvent.0 + mutateFactor;
    currEvent.1 = currEvent.0 + eventLength;
    return currEvent;
}

pub fn checkViolations(newEvent: &(i128, i128), model_data: &Vec<(i128, i128)>) -> bool{
    let mut violated: bool = false;
    //Add code to allow border cases
    for hc in model_data{
        if (hc.0 <= newEvent.0 && newEvent.1 <= hc.1) || (newEvent.0 <= hc.0  && hc.0 <= newEvent.1 && newEvent.1 <= hc.1) || (hc.0 <= newEvent.0 && newEvent.0 <= hc.1 && hc.1 <= newEvent.1) {
            if hc.0 <= newEvent.0 && newEvent.1 <= hc.1{
                println!("Case 2");
            }
            if newEvent.0 <= hc.0  && hc.0 <= newEvent.1 && newEvent.1 <= hc.1{
                println!("Case 4");
            }
            if hc.0 <= newEvent.0 && newEvent.0 <= hc.1 && hc.1 <= newEvent.1{
                println!("Case 5");
            }
            violated = true;
            break;
        }
    }
    return violated;
}

pub fn calculateAvgFitness(gaPool: &Vec<GAPath>) -> f32{
    let mut fitnessSum = 0.0;
    for x in gaPool{
        fitnessSum += ga::calculateFitness(x, x.EndOfCycle);
    }
    return fitnessSum/(gaPool.len() as f32);
}

pub fn selectParent(gaPool: &Vec<GAPath>) -> (&GAPath, &GAPath){
    let mut rng = rand::thread_rng();
    let mut parent1: Option<&GAPath> = None::<&GAPath>;
    while parent1 == None {
        for x in gaPool{
            if rng.gen::<f32>() <= 0.3{
                parent1 = Some(x);
            }
        }
    }
    let mut parent2: Option<&GAPath> = None::<&GAPath>;
    while parent2 == None {
        for x in gaPool{
            if rng.gen::<f32>() <= 0.3 && x != parent1.unwrap(){
                parent2 = Some(x);
            }
        }
    }
    return (parent1.unwrap(), parent2.unwrap());
}

pub fn run(population: i32, hardConstraints: &Vec<(i128, i128)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32) -> Vec::<GAPath>{
    let mut gaPool = Vec::<GAPath>::new();
    let mut fitnessIterations = (0.0, 0.0);
    for _ in 0..population{
        let mut newGenome = ga::GAPath::new();
        newGenome.init(hardConstraints, listOfRequestedEvents, EndOfCycle);
        let _ = &gaPool.push(newGenome);
    }
    fitnessIterations.0 = ga::calculateAvgFitness(&gaPool);
    while fitnessIterations.1/fitnessIterations.0 < 0.3{
            //Select Parents for Crossover
            let mut newPool = Vec::<GAPath>::new();
            while (newPool.len() as i32) < population {
                let parents: (&GAPath, &GAPath) = ga::selectParent(&gaPool);
                let mut newGenome = ga::crossover(parents.0, parents.1, hardConstraints);
                ga::mutate(&mut newGenome, hardConstraints);
                newPool.push(newGenome);
            }
            fitnessIterations.1 = fitnessIterations.0;
            fitnessIterations.0 = ga::calculateAvgFitness(&gaPool);
            gaPool = newPool;
    }
    return gaPool;
}