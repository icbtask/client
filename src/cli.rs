use clap::{arg, builder, Command};

pub fn cli() -> Command {
    Command::new("icbtask")
        .subcommand_required(true)
        .subcommand(
            Command::new("todolist")
                .about("Manage todolists")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .about("Add new todolist")
                        .arg(
                            arg!(--name <name>)
                                .required(true)
                                .help("todolist name (Required)")
                                .require_equals(true)
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a todolist")
                        .arg(
                            arg!(--todolist_id <todolist_id>)
                                .long("todolist-id")
                                .required(true)
                                .require_equals(true)
                                .help("todolist id (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg_required_else_help(true),
                )
                .subcommand(Command::new("list").about("List all todolists")),
        )
        .subcommand(
            Command::new("task")
                .about("Manage tasks")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .about("Add new task")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--todolist_id <todolist_id>)
                                .long("todolist-id")
                                .required(true)
                                .help("todolist id (Required)")
                                .require_equals(true)
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg(
                            arg!(--project <project>)
                                .required(true)
                                .require_equals(true)
                                .help("project name (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg(
                            arg!(--description <description>)
                                .required(true)
                                .require_equals(true)
                                .help("Task description (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("list")
                        .about("List all tasks")
                        .arg(
                            arg!(--show_complete)
                                .long("show-complete")
                                .help("Show completed tasks as well (Optional)")
                                .action(builder::ArgAction::SetTrue)
                                .required(false),
                        )
                        .arg(
                            arg!(--todolist_id <todolist_id>)
                                .long("todolist-id")
                                .required(false)
                                .help("Show only tasks from this todolist (Optional)"),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a task")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--task_id <task_id>)
                                .long("task-id")
                                .required(true)
                                .help("task id (Required)")
                                .require_equals(true)
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("complete")
                        .about("Set a task as complete")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--task_id <task_id>)
                                .long("task-id")
                                .required(true)
                                .require_equals(true)
                                .help("task id (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("edit")
                        .about("Edit a task")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--task_id <task_id>)
                                .long("task-id")
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(arg!(--project <project>).require_equals(true))
                        .arg(arg!(--description <description>).require_equals(true)),
                )
                .subcommand(
                    Command::new("share")
                        .about("Share a task")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--task_id <task_id>)
                                .long("task-id")
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("remote address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("unshare")
                        .about("Unshare a task")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--task_id <task_id>)
                                .long("task-id")
                                .required(true)
                                .require_equals(true),
                        )
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("remote address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                ),
        )
        .subcommand(
            Command::new("address")
                .about("Manage addresses")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(Command::new("add").about("Add new address"))
                .subcommand(Command::new("list").about("List all addresses"))
                .subcommand(
                    Command::new("delete")
                        .about("Delete an address")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("user address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("attach")
                        .about("Attach an address to a todolist")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("user address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg(
                            arg!(--todolist_id <todolist_id>)
                                .long("todolist-id")
                                .required(true)
                                .require_equals(true)
                                .help("todolist id (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("detach")
                        .about("Detach an address from a todolist")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("user address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("allow")
                        .about("Allow an address")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("user address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg(
                            arg!(--remote_address <remote_address>)
                                .long("remote-address")
                                .required(true)
                                .require_equals(true)
                                .help("remote address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        ),
                )
                .subcommand(
                    Command::new("revoke")
                        .about("Revoke an address")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--address <address>)
                                .required(true)
                                .require_equals(true)
                                .help("user address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg(
                            arg!(--remote_address <remote_address>)
                                .long("remote-address")
                                .required(true)
                                .require_equals(true)
                                .help("remote address (Required)")
                                .value_parser(builder::NonEmptyStringValueParser::new()),
                        )
                        .arg_required_else_help(true),
                ),
        )
}
