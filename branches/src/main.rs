fn main() {
    let number = 3;

    //Cấu trúc nếu thì (if... else...)
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    a_loop();
    a_while();
    a_while_with_array();
    a_for();
    a_rev_for();
}

//Hàm vòng lặp có bộ đếm kết hợp if... else...
fn a_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

//Hàm vòng lặp while theo điều kiện cho trước
fn a_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//Hàm vòng lặp while đọc mảng có sẵn
fn a_while_with_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

//Hàm vòng lặp for đọc mảng có sẵn
fn a_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

//Hàm vòng lặp for đọc mảng có sẵn theo trình tự ngược lại
fn a_rev_for() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
