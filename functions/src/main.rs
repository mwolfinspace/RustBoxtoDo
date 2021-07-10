//Chương trình chính
fn main() {
    println!("Hello, world!");

    another_function();

    print_number(10);

    print_double_number(5, 6);

    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let hun = hundred();
    println!("This is hun: {}", hun);

    println!("So when hun add one is: {}", plus_one(hun));

}

//Hàm in chữ
fn another_function() {
    println!("Another function.");
}

//Hàm in số
fn print_number(x: i32){
    println!("The value of x is: {}", x);
}

//Hàm nhân đôi và in số
fn print_double_number(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

//Hàm tạo hằng số
fn hundred() -> i32{
    100
}

//Hàm cộng thêm cho một
fn plus_one(x: i32) -> i32{
    x + 1
}