mod address;
mod cli;
mod config;
mod task;
mod todolist;

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Todolist {
    id: String,
    name: String,
}

#[derive(Tabled)]
struct Task {
    id: String,
    status: String,
    project: String,
    description: String,
}

#[derive(Tabled)]
struct Address {
    address: String,
    todolist_id: String,
    allowed_addresses: String,
}

#[tokio::main]
async fn main() {
    let matches = cli::cli().get_matches();
    match matches.subcommand() {
        Some(("todolist", sub_matches)) => {
            let todolist_subcommand = sub_matches.subcommand().unwrap();
            match todolist_subcommand {
                ("add", args) => {
                    let todolist_name = args.get_one::<String>("name").unwrap();
                    todolist::create_todolist(todolist_name).await.unwrap();
                    println!("New todolist `{}` created", todolist_name);
                }
                ("delete", args) => {
                    let todolist_id = args.get_one::<String>("id").unwrap();
                    todolist::delete_todolist(todolist_id).await.unwrap();
                    println!("Todolist deleted");
                }
                ("list", _) => {
                    let todolists = todolist::get_todolists().await.unwrap();
                    let data: Vec<Todolist> = todolists
                        .into_iter()
                        .map(|x| Todolist {
                            id: x.todolist_id,
                            name: x.name,
                        })
                        .collect();
                    println!("{}", Table::new(data));
                }
                (name, _) => {
                    unreachable!("Unknown command {}", name)
                }
            }
        }
        Some(("task", sub_matches)) => {
            let todolist_subcommand = sub_matches.subcommand().unwrap();
            match todolist_subcommand {
                ("add", args) => {
                    let todolist_id = args.get_one::<String>("todolist_id").unwrap();
                    let project = args.get_one::<String>("project").unwrap();
                    let description = args.get_one::<String>("description").unwrap();
                    task::create_task(todolist_id, project, description)
                        .await
                        .unwrap();
                    println!("New task created");
                }
                ("list", _) => {
                    let tasks = task::get_tasks().await.unwrap();
                    let data: Vec<Task> = tasks
                        .into_iter()
                        .map(|x| Task {
                            id: x.task_id,
                            status: x.status,
                            project: x.project,
                            description: x.description,
                        })
                        .collect();
                    println!("{}", Table::new(data));
                }
                ("delete", args) => {
                    let task_id = args.get_one::<String>("id").unwrap();
                    task::delete_task(task_id).await.unwrap();
                    println!("Task deleted");
                }
                ("complete", args) => {
                    let task_id = args.get_one::<String>("id").unwrap();
                    task::complete_task(task_id).await.unwrap();
                    println!("Task completed");
                }
                ("edit", args) => {
                    let task_id = args.get_one::<String>("id").unwrap();
                    let project = args.get_one::<String>("project");
                    let description = args.get_one::<String>("description");
                    task::edit_task(task_id, project, description)
                        .await
                        .unwrap();
                    println!("Task edited");
                }
                ("share", args) => {
                    let task_id = args.get_one::<String>("id").unwrap();
                    let address = args.get_one::<String>("address").unwrap();
                    task::share_task(task_id, address).await.unwrap();
                    println!("Task shared");
                }
                ("unshare", args) => {
                    let task_id = args.get_one::<String>("id").unwrap();
                    let address = args.get_one::<String>("address").unwrap();
                    task::unshare_task(task_id, address).await.unwrap();
                    println!("Task unshared");
                }
                (name, _) => {
                    unreachable!("Unknown command {}", name)
                }
            }
        }
        Some(("address", sub_matches)) => {
            let address_subcommand = sub_matches.subcommand().unwrap();
            match address_subcommand {
                ("add", _) => {
                    address::create_address().await.unwrap();
                    println!("New address created");
                }
                ("list", _) => {
                    let addresses = address::get_addresses().await.unwrap();
                    let data: Vec<Address> = addresses
                        .into_iter()
                        .map(|x| Address {
                            address: x.address,
                            todolist_id: x.todolist_id.unwrap_or_default(),
                            allowed_addresses: x
                                .allowed_addresses
                                .into_iter()
                                .map(|a| format!("{} @{}\n", a.address, a.username))
                                .collect(),
                        })
                        .collect();
                    println!("{}", Table::new(data));
                }
                ("delete", args) => {
                    let address = args.get_one::<String>("address").unwrap();
                    address::delete_address(address).await.unwrap();
                    println!("Address deleted");
                }
                ("attach", args) => {
                    let address = args.get_one::<String>("address").unwrap();
                    let todolist_id = args.get_one::<String>("todolist_id").unwrap();
                    address::attach_address(address, todolist_id).await.unwrap();
                    println!("Address attached successfuly");
                }
                ("detach", args) => {
                    let address = args.get_one::<String>("address").unwrap();
                    address::detach_address(address).await.unwrap();
                    println!("Address detached successfuly");
                }
                ("allow", args) => {
                    let address = args.get_one::<String>("address").unwrap();
                    let remote_address = args.get_one::<String>("remote_address").unwrap();
                    address::allow_address(address, remote_address)
                        .await
                        .unwrap();
                    println!("Remote address allowed");
                }
                ("revoke", args) => {
                    let address = args.get_one::<String>("address").unwrap();
                    let remote_address = args.get_one::<String>("remote_address").unwrap();
                    address::revoke_address(address, remote_address)
                        .await
                        .unwrap();
                    println!("Remote address revoked");
                }
                (name, _) => {
                    unreachable!("Unknown command {}", name)
                }
            }
        }
        _ => unreachable!(),
    }
}
