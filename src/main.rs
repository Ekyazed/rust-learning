use std::io;
#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    level: i64,
}
#[derive(Debug)]
struct Tasks {
    inner: Vec<Task>
}

impl Tasks {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, task: Task) {
        println!("Task {:?} ajouter", &task.name);
        self.inner.push(task);
    }

    fn get_all(&self) -> &Vec<Task> {
        &self.inner
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please entry a valid data")
    }
    buffer.trim().to_owned()
}

fn get_f64_value() -> i64 {
    loop {
        let input = get_input();
        let parsed_input: Result<i64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return amount,
            Err(_) => println!("Merci de renseigner une data valide")
        }
    }
}

fn create_task(tasks: &mut Tasks) {
    println!("");
    println!("Donnez un titre à votre tâche");
    let name = get_input();
    println!("Donnez une description à votre tâche");
    let description = get_input();
    println!("Donnez un niveau de priorité à votre tâche");
    let level = get_f64_value();
    let task = Task {name, description, level};
    tasks.add(task)
}

fn display_tasks(tasks: &Tasks) {
    for task in tasks.get_all() {
        println!("");
        println!("titre de la tâche: {:?} \nDescription de la tâche: {:?} \nNiveau de priorité: {:?}", task.name, task.description, task.level)
    }
}

fn show_menu() {
    fn show() {
        println!("");
        println!("== Task Manager ==");
        println!("1. Créer une nouvelle tâche");
        println!("2. Afficher toutes les tâches");
        println!("");
        println!("Choisissez une option");
    }

    let mut tasks = Tasks::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => create_task(&mut tasks),
            "2" => display_tasks(&tasks),
            _ => break,
        }
    }

}

fn main() {
    show_menu();
}
