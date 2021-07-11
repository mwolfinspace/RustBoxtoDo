use std::io;

fn main(){
    println!("Tính toán hai số cơ bản!");
    let num1 = input_number_loop();
    let num2 = input_number_loop();
    add(num1,num2);
    subtract(num1,num2);
    multiply(num1,num2);
    division(num1,num2);

}

fn input_number_loop() -> f32{
    println!("Vui lòng nhập số:");
    loop {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let _check_input:f32 = match input.trim().parse() {
            Ok(number) => {
                println!("Số bạn đã nhập là: {}", number);
                return number;
            },
            Err(_) => {
                println!("Làm ơn nhập số vào ạ!");
                println!("Vui lòng nhập SỐ NHÉ!");
                continue;
            },
        };
    }
}

// fn input_number() -> f32{
//     println!("Vui lòng nhập số:");
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Fail to read the line");
//     let number: f32 = input.trim().parse().expect("Please type a number!");
//     println!("Bạn đã nhập: {}", number);
//     return number
// }



fn add(a: f32, b: f32){
    println!("Tổng {} + {} là: {}", a, b, a + b);
}

fn subtract(a: f32, b: f32){
    println!("Hiệu {} - {} là: {}", a, b, a - b);
}

fn multiply(a: f32, b: f32){
    println!("Tích {} x {} là: {}", a, b, a * b);
}

fn division(x: f32, y: f32){
    if y==0.0{
        println!("Phép chia cho 0! Dẹp khỏi tính!");
    } else {
        println!("Thương {} / {} là: {}", x, y, x/y);
    }
}

