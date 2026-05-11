// Generics Pratice
// Here T, U are used as part of standards, there are need not to always same
// We can name whatever we want, just follow UpperCamalCase

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Method definition
// We have to give <T> to impl, so that compiler knows that T in Point<T> is a generic
// Not any data type. Else we get the error: cannot find type `T` in this scope
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
struct PointMultiType<T, U> {
    x: T,
    y: U,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest // return largest
}

fn main() {
    let nums = vec![1, 2, 30, 4, 5];
    let chrs = vec!['c', 'b', 'a'];

    println!("{}", largest(&nums));
    println!("{}", largest(&chrs));

    let point1 = Point { x: 4, y: 5 };
    println!("point1.x: {}", point1.x());

    // throws error
    // mismatched types: expected integer, found floating-point number
    // because Point<T>, when x is assigned with 4, T defers to Interger
    // and when 5.0 come which is floating point causes the error because
    // by that time complier decided the T would be i32.
    // We can fix by using multiple type parameters
    // let point2 = Point { x: 4, y: 5.0 };
    // println!("{:?}", point2);

    let point2 = PointMultiType { x: 4, y: 5 };  // works now
    let point3 = PointMultiType { x: 4.0, y: 5 }; // works
    let point4 = PointMultiType { x: 4, y: 5.0 }; // works
    println!("{:?}", point2);
    println!("{:?}", point3);
    println!("{:?}", point4);

}