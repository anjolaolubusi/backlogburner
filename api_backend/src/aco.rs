use crate::model;
//use crate::ga;
//use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Ant{
    pub path: Vec<(f32, f32)>,
    pub curr_node: (f32, f32)
}

impl Ant{
    pub fn MoveToEndNode(&mut self, start: (f32, f32), to: (f32, f32)){
        self.path.push(start);
        self.curr_node = to;
    }
}

#[derive(Debug, Clone)]
pub struct Node{
    pub interval: (f32, f32),
    pub pheromone: f32,
}

#[derive(Debug, Clone)]
pub struct Graph{
    pub nodes: Vec<Vec<Node>>
}

impl Graph{
    pub fn init(&mut self, list_of_new_events: &Vec<model::RequestedEvent>, list_of_free_intervals: &Vec<(f32, f32)>){
            let mut nodes = Vec::<Vec<Node>>::new();
            for new_event in list_of_new_events{
                let mut node_list = Vec::<Node>::new();
                for interval in list_of_free_intervals{
                    for i in 1..((interval.1 - interval.0)/new_event.length) as i128 + 1{
                        let mut node: Node = Node{
                            interval: (0.0, 0.0),
                            pheromone: 0.0
                        };
                        node.interval = (interval.0 + ((i-1) as f32 * new_event.length) , interval.0 + (i as f32 * new_event.length));
                        node.pheromone = 0.0;
                        node_list.push(node);
                    }
                }
                nodes.push(node_list);
            }
            self.nodes = nodes;
    }
}

pub fn run(population: i32, list_of_new_events: &Vec<model::RequestedEvent>, list_of_free_intervals: &Vec<(f32, f32)>){
    
}