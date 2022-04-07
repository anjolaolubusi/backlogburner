#![allow(non_snake_case)]

//rand::{thread_rng, Rng}, rand_chacham rand_core imports a random number generator
//create::ga is our genetic algorithm implementation
//crate::aco is out ant colony optimization implementation
//rsgenetic::pheno::* imports our Genetic Algorithm genomes

use crate::model;
use crate::ga;
use rand::{seq::SliceRandom, thread_rng, Rng};
use rsgenetic::pheno::*;
use std::cmp::Ordering;
use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;

/**
 * Fitness value as a struct
 * value - 128 bit integer that represents the fitness value
 */
#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub struct GAFitness{
    value: i128
}

impl Fitness for GAFitness{
    /**
     * Returns a Fitness value of zero
     * Pre-condition: None
     * Post-condition: Returns a GAFitness struct with a value property of 0
     */
    fn zero() -> GAFitness{
        GAFitness{ value: 0}
    }

    /**
     * Returns the absoulte difference between two fitness values
     * Parameters:
     *  self - Pointer to the GAFitness object
     *  other - Pointer to another GAFitness object
     * Pre-condition: None
     * Post-condition: The absoulte difference between two fitness values is returned
     */
    fn abs_diff(&self, other: &Self) -> GAFitness{
        GAFitness{
            value: (self.value - other.value).abs()
        }
    }
}

//Genome class
/**
 * Genome class
 * schedule - Vector of 128 bit integer tuples. This represents the user's schedule within the GACO algorithm
 * newEventsIndex - Vector of pointer-sized unsigned integer that represents a indicies of each scheduled hobby
 * EndOfCycle - 32 bit float number that represents the last time of block of the schedule
 * model_data - Vector of 128 bit integer tuples. This vector represents a list of time blocks that are allocated
 * recurType - String variable that represents how often the hobby reccurs
 */
#[derive(Clone, Debug)]
pub struct MyPheno{
    pub schedule: Vec<(i128, i128)>,
    pub newEventsIndex: Vec<usize>,
    pub EndOfCycle: f32,
    pub model_data: Vec<(i128, i128)>,
    pub recurType: String
}

impl Phenotype<i32> for MyPheno{
    /**
     * Fitness function
     * Parameters:
     *  self - Pointer to the MyPheno struct
     * Pre-condition: None
     * Post-condition: Returns a 32 bit integer that represents the fitness value of the genome's schedule
     */
    fn fitness(&self) -> i32{
        let mut minFitness: f32 = 999999999999999.0;
        //let temp = genome.schedule.clone();
        let ListOfFreeTime = ga::getListOfFreeTime(&self.schedule, self.EndOfCycle);
        for x in ListOfFreeTime{
            if (x.1 - x.0).abs() as f32 - minFitness < 0.0 && (x.1 - x.0 != 0){
                minFitness = (x.1 - x.0).abs() as f32;
            }
        }
        return minFitness as i32;
    }

