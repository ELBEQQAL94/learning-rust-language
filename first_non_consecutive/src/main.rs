fn main() {
    println!("The result is: {:?}", first_non_consecutive(vec![-5,-4,-3,-1]));
}

fn first_non_consecutive(arr: Vec<i32>) -> Option<i32> {
    // let mut index: i32 = arr[0];
    // for element in arr.iter() {
    //     if index != *element {
    //         return Some(*element);
    //     } else {
    //         index += 1;
    //     }
    // }
    // None
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] != 1 {
          return Some(arr[i]);
        }
      };
      None
}
