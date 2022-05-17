use std::env;

mod actions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];
    match &mode[..] {
        "socket" => run_socket_mode(),
        "console" => run_console_mode(),
        _ => panic!("Illegal argument (expected : socket / console)")
    };
}

fn run_console_mode() {
    println!("CONSOLE MODE");
    actions::action_list::create_action_list_file(String::from("test.yaml"));
}

fn run_socket_mode() {
    println!("SOCKET MODE");
}