    /**
     * Crossover function
     * Parameters:
     *  self - Pointer to the MyPheno struct
     *  other - Pointer to another MyPheno struct
     * Pre-condition: There is another selected genome
     * Post-condition: Returns a new MyPheno object that is created by the crossover function
     */
    fn crossover(&self, other: &MyPheno) -> MyPheno{
        let mut newGenome = self.clone();
        newGenome.EndOfCycle = self.EndOfCycle;
        let mut rng = rand::thread_rng();
        let mut newEventsList = Vec::<(i128, i128)>::new();
        let mut newSchedule = Vec::<(i128, i128)>::new();
        for i in 0..self.newEventsIndex.len(){
            let eventLength = self.schedule[self.newEventsIndex[i]].1 - self.schedule[self.newEventsIndex[i]].0;
            let mut mixScale: i128 = rng.gen_range(0..2);
            let mut currEventStart = self.schedule[self.newEventsIndex[i]].0  * mixScale + (1 - mixScale) * other.schedule[other.newEventsIndex[i]].0;
            let mut currEventEnd = currEventStart + eventLength;
            let mut nEvent = (currEventStart, currEventEnd);
            while checkViolations(&nEvent, &self.model_data, &self.recurType) {
                mixScale = rng.gen_range(0..2);
                currEventStart = self.schedule[self.newEventsIndex[i]].0  * mixScale + (1 - mixScale) * other.schedule[other.newEventsIndex[i]].0;
                currEventEnd = currEventStart + eventLength;
                nEvent = (currEventStart, currEventEnd);
            }
            newEventsList.push(nEvent);
        }
        for x in &self.model_data{
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
        //newGenome.fitness = ga::calculateFitness(&newGenome, newGenome.EndOfCycle);
        return newGenome.clone();
    }

    /**
     * Mutate function
     * Parameters:
     *  self - Pointer to the MyPheno struct
     * Pre-condition: None
     * Post-condition: Mutates the schedule of the genome
     */
    fn mutate(&self) -> MyPheno{
        let mut newEventsList = Vec::<(i128, i128)>::new();
        let mut temp = self.clone();
        for i in &temp.newEventsIndex{
            let mut eventTemp = ga::getNewTimings(&temp.schedule[*i]);
            while checkViolations(&eventTemp, &temp.model_data, &self.recurType) {
                eventTemp = ga::getNewTimings(&temp.schedule[*i]);
            }
            newEventsList.push(eventTemp);
        }
        
        let mut newSchedule = Vec::<(i128, i128)>::new();
        for x in &temp.model_data{
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
    
        temp.schedule = newSchedule.clone();
        temp.newEventsIndex = newEventsIndex.clone();
        return temp.clone();
        //self.fitness = ga::calculateFitness(&genome, genome.EndOfCycle);
    }
}

impl MyPheno{
    /**
     * Returns a list containing the timings of each hobby
     * Parameters:
     *  self - Pointer to the MyPheno struct
     * Pre-condition: None
     * Post-condition: Returns a vector of 128 bit integer containing the timings of each hobby
     */
    pub fn getNewEventsList(&self) -> Vec<(i128, i128)>{
        let mut newEventList: Vec<(i128, i128)> = Vec::new();
        for index in &self.newEventsIndex{
            newEventList.push(self.schedule[*index]);
        }
        return newEventList;
    }
}

/**
 * Gets list of non-allocated time chunks
 * Parameters:
 *  listOfEvents - Pointer to vector of 128 bit integer tuples. This represents the user's events in the GACO algorithm.
 *  EndOfCycle - 32 bit float number that represents the last time of block of the schedule
 * Pre-condition: None
 * Post-condition: Returns a vector of 128 bit integer containing all free time within the schedule
 */
pub fn getListOfFreeTime(listOfEvents: &Vec<(i128, i128)>, EndOfCycle: f32) -> Vec<(i128, i128)>{
    let mut ListOfFreeTime = Vec::new();
    let mut lastTime: i128 = 0;
    let mut startIndex = 0;
    if(lastTime == listOfEvents[0].0){
        lastTime = listOfEvents[0].1;
        startIndex = 1;
    }
    for x in startIndex..listOfEvents.len(){
        ListOfFreeTime.push((lastTime, listOfEvents[x].0));
        lastTime = listOfEvents[x].1;
    }
    ListOfFreeTime.push((lastTime, EndOfCycle as i128));
    return ListOfFreeTime;
}


/**
 * Changes event timings
 * Parameter:
 *  orgiEvent - Point to 128 bit integer tuple
 * Pre-condition: None
 * Post-condition: Returns a 128 bit integer tuple that represents the changed version of an event
 */
pub fn getNewTimings(origEvent: &(i128, i128)) -> (i128, i128){
    let mut rng = rand::thread_rng();
    let mut currEvent = *origEvent;
    let eventLength = currEvent.1 - currEvent.0;
    let mutateFactor = rng.gen_range(-48.0 .. 48.0) as i128;
    currEvent.0 = currEvent.0 + mutateFactor;
    currEvent.1 = currEvent.0 + eventLength;
    return currEvent;
}


/**
 * Checks if hobby violates any hard constraints
 * Parameters:
 *  newEvent - Poitner to 128 bit integer tuple
 *  model_data - Pointer to a vector of 128 bit integer tuples. This vector represents a list of time blocks that are allocated
 *  recur - Pointer to a string that represents the recurrence of the hobby. The function changes how violations are handled depending on the recurrance of the hobby.
 * Pre-condition: None
 * Post-condition: Returns true if the hobby violates any hard constraints. Otherwise, the function returns false
 */
pub fn checkViolations(newEvent: &(i128, i128), model_data: &Vec<(i128, i128)>, recur: &String) -> bool{
    let mut violated: bool = false;
    let mut checkByTime: bool = false;
    if recur != "J"{
        checkByTime = true;
    }
    //Add code to allow border cases
    if !checkByTime {
        for hc in model_data{
            if (hc.0 <= newEvent.0 && newEvent.1 <= hc.1)
            || (newEvent.0 <= hc.0  && hc.0 <= newEvent.1 && newEvent.1 <= hc.1) 
            || (hc.0 <= newEvent.0 && newEvent.0 <= hc.1 && hc.1 <= newEvent.1)
            || (newEvent.0 <= hc.0 && hc.1 <= newEvent.1 && hc.0 <= newEvent.1 && newEvent.0 <= hc.1)
            {
                // if hc.0 <= newEvent.0 && newEvent.1 <= hc.1 && cfg!(debug_assertions){
                //     println!("Case 2");
                // }
                // if newEvent.0 <= hc.0  && hc.0 <= newEvent.1 && newEvent.1 <= hc.1 && cfg!(debug_assertions){
                //     println!("Case 4");
                // }
                // if hc.0 <= newEvent.0 && newEvent.0 <= hc.1 && hc.1 <= newEvent.1 && cfg!(debug_assertions){
                //     println!("Case 5");
                // }
                violated = true;
                break;
            }
        }
    }
    if checkByTime {
        let modelDay = 24 * 5;
        for hc in model_data{
            if (hc.0 % modelDay  <= newEvent.0 % modelDay && newEvent.1 & modelDay <= hc.1 % modelDay) || 
            (newEvent.0 % modelDay <= hc.0 % modelDay  && hc.0 % modelDay <= newEvent.1 % modelDay && newEvent.1 % modelDay <= hc.1 % modelDay) || 
            (hc.0 % modelDay <= newEvent.0 % modelDay && newEvent.0 % modelDay <= hc.1 % modelDay && hc.1 % modelDay <= newEvent.1 % modelDay) ||
            (newEvent.0 % modelDay <= hc.0 % modelDay && hc.1 % modelDay <= newEvent.1 % modelDay && hc.0 % modelDay <= newEvent.1 % modelDay && newEvent.0 % modelDay <= hc.1 % modelDay){
                // if hc.0 <= newEvent.0 && newEvent.1 <= hc.1 && cfg!(debug_assertions){
                //     println!("Case 2");
                // }
                // if newEvent.0 <= hc.0  && hc.0 <= newEvent.1 && newEvent.1 <= hc.1 && cfg!(debug_assertions){
                //     println!("Case 4");
                // }
                // if hc.0 <= newEvent.0 && newEvent.0 <= hc.1 && hc.1 <= newEvent.1 && cfg!(debug_assertions){
                //     println!("Case 5");
                // }
                violated = true;
                break;
            }
        }
    }
    return violated;
}

/**
 * Get total fitness of pool
 * Parameters:
 *  gaPool - Pointer to a vector of MyPheno structs. This represents the genome population
 * Pre-condition: None
 * Post-condition: Returns a floating point number that represents the total fitness of the population
 */
pub fn calculateSumFitness(gaPool: &Vec<MyPheno>) -> f32{
    let mut fitnessSum = 0.0;
    for x in gaPool{
        fitnessSum += x.fitness() as f32;
    }
    return fitnessSum;
}

/**
 * Initalizes a genome
 * Parameters:
 *  hardConstraints - Pointer to vector of 128 bit integer tuple. This represents the user's events in the GACO algorithm
 *  listOfRequestedEvents -  Pointer to a vector of RequestedEvent structs. This represents the list of hobbies
 *  EndOfCycle - 32 bit float number that represents the last time of block of the schedule
 * Pre-condition: None
 * Post-condition: Returns an initalized MyPheno struct
 */
pub fn init(hardConstraints: &Vec<(i128, i128)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32) -> MyPheno{
    let mut temp : MyPheno = MyPheno{
        EndOfCycle: EndOfCycle,
        schedule: hardConstraints.clone(),
        model_data: hardConstraints.clone(),
        newEventsIndex: Vec::<usize>::new(),
        recurType: listOfRequestedEvents[0].recurType.clone()
    };
    let ListOfFreeTime = ga::getListOfFreeTime(hardConstraints, EndOfCycle);
    let mut rng = thread_rng();
    let mut listOfNewEvents = Vec::<(i128, i128)>::new();
    for x in listOfRequestedEvents{
        let mut freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
        while freeTime.1 - freeTime.0 < (x.length/5.0) as i128{
            freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
        }
        //Figure out of bounds bug for Lines 28 - 31 and add check for out of bounds
        let mut freetime_length = freeTime.1 - freeTime.0;
        let mut multiple: i128 = rng.gen_range(0 .. (freetime_length as f32/(x.length/5.0)) as i128);
        let mut start = freeTime.0 + (multiple * (x.length/5.0)  as i128);
        let mut sample_schedule: (i128, i128) = (start, start + (x.length/5.0) as i128);
        while ga::checkViolations(&sample_schedule, hardConstraints, &temp.recurType){
            freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            while freeTime.1 - freeTime.0 < (x.length/5.0) as i128{
                freeTime = ListOfFreeTime.choose(&mut rand::thread_rng()).unwrap();
            }
            freetime_length = freeTime.1 - freeTime.0;
            multiple = rng.gen_range(0 .. (freetime_length as f32/(x.length/5.0)) as i128);
            start = freeTime.0 + (multiple * (x.length/5.0)  as i128);
            sample_schedule = (start, start + (x.length/5.0) as i128);
        }
        temp.schedule.push(sample_schedule);
        listOfNewEvents.push(sample_schedule);
    }
    temp.schedule.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for x in listOfNewEvents{
        let index = temp.schedule.iter().position(|&r| r == x).unwrap();
        temp.newEventsIndex.push(index);
    }
    return temp.clone();
}

/**
 * Creates genome pool and applies the Genetic ALgorithm on the geneome pool
 * Parameters:
 *  population - 32 bit integer that represents the total number of ants
 *  hardConstraints - Pointer to a vector of 128 bit integer tuples. This represents the user's events in the GACO algorithm
 *  listOfRequestedEvents - Pointer to a vector of RequestedEvent structs. This represents the user's hobbies
 *  EndOfCycle - 32 bit float number that represents the last time of block of the schedule
 * Pre-condition: None
 * Post-condition: Returns a tuple containing the top 10 fittest genome and the average fitness of the new top fittest genome pool
 */
pub fn run(population: i32, hardConstraints: &Vec<(i128, i128)>, listOfRequestedEvents: &Vec<model::RequestedEvent>, EndOfCycle: f32) -> (Vec<MyPheno>, f32){
    let mut pop: Vec<MyPheno> = (0..population).map(|i| ga::init(hardConstraints, listOfRequestedEvents, EndOfCycle)).collect();
    let mut s = Simulator::builder(&mut pop)
        .set_selector(Box::new(UnstableMaximizeSelector::new(10)))
        .set_max_iters(50)
        .build();
    //s.run();
    let mut temp = StepResult::Success;
    let mut counter = 0;
    let mut fitnessTuple = (0.0, 0.0);
    let mut fitnessSum = 0.0;
    let mut popCopy = s.population();
    fitnessSum = ga::calculateSumFitness(&popCopy);
    fitnessTuple.0 = fitnessSum/(popCopy.len() as f32);
    loop {
        temp = s.checked_step();
        fitnessTuple.1 = fitnessTuple.0;
        match temp {
            StepResult::Failure => {
                println!("Failutre");
                break;
            },
            StepResult::Done => {
                break;
            },
            _ => {

                fitnessSum = ga::calculateSumFitness(&popCopy);
                fitnessTuple.0 = fitnessSum/(popCopy.len() as f32);
                if fitnessTuple.1/(fitnessTuple.0.powf(3.0)) < 0.03 {
                    counter += 1
                }else{
                    counter = 0;
                }

                if(counter == 3){
                    break;
                }
                
            }
        }
    }

    popCopy = s.population();
    popCopy.sort_by(|a, b| (a.fitness()).partial_cmp(&b.fitness()).unwrap());
    let mut selectedSolutions : Vec<MyPheno> = Vec::new();
    for i in 0..( (population * 10)/ 100) as usize{
        selectedSolutions.push(popCopy[i].clone());
    }
    //Get distinct selectedSolutions
    let mut avg_fitness = 0.0;
    for x in &selectedSolutions{
        avg_fitness += x.fitness() as f32;
    }
    avg_fitness = avg_fitness/ (selectedSolutions.len() as f32);
    return (selectedSolutions, avg_fitness);
}