use std::{collections::HashMap, env, io::Read, str::FromStr};

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), FromStr::from_str(v).unwrap())).collect();
            Ok(Todo { map })
    }

    fn insert(&mut self, key: &String) {
        self.map.insert(key.to_string(), false);
    }

    fn upadte(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = !*v),
            None => None
        }
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }

        std::fs::write("db.txt", content)
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
