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
        self.map.insert(key.to_string(), false);
    }

    fn list(self) {
        for (k, v) in self.map {
            println!("{k} => {v}");
        }
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

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }
}

fn main() {
    let mut todo = Todo::new().expect("Initialisation of db failed");

    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    
    match &command[..] {
        "add" => {
                todo.insert(&args[2]);
                    match todo.save() {
                        Ok(_) => todo.list(),
                        Err(why) => println!("An error occured: {}", why),
                    }
                }
        "update" => 
            match todo.upadte(&args[2]) {
                None => println!("'{}' is not present in the list", &args[2]),
                Some(_) => match todo.save() {
                    Ok(_) => todo.list(),
                    Err(why) => println!("An error ocurred: {}", why)
                }
            }
        "ls" => todo.list(),
        _ => println!("worng command")
    }

}
