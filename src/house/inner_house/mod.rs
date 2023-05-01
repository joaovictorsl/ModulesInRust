mod bedroom;
mod kitchen;
mod bathroom;

pub fn ask_friend_how_is_inside() -> String {
    let bedroom_situation = bedroom::peek();
    let kitchen_situation = kitchen::peek();
    let bathroom_situation = bathroom::peek();
    let mut overall_situation = String::new();

    overall_situation.push_str(&bedroom_situation);
    overall_situation.push('\n');
    overall_situation.push_str(&kitchen_situation);
    overall_situation.push('\n');
    overall_situation.push_str(&bathroom_situation);
    
    overall_situation
}
