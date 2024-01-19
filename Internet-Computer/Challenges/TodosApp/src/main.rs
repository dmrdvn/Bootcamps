
// We keep variables for each task in a single struct
#[derive(Clone,Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// We keep each task in a struct according to its id so that we can call it later.
struct ToDoList {
    tasks: Vec<Task>,
    counter: usize,
}

// Adding task
fn add_task(todos: &mut ToDoList ,description: &str) -> Task {
        
    let task = Task {
        id: todos.counter,
        description: description.to_string(),
        completed: false,
    };

    todos.tasks.push(task.clone());
    todos.counter += 1;

    task
}

// Completing task
fn complete_task(todos: &mut ToDoList, id: usize) -> Option<&Task> {
    
    if let Some(task) = todos.tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        Some(task)
    } else {
        None
    }
}

// Listing tasks
fn list_tasks(todos: &ToDoList) {
    
    for task in &todos.tasks {
        println!(
            "ID: {}, Description: {}, Completed: {}",
            task.id, task.description, task.completed
        );
    }
}


fn main() {
    
    println!(" ========= To Do App =========");
    
    let mut todos = ToDoList {
        tasks: Vec::new(),
        counter: 1,
    };

    let task1 = add_task(&mut todos,"Hello World!");
    let task2 = add_task(&mut todos,"Hello Rust!");
    let _task2 = add_task(&mut todos,"Bye Solidity!");

    list_tasks(&todos);
    complete_task(&mut todos,task1.id);
    complete_task(&mut todos,task2.id);
    
    println!("Task number {} completed.", task1.id);
    
    list_tasks(&todos);
    
}
