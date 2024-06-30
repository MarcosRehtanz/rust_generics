// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
// fn fn_point() {
//     let mouse_pointer: Point<f32> = Point { x: 3.0, y: 5.5 };
//     println!("{:#?}", mouse_pointer);
//     let x = mouse_pointer.x;
//     let y = mouse_pointer.y;
//     println!("{} + {} = {}", x, y, x + y);
// }

// #[derive(Debug)]
// struct Vector<T, U> {
//     x: T,
//     y: U,
// }
// fn fn_vector() {
//     let user_vector = Vector { x: 5, y: "up" };
//     println!("{:#?}", user_vector);
//     let x = user_vector.x;
//     let y = user_vector.y;
//     println!("{} + \'{}\' = Error", x, y);
// }

// // Rasgo - aplicado
// trait Area {
//     fn area(&self) -> f64;
// }
// struct Circle {
//     radius: f64,
// }
// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// // Extends from Area to Circle
// impl Area for Circle {
//     fn area(&self) -> f64 {
//         use std::f64::consts::PI;
//         PI * self.radius.powf(2.0)
//     }
// }
// // Extends from Area to Rectangle
// impl Area for Rectangle {
//     fn area(&self) -> f64 {
//         self.height * self.width
//     }
// }

// fn fn_math() {
//     let circle = Circle { radius: 15.0 };
//     println!("area of circle: {}", circle.area())
// }

// trait AsJson {
//     fn as_json(&self) -> String;
// }
// // Cualquier valor que tenga implementado AsJson
// fn send_data_as_json(value: &impl AsJson) {
//     println!("Sending JSON data to server...");
//     println!("-> {}", value.as_json());
//     println!("Done!\n");
// }
// struct Person {
//     name: String,
//     age: u8,
//     favorite_fruit: String,
// }
// impl AsJson for Person {
//     fn as_json(&self) -> String {
//         format!(
//             r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
//             self.name, self.age, self.favorite_fruit
//         )
//     }
// }

// fn fn_creatures() {
//     let person = Person {
//         name: String::from("Juan"),
//         age: 26,
//         favorite_fruit: String::from("Banana"),
//     };
//     send_data_as_json(&person);
// }

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}
impl Counter {
    fn new(length: usize) -> Counter {
        Counter { count: 0, length }
    }
}
impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

struct Container<T> {
    value: T,
}
impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

#[derive(Debug)]
struct Groups<T> {
    inner: Vec<T>,
}
impl<T> Groups<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}
impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let mut group = Vec::new();
        let first = self.inner.remove(0);
        group.push(first);

        while !self.inner.is_empty() && self.inner[0] == group[0] {
            group.push(self.inner.remove(0));
        }

        Some(group)
    }
}

fn main() {
    let mut counter = Counter::new(3);
    println!("Counter: {:#?}", counter);
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));

    println!("\n-------------\n");

    let container = Container::new(String::from("Juan"));
    println!("I\'m {}", container.value);

    println!("\n-------------\n");

    let vector = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    let group = Groups::new(vector);
    let _group = group.into_iter().collect::<Vec<Vec<_>>>();
    println!("group: {:#?}", _group);
    assert_eq!(
        _group,
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );
}
