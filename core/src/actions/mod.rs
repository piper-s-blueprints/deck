mod action_list;

fn initialize_action_list(name: &String) {
    if !action_list::check_action_list_file(name) {
        action_list::create_action_list_file(name);
    }
}

pub fn get_action_list(name: String) -> String {
    initialize_action_list(&name);
    action_list::get_action_list(&name)
}