#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectange) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let rect1 = Rectange {
    //     width: 30,
    //     height: 50,
    // };

    // let rect2 = Rectange {
    //     width: 60,
    //     height: 200,
    // };

    // let rect3 = Rectange {
    //     width: 80,
    //     height: 80,
    // };
    // println!("The rect2 can hold rect3 {}", rect3.can_hold(&rect2));
    println!("{}", expressions_matter(3, 3, 3))
}

fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    let result = [a * (b + c), a * b * c, a + b * c, (a + b) * c];
    let maximum: &u64 = largest(&result);
    *maximum
}

fn largest(list: &[u64]) -> &u64 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn maximu(list: &[i32]) -> i32 {
    *list.iter().max().unwrap()
}
