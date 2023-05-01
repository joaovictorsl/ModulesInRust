// You can access bedroom, kitchen and bathroom from closet
// since inner modules know the context where they were created.
pub fn peek() -> String {
    let number_of_pans_in_kitchen = super::super::kitchen::NUMBER_OF_PANS;
    let owner_of_this_closet = super::OWNER_NAME.to_owned();

    format!(
        "This is {}'s closet, there's a lot of red T-Shirts. I also know that there are {} pans in the kitchen.", 
        owner_of_this_closet,
        number_of_pans_in_kitchen
    )
}
