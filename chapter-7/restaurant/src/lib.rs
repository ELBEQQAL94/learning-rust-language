// mod fron_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // absolute path
//     fron_of_house::hosting::add_to_wait_list();

//     // relative path
//     crate::fron_of_house::hosting::add_to_wait_list();
// }

fn serve_order() {}

mod back_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}
