#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    println!("Hello, world!");


    // Ownership

    // copy value
    let x = 10;
    let y = x;
    println!("x = {x}, y = {y}");

    // move value (kode dibawah akan error karena value s1 telah dipindahkan ke s2 dan s1 sudah tidak memiliki value kembali)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 = {s1}, s2 = {s2}");

    // Cara agar tidak error maka kita clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");

    // owenership pada function
    let s = String::from("hello"); // variable s masuk ke scope

    takes_ownership(s); // Variable s diambil ownership nya oleh function dan s tidak akan valid lagi setelahnya

    // println!("{s}"); // ini akan error

    let i = 1;

    makes_copy(i);


    // ownership dengan function yang return value
    let string1 = gives_ownership(); // func akan mereturn value into string1
    let string2 = String::from("hello"); //string 2 masuk ke scope
    let string3 = takes_and_gives_back(string2); // string 2 akan dipindahkan ke func dan akan di return kembali nilai nya ke string 3


    // references and borrowing
    let  str1 = String::from("hlo");

    let len  = calculate_string(&str1); // & merupakan tanda bahwa mereferensi ke s1
    println!("str1 = {str1}, len s1 = {len}"); // s1 masih dapat digunakan karena kita hanya mereferensi tanpa memindahkan

    let mut s = String::from("hello");

    // Penggunaan yang salah
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // kita bisa borrow s yang mutable karena s sedang di pinjam oleh dengan immutable
    // println!("{r1}, {r2}, and {r3}");

    // Penggunaan yang benar
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, and {r2}");

    let r3 = &mut s;
    println!("{r3}");

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect1 is {:?}", rect1);


}

fn calculate_string(str: &String)->usize{
    str.len()
}

fn gives_ownership() -> String {
    String::from("hello") // jangan di beri titik koma agar rust mengira ini adalah retur value nya
}

fn takes_and_gives_back(string: String) -> String{
    string
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}



