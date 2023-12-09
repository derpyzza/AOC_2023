use std::fs;

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

fn traverse_tree(node: Node) {

}



fn main() {

    let input_file = "./testCase.txt";

    let contents = fs::read_to_string(input_file)
        .expect("Could not read file");

    let instructions: Vec<char> = contents.split("\n").into_iter().next()
        .unwrap().chars().collect();
    let mut instruction_pointer = 0;
    let mut lines: Vec<Node> = contents.split("\n").skip(2)
        .into_iter()
        .map(|x| {
        if x == "" {
            None
        } else {
            let raw: Vec<String> = x.split("=").map(|x| x.to_string()).collect();
            let _id = raw[0].clone();
            let vars: [String; 2] = raw[1].clone().split(",").into_iter().map(|x| {
                let y: String = x.chars().map(|c| {
                    if c == '(' || c == ')' || c == ' ' {
                        None
                    } else {
                        Some(c)
                    }
                }).filter_map(|x| x)
                .collect();
                y
            }).collect::<Vec<_>>().try_into().unwrap();

            let node = Node {
                id: _id,
                left: vars[0].clone(),
                right: vars[1].clone(),
            };

            println!("vars: {:#?}", vars);

            Some(node)
        }
    }).filter_map(|x| x).collect();

    lines.sort_by(|a, b| a.cmp(&b));

    for (id, line) in lines.into_iter().enumerate() {
        println!("[{}] line: {:#?}", id, line);
    }
}
