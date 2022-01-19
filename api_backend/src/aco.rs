use crate::model;
use crate::ga;
use rand::{thread_rng, Rng};

//use crate::ga;
//use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Ant{
    pub path: Vec<(i128, i128)>,
    pub curr_node: (i128, i128),
    pub EndOfCycle: f32
}

impl Ant{
    pub fn move_to_end_node(&mut self, to: (i128, i128)){
        self.curr_node = to;
        self.path.push(self.curr_node);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node{
    pub interval: (i128, i128),
    pub pheromone: f32
}

#[derive(Debug, Clone)]
pub struct Graph{
    pub nodes: Vec<Vec<Node>>
}

impl Graph{
    pub fn init(&mut self, list_of_new_events: &Vec<model::RequestedEvent>, list_of_free_intervals: &Vec<(i128, i128)>){
            let mut nodes = Vec::<Vec<Node>>::new();
            for new_event in list_of_new_events{
                let mut node_list = Vec::<Node>::new();
                for interval in list_of_free_intervals{
                    for i in 1..((interval.1 - interval.0) as f32/(new_event.length * 12.0)) as i128 + 1{
                        let mut node: Node = Node{
                            interval: (0, 0),
                            pheromone: 0.0
                        };
                        node.interval = (interval.0 + ((i-1) as f32 * new_event.length * 12.0) as i128 , interval.0 + (i as f32 * new_event.length * 12.0) as i128);
                        node.pheromone = 0.0;
                        node_list.push(node);
                    }
                }
                nodes.push(node_list);
            }
            self.nodes = nodes;
    }
}

pub fn run(population: i32, list_of_new_events: &Vec<model::RequestedEvent>, list_of_free_intervals: &Vec<(i128, i128)>, chromosones: &Vec<ga::GAPath>, model_data: &Vec<(i128, i128)>) -> Vec<Vec<(i128, i128)>>{
    let mut aco_graph : Graph = Graph{
        nodes: Vec::<Vec<Node>>::new()
    };
    let mut curr_best_path: Vec::<(i128, i128)> = Vec::<(i128, i128)>::new();
    aco_graph.init(list_of_new_events, list_of_free_intervals);
    let mut possible_path: Vec<Vec<(i128, i128)>> = Vec::<Vec<(i128, i128)>>::new();
    let mut pheromone_sum: Vec::<f32> = Vec::<f32>::new();
    for chromo in chromosones{
        let path = chromo.getNewEventsList();
        if !possible_path.contains(&path){
            possible_path.push(path.to_vec());
        }    
    }
    for path in possible_path{
        for i in 0..aco_graph.nodes.len(){
            let index = aco_graph.nodes[i].iter().position(|&r| r.interval == path[i]);
            if index != None{
                aco_graph.nodes[i][index.unwrap()].pheromone += 10.0;
            }
        }
    }

    for i in 0..aco_graph.nodes.len(){
        pheromone_sum.push(0.0);
        for j in 0..aco_graph.nodes[i].len(){
            aco_graph.nodes[i][j].pheromone += 10.0;
            pheromone_sum[i] += aco_graph.nodes[i][j].pheromone;
        }
    }

    let mut ants: Vec<Ant> = Vec::<Ant>::new();
    for _ in 0..population{
        let ant: Ant = Ant{
            path: Vec::<(i128, i128)>::new(),
            curr_node: (-1, -1),
            EndOfCycle: chromosones[0].EndOfCycle
        };
        ants.push(ant);
    }

    let mut counter = 0;
    while !is_over(&aco_graph, counter){
        for i in 0..aco_graph.nodes.len(){
            aco_graph.nodes[i].retain(|&x| x.pheromone != 0.0);
        }
        println!("GRAPH: {:?}", aco_graph);
        reinitalize_ants(&mut ants, population);
        move_ants(&aco_graph, &mut ants, &pheromone_sum, model_data);
        update_pheromone(&mut aco_graph, &mut ants, chromosones[0].EndOfCycle, model_data);
        pheromone_sum = get_pheromone_sum(&aco_graph);
        curr_best_path = ants[0].path.clone();
        if cfg!(debug_assertions) {
            println!("");
        }
        counter += 1;
    }

    if cfg!(debug_assertions){
        println!("Outside Loop: {:?}", ants);
        if counter >= 100 {
            println!("Hit Limit");
        }
    }
    let mut listOfPaths = Vec::<Vec<(i128, i128)>>::new();
    for ant in ants{
        if !listOfPaths.contains(&ant.path){
            listOfPaths.push(ant.path);
        }
    }
    listOfPaths.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("LIST OF PATH: {:?}", listOfPaths);
    return listOfPaths;
    // Add exit conditions
}

pub fn get_pheromone_sum(aco_graph: &Graph) -> Vec<f32>{
    let mut pheromone_sum: Vec<f32> = Vec::<f32>::new();
    for i in 0..aco_graph.nodes.len(){
        pheromone_sum.push(0.0);
        for j in 0..aco_graph.nodes[i].len(){
            if aco_graph.nodes[i][j].pheromone < 0.0{
                continue;
            }
            pheromone_sum[i] += aco_graph.nodes[i][j].pheromone;
        }
    }
    return pheromone_sum;
}

pub fn get_pheromone_col_sum(node_col: &Vec<Node>, path_travelled: &Vec<(i128, i128)>, model_data: &Vec<(i128, i128)>, EndOfCycle: f32) -> f32{
    let mut sum = 0.0;
    for node in node_col{
        let mut temp_path = path_travelled.clone();
        temp_path.push(node.interval);
        let fitness = calculateFitness(&temp_path, model_data, EndOfCycle);
        sum += fitness + node.pheromone;
    }
    return sum;
}

pub fn update_pheromone(aco_graph: &mut Graph, ants: &Vec<Ant>, EndOfCycle: f32, model_data: &Vec<(i128, i128)>){
    //Remove Heurisitc Update
    let mut avg_fitness = 0.0;
    for ant_index in 0..ants.len(){
        avg_fitness += calculateFitness(&ants[ant_index].path, &model_data, EndOfCycle);
    }
    avg_fitness = avg_fitness/ants.len() as f32;
    for ant_index in 0..ants.len(){
        let fitnesValue = calculateFitness(&ants[ant_index].path, &model_data, EndOfCycle);
        for node_index in 0..ants[ant_index].path.len(){
            let index = aco_graph.nodes[node_index].iter().position(|&r| r.interval == ants[ant_index].path[node_index]);
            if index != None{
                aco_graph.nodes[node_index][index.unwrap()].pheromone += 20.0;
            }
        }
    }

    for i in 0..aco_graph.nodes.len(){
        for j in 0..aco_graph.nodes[i].len(){
            aco_graph.nodes[i][j].pheromone -= 20.0;
            if aco_graph.nodes[i][j].pheromone < 0.0{
                aco_graph.nodes[i][j].pheromone = 0.0;
            }
        }
    }
}

pub fn calculateFitness(path: &Vec<(i128, i128)>, model_data: &Vec<(i128, i128)>, EndOfCycle: f32) -> f32{
    let mut minFitness: f32 = 999999999999999.0;
    let mut ListOfFreeTime = Vec::new();
    let mut lastTime: i128 = 0;

    let mut temp_path = model_data.clone();
    for node in path{
        temp_path.push(*node);
    }
    temp_path.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for x in temp_path{
        ListOfFreeTime.push((lastTime, x.0));
        lastTime = x.1;
    }
    ListOfFreeTime.push((lastTime, EndOfCycle as i128));
    for x in ListOfFreeTime{
        if (x.1 - x.0).abs() as f32 - minFitness < 0.0{
            minFitness = (x.1 - x.0).abs() as f32;
        }
    }
    return minFitness;

}

pub fn move_ants(aco_graph: &Graph, ants: &mut Vec<Ant>, pheromone_sum: &Vec<f32>, model_data: &Vec<(i128, i128)>){
    //Add Heuristic Infomation
    //Change Code to Use Hash Table for Heuristic Infomation Lookup
    let mut rng = thread_rng();
    for i in 0..aco_graph.nodes.len(){
        for ant_index in 0..ants.len(){
            let mut prob: f32 = 0.0;
            let mut prob_cap = rng.gen::<f32>();
            for node in &aco_graph.nodes[i]{
                let mut curr_path = ants[ant_index].path.clone();
                curr_path.push(node.interval);
                prob += node.pheromone * calculateFitness(&curr_path, model_data, ants[ant_index].EndOfCycle) / get_pheromone_col_sum(&aco_graph.nodes[i], &ants[ant_index].path, model_data, ants[ant_index].EndOfCycle);
                if  prob_cap < prob{
                    ants[ant_index].move_to_end_node(node.interval);
                    break;
                }
            }
        }
    }

}

pub fn reinitalize_ants(ants: &mut Vec<Ant>, population: i32){
    let temp = ants[0].EndOfCycle;
    *ants = Vec::<Ant>::new();
    for _ in 0..population{
        let ant: Ant = Ant{
            path: Vec::<(i128, i128)>::new(),
            curr_node: (-1, -1),
            EndOfCycle: temp
        };
        ants.push(ant);
    }
}

pub fn is_over(aco_graph : &Graph, iterationNum: i128) -> bool{
    if iterationNum == 0{
        return false;
    }
    if iterationNum > 100{
        return true;
    }
    for i in 0..aco_graph.nodes.len(){
        let first = aco_graph.nodes[i][0];
        for j in 1..aco_graph.nodes[i].len() {
            if (first.pheromone != aco_graph.nodes[i][j].pheromone){
                return false;
            }
        }
    }
    return true;
}