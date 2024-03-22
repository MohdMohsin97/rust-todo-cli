use std::{collections::HashMap, env};

#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new()
            }), 
            Err(e) => panic!("An error ocurred: {}", e),
        }

    }

    fn insert(&mut self, key: &String) {
        self.map.insert(key.to_string(), true);
    }

    fn upadte(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = !*v;    
                println!("Update: {:?}", self.map); 
                Some({})
            }
            None => None
        }
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        println!(" Save: {:?}", self.map);
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }
}

fn main() {
    let mut todo = Todo::new().expect("Initialisation of db failed");

    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let item = &args[2];

    match &command[..] {
        "add" => {
                todo.insert(item);
                    match todo.save() {
                        Ok(_) => println!("todo saved"),
                        Err(why) => println!("An error occured: {}", why),
                    }
                }
        "update" => 
            match todo.upadte(&item) {
                None => println!("'{}' is not present in the list", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("Todo saved"),
                    Err(why) => println!("An error ocurred: {}", why)
                }
            }
        _ => println!("worng command")
    }

}
