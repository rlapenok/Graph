use super::vertex::*;

use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
pub struct Graph<T> {
    pub count: usize,
    pub hashmap: HashMap<usize, Vertex<T>>,
}

impl<T: PartialEq + Debug + std::clone::Clone> Graph<T> {
    //Creating a new graph, where a new vertex is created in this graph with id=1,and value=None;
    pub fn new() -> Self {
        let mut hashmap: HashMap<usize, Vertex<T>> = HashMap::new();
        hashmap.insert(1 as usize, Vertex::new_default());
        Self {
            count: 1,
            hashmap: hashmap,
        }
    }
    //The graph is pushed vertxe with a value=None, whose id=id(prev vertex)+1;
    pub fn push_vertex(&mut self) {
        self.hashmap.insert(self.count + 1, Vertex::new_default());
        self.count += 1;
    }
    //Changing the value of a vertex by its id to new_value;
    pub fn insert_change_value(&mut self, id: usize, new_value: T) {
        let vertex = self.hashmap.get_mut(&id).unwrap();
        vertex.insert_value(new_value);
    }

    //An edge is created between the vertices;
    pub fn insert_edge(&mut self, to_id: usize, from_id: usize) {
        let from_ptr =
            self.hashmap.get_mut(&from_id).unwrap() as *const Vertex<T> as *mut Vertex<T>;
        let to_vertex = self.hashmap.get_mut(&to_id).unwrap();

        to_vertex.push_vertex(from_ptr);
    }

    //A vertex is deleted by its id;
    pub fn pop_vertex(&mut self, id_vertex: usize) {
        //Сделать так, чтобы при удалении смещались индексы графа
        let ptr = self.hashmap.get(&id_vertex).unwrap() as *const Vertex<T> as *mut Vertex<T>;

        self.hashmap.iter().for_each(|some_vertex| {
            some_vertex.1.pop(ptr);
        });

        let result = self.hashmap.remove_entry(&id_vertex).unwrap();
        println!("Vertex with id :{} was pop successfully", result.0);
        self.count-=1;
        drop(result);
    }

    //The graph is serialized in TGF to a file graph.txt
    /*Пример получающегося файла graph.txt
    1 node
    2 node
    3 node
    4 node
    5 node
    6 node
    7 node
    8 node
    9 node
    10 node
    #
    Vertex id : 1 have Edge to 2 3
    Vertex id : 2 have Edge to 4 5
    Vertex id : 3 have Edge to 7 6
    Vertex id : 4 have Edge to 8 9
    Vertex id : 8 have Edge to 9
    Vertex id : 9 have Edge to 10  */
    pub fn serealize(&self) {
        let file = match File::open("graph.txt") {
            Err(_) => File::create("graph.txt").unwrap(),
            Ok(result) => result,
        };
        let mut buff = BufWriter::new(file);

        let mut data = Vec::new();
        self.hashmap.iter().for_each(|vertex| {
            data.push(vertex.0);
        });
        data.sort();
        data.iter().for_each(|vertex| {
            buff.write(format!("{} node\n", vertex).as_bytes()).unwrap();
        });
        buff.write("#\n".as_bytes()).unwrap();
        self.hashmap.iter().for_each(|vertex| {
            if vertex.1.edge.borrow().is_empty() {
            } else {
                let mut edge = String::new();
                let mut vec_of_id = vertex
                    .1
                    .edge
                    .borrow()
                    .iter()
                    .map(|ptr| {
                        let vertex = unsafe { &**ptr };
                        self.hashmap
                            .iter()
                            .find(|result| {
                                *result.1.value.as_ref().unwrap() == *vertex.value.as_ref().unwrap()
                            })
                            .unwrap()
                            .0
                    })
                    .collect::<Vec<&usize>>();
                vec_of_id.sort();
                vec_of_id.iter().for_each(|id| {
                    edge.push_str(id.to_string().as_str());
                    edge.push_str(" ");
                });
                buff.write(format!("Vertex id : {} have Edge to {}\n", vertex.0, edge).as_bytes())
                    .unwrap();
            }
        })
    }

    pub fn bfs(&self, value: T) {
        let mut vec_deque = VecDeque::new();
        let root = self.hashmap.get(&1).unwrap();
        root.bfs(value, &mut vec_deque);
    }
}

pub fn deserealize<T: Debug + PartialEq + std::clone::Clone>(addr: &str) -> Graph<T> {
    let file = File::open(addr).unwrap();

    let buff = BufReader::new(file);

    let mut graph: Graph<T> = Graph::new();

    for lines in buff.lines() {
        let result = lines.unwrap().trim().to_owned();
        if result.contains("#") {
        } else if result.starts_with("Vertex") {
            let vertex_edges = result
                .split_whitespace()
                .filter_map(|data| data.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            println!("{:?}", vertex_edges);
            for i in 1..vertex_edges.len() {
                graph.insert_edge(vertex_edges[0], i);
            }
        } else if result.starts_with("1") {
        } else {
            graph.push_vertex();
        }
    }
    graph
}
