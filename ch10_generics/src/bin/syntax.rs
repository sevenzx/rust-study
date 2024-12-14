struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 可以选择为 Point<f64> 实例实现方法，而不是为泛型 Point 实例
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    println!("p1.x = {}", p1.x());
    println!("p2.x = {}", p2.x());
    let p3 = Point { x: 2.14, y: 9.25 };
    println!("distance from origin: {}", p3.distance_from_origin());
}