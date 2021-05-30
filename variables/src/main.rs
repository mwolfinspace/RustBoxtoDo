fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    let spaces = "      ";
    let spaces = spaces.len();
    println!("The number of space is: {}", spaces);
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("There are three characters: {}, {}, {}", c, z, heart_eyed_cat);
     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
 
     // remainder
     let remainder = 43 % 5;
     println!("Sum is: {}. Difference is: {}. Product is: {}. Quotient is: {}. Remainder is: {}", sum, difference, product, quotient, remainder);

     let t = true;
     let f = false;
     println!("Now we know t for {} and f for {}", t, f);

     let tup = (500, 6.4, 1);
     let (_x, y, _z) = tup;
     println!("The value of y is: {}", y);
     let five_hundred = tup.0;
     let six_point_four = tup.1;
     let one = tup.2;
     println!("A tup has 3 values: {}, {}, {}", five_hundred, six_point_four, one);

     let _a = [1, 2, 3, 4, 5];
     let _month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
}
