fn main() {
    //     let mut x = 5;
    //     println!("The value of x is: {}", x);
    //     x = 6;
    //     println!("The value of x is: {}", x);
    //

    //     let x = 5;
    //     let x = x + 1;
    //     let x = x * 2;
    //     println!("The value of x is: {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("The value of guess is: {}", guess);

    // let guess = 1_000u16;
    // println!("The value of guess is: {}", guess);

    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32
    // println!("The value of x is: {}, y is: {}", x, y);

    // let x = 'z';
    // let y = 'ℤ';
    // let heart_eyed_cat = '😻';
    // println!("The value of x is: {}, y is: {}, heart_eyed_cat is: {}", x, y, heart_eyed_cat);

    // Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, _z) = tup;
    // let z = tup.2;
    // println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);

    // Array
    // Arrays have fixed length
    // let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // creates array with 5 elements and each value is 3
    let a = [3; 5];
    println!("The value of a[0] is: {}", a[0]);
}
