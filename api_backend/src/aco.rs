use crate::model;
use crate::ga;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng, Rng};

//use crate::ga;
//use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Ant{
    pub path: Vec<(i128, i128)>,
    pub curr_node: (i128, i128)
}

impl Ant{
    pub fn MoveToEndNode(&mut self, to: (i128, i128)){
        self.curr_node = to;
        self.path.push(self.curr_node);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node{
    pub interval: (i128, i128),
    pub pheromone: f32,
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

pub fn run(population: i32, list_of_new_events: &Vec<model::RequestedEvent>, list_of_free_intervals: &Vec<(i128, i128)>, chromosones: &Vec<ga::GAPath>){
    let mut aco_graph : Graph = Graph{
        nodes: Vec::<Vec<Node>>::new()
    };
    let mut curr_best_path: Vec::<Node> = Vec::<Node>::new();
    aco_graph.init(list_of_new_events, list_of_free_intervals);
    let mut possible_path: Vec<Vec<(i128, i128)>> = Vec::<Vec<(i128, i128)>>::new();
    let mut pheromone_sum: Vec::<f32> = Vec::<f32>::new();
    let mut rng = thread_rng();
    for chromo in chromosones{
        let path = chromo.getNewEventsList();
        if !possible_path.contains(&path){
            possible_path.push(path.to_vec());
        }    
    }

    for path in possible_path{
        for i in 0..aco_graph.nodes.len(){
            let index = aco_graph.nodes[i].iter().position(|&r| r.interval == path[i]);
            if(index != None){
                aco_graph.nodes[i][index.unwrap()].pheromone += 20.0;
                println!("WOOP")
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
            curr_node: (-1, -1)
        };
        ants.push(ant);
    }
    for i in 0..aco_graph.nodes.len(){
        for ant_index in 0..ants.len(){
            let mut prob: f32 = 0.0;
            for node in &aco_graph.nodes[i]{
                prob += node.pheromone / &pheromone_sum[i];
                if rng.gen::<f32>() < prob{
                    ants[ant_index].MoveToEndNode(node.interval);
                    break;
                }
            }
        }
    }

    for ant_index in 0..ants.len(){
        for node_index in 0..ants[ant_index].path.len(){
            let index = aco_graph.nodes[node_index].iter().position(|&r| r.interval == ants[ant_index].path[node_index]);
            if (index != None){
                aco_graph.nodes[node_index][index.unwrap()].pheromone += 5.0;
            }
        }
    }

    for i in 0..aco_graph.nodes.len(){
        for j in 0..aco_graph.nodes[i].len(){
            aco_graph.nodes[i][j].pheromone -= 10.0;
        }
    }

    println!("{:?}", ants);
    // Add exit conditions
}