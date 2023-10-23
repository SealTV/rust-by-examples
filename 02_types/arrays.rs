use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}" , slice[0]);
    println!("The slice has {} elements.", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

     // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());


    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow whole array as a slice");
    analyze_slice(&xs);


    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);


    let empty_array: [i32; 0] = [];

    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(val) =>  println!("{} {}", i, val),
            None => println!("Slow down! {} it too far!", i),
        }
    }
}