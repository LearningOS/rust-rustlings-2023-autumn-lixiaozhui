// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);


    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice);
    assert_eq!(&[2, 3, 4], nice_slice);
    if [2, 3, 4] == *nice_slice {
        println!("[2, 3, 4]== nice_slice");
    }
    if [2, 3, 4] == nice_slice {
        println!("[2, 3, 4]== nice_slice");
    }
    if &[2, 3, 4] == nice_slice {
        println!("&[2, 3, 4] == nice_slice");
    }
}
