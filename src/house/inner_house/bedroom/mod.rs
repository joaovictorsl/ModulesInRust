pub mod closet;
// Can't be accessed directly.
const OWNER_NAME: &str = "Anderson";

pub fn peek() -> String {
    // OWNER_NAME can't be accessed directly from outside
    // but this function can format it into a String and return to the outside modules.
    let peeking_on_closet = closet::peek();

    let mut peek_result = format!("There's {}'s bedroom and it has a closet.", OWNER_NAME);
    peek_result.push_str(&peeking_on_closet);

    peek_result
}
