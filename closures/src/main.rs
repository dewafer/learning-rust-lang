// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor{ 
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }

//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }

// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }

/* ------------------ */

// fn main() {
    //     let mut list = vec![1,2,3];
    //     println!("Before defining closure: {list:?}");
    //     let mut b2 = &list;
    
    //     let mut borrows_mutable = || list.push(7);
    
    //     println!("Before calling closure: {b2:?}");
    //     borrows_mutable();
    //     println!("After calling closure: {list:?}");
    // }

    
/* ------------------ */


use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    
    thread::spawn(move || println!("From thread: {list:?}"))
    .join()
    .unwrap();
    // println!("After thread run: {list:?}");
}