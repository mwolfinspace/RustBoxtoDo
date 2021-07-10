fn main() {
    println!("Tính toán hai số cơ bản!");
    add(1,2);
    subtract(3,2);
    multiply(5,3);
    
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// fn division(x: i32, y: i32) -> Result <i32, &'static srt> {
//     let a = x as i32;
//     let b = y as i32;
//     let out = a.checked_div(b)
//             .ok_or_else(||{
//                 "Division by zero"
//             })?;
// }

