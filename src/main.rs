mod house;
pub use house::{inner_house, outer_house};
// pub use house::{inner_house as inner, outer_house as outer};

fn main() {
    let access_backyard = outer_house::backyard::peek();
    let overall_inside_situation = inner_house::ask_friend_how_is_inside();
    // The line below throws an error since bedroom is private.
    // let access_bedroom = house::inner_house::bedroom::current_location();
    println!("You can see in the backyard: {}", access_backyard);
    println!(
        "By asking a friend from inside the house, you know that:\n{}",
        overall_inside_situation
    );
}

// Output:
// You can see in the backyard: The dog is drinking water.
// By asking a friend from inside the house, you know that:
// There's Anderson's bedroom and it has a closet.This is Anderson's closet, there's a lot of red T-Shirts. I also know that there are 42 pans in the kitchen.
// Wow, there's 42 pans in this kitchen!
// Someone's taking a shower.
