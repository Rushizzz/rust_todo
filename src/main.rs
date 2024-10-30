use std::io;

#[derive(Debug)]
struct Tasks {
    id : u8,
    name : String,
    completed : bool,
}

impl Tasks {
    fn new(id : u8) -> Tasks {
        let mut user_line = String::new();
        println!("Enter your task");
        let _ = io::stdin().read_line(&mut user_line);
        let name = user_line.trim().to_string();
        let completed = false;

        Tasks {id, name, completed}
    }

    fn completed(&mut self) {
        self.completed = true;
    }

    fn view(&self) {
        println!("id: {}, name: {}, completed: {}", self.id, self.name, self.completed);
    }
}




fn main() {
    let mut database: Vec<Tasks> = Vec::new();
    let mut id : u8 = 0;

    let temp = println!("{:#?}", database);

    loop {
        println!("Please enter your command");
        println!("1. Insert \n2. View \n3. Delete \n4. Exit");

        let mut user_input = String::new();
        let _ = io::stdin().read_line(&mut user_input);
        let user_input = user_input.trim().parse().unwrap();


        match user_input {
            1 => {
                if id > 0 {
                    id += 1;
                    database.push(Tasks::new(id))
                }
                else {
                    id += 1;
                    database.push(Tasks::new(id))
                }
            }
            2 => {
                for i in database.iter_mut() {
                    println!("inside view");
                    i.view()
                }
            }
            3 => {
                for i in database.iter_mut() {
                    i.view()
                }
                let mut user_id = String::new();
                println!("Enter your id which you want to remove");
                let _ = io::stdin().read_line(&mut user_id)
                .expect("Failed to read line");
                let user_id = user_id.trim().parse().unwrap();

                database.retain(|t| t.id != user_id);
            }
            4 => break,
            _ => println!("Invalid command!"),
        };
    }

}
