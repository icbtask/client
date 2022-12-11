use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("icbtask")
        .subcommand_required(true)
        .subcommand(
            Command::new("todolist")
                .about("Manage todolists")
                .subcommand_required(true)
                .subcommand(
                    Command::new("add").about("Add new todolist").arg(
                        arg!(--name <name>)
                            .short('n')
                            .required(true)
                            .require_equals(true),
                    ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a todolist")
                        .arg(arg!(--id <todolist_id>).required(true).require_equals(true)),
                )
                .subcommand(Command::new("list").about("List all todolists")),
        )
        .subcommand(
            Command::new("task")
                .about("Manage tasks")
                .subcommand_required(true)
                .subcommand(
                    Command::new("add")
                        .about("Add new task")
                        .arg(
                            arg!(--todolist_id <todolist_id>)
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--project <project>)
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--description <description>)
                                .required(true)
                                .require_equals(true),
                        ),
                )
                .subcommand(Command::new("list").about("List all tasks"))
                .subcommand(
                    Command::new("delete")
                        .about("Delete a task")
                        .arg(arg!(--id <task_id>).required(true).require_equals(true)),
                )
                .subcommand(
                    Command::new("complete")
                        .about("Set a task as complete")
                        .arg(arg!(--id <task_id>).required(true).require_equals(true)),
                )
                .subcommand(
                    Command::new("edit")
                        .about("Edit a task")
                        .arg(arg!(--id <task_id>).required(true).require_equals(true))
                        .arg(arg!(--project <project>).require_equals(true))
                        .arg(arg!(--description <description>).require_equals(true)),
                )
                .subcommand(
                    Command::new("share")
                        .about("Share a task")
                        .arg(arg!(--id <task_id>).required(true).require_equals(true))
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true),
                        ),
                )
                .subcommand(
                    Command::new("unshare")
                        .about("Unshare a task")
                        .arg(arg!(--id <task_id>).required(true).require_equals(true))
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("address")
                .about("Manage addresses")
                .subcommand_required(true)
                .subcommand(Command::new("add").about("Add new address"))
                .subcommand(Command::new("list").about("List all addresses"))
                .subcommand(
                    Command::new("delete").about("Delete an address").arg(
                        arg!(--address <address>)
                            .required(true)
                            .require_equals(true),
                    ),
                )
                .subcommand(
                    Command::new("attach")
                        .about("Attach an address to a todolist")
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--todolist_id <todolist_id>)
                                .required(true)
                                .require_equals(true),
                        ),
                )
                .subcommand(
                    Command::new("detach")
                        .about("Detach an address from a todolist")
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true),
                        ),
                )
                .subcommand(
                    Command::new("allow")
                        .about("Allow an address")
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--remote_address <remote_address>)
                                .required(true)
                                .require_equals(true),
                        ),
                )
                .subcommand(
                    Command::new("revoke")
                        .about("Revoke an address")
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--remote_address <remote_address>)
                                .required(true)
                                .require_equals(true),
                        ),
                ),
        )
}
