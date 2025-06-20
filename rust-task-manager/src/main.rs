use std::io::{stdin, Stdin, Write};

#[derive(Debug)]
enum Status {
    Pending,
    Completed,
}

struct Task {
    id: u32,
    name: String,
    status: Status,
}

enum Options {
    Create,
    List,
    Delete,
    CompleteTask,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut task_counter : u32 = 0;


    loop {
        let mut option: String = String::new();

        println!("");
        println!("please select below Option:");
        println!("1. Create tasks");
        println!("2. List tasks:");
        println!("3. Delete tasks");
        println!("4. complete tasks");
        println!("5. quit");

        print!("> ");
        std::io::stdout().flush().unwrap();

        stdin().read_line(&mut option).expect("failed to read line");

        if (option.trim() == "5") {
            println!("Exiting...");
            break;
        }

        let user_choice: Options = match option.trim() {
            "1" => Options::Create,
            "2" => Options::List,
            "3" => Options::Delete,
            "4" => Options::CompleteTask,
            // "5" => option = String::from("quit"),
            _ => {
                println!("Invalid choice");
                break;
            }
        };
        std::io::stdout().flush().unwrap();

        match user_choice {
            Options::Create => {
                let mut name: String =  String::new();
                println!("Enter task name");
                stdin().read_line(&mut name).expect("failed to read name");
                task_counter = task_counter + 1;
                let task = Task {
                    id: task_counter,
                    name: name,
                    status: Status::Pending,
                };

                tasks.push(task);
                println!("Task created.");
            }

            Options::List => {
                println!("Tasks: ");
                for (i, task) in tasks.iter().enumerate() {
                    println!("Task id : {}", i + 1);
                    println!("Task Name: {}", task.name.trim().to_string());
                    println!("Task Status: {:?}\n", task.status);
                }
            },
            Options::Delete => {
                let mut task_id: String = String::new();
                println!("please enter the Task Id which you want to delete: ");
                stdin().read_line(&mut task_id).expect("unable to read the id");

                if let Ok(index) = task_id.trim().parse::<usize>() {
                    if index == 0 || index > tasks.len() {
                        println!("Invalid task number.");
                    } else {
                        tasks.remove(index - 1);
                        println!("Task deleted.");
                    }
                }

            },
            Options::CompleteTask => {
                let mut task_id: String = String::new();
                println!("please enter the task id which you want to update: ");
                stdin().read_line(&mut task_id).expect("unable to read the id");

                if let Ok(index) = task_id.trim().parse::<usize>() {
                    if index == 0 || index > tasks.len() {
                        println!("Invalid task number.");
                    } else {
                        tasks[index - 1].status = Status::Completed;
                        println!("Task deleted.");
                    }
                }
            }
        }
    }
}
