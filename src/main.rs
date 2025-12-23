fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello Rafi");
}

#[test]
fn test_variable(){
    let name = "Rafi Valli";
    println!("Halo {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Rafi";
    println!("Halo {}", name);

    name = "Valli";
    println!("Halo {}", name);
}

#[test]
fn static_typing() {
    let name = "Rafi";
    println!("Halo {}", name);
    // name = 10;
    println!("Hello {}", name)
}

#[test]
fn shadowing() {
    let name = "Rafi";
    println!("Halo {}", name);
    let name = 10;
    println!("Halo {}", name);
}

#[test]
fn comment() {
    println!("Hello"); // this is comment
    /*
        this is multiple
        line comments
        with yes
     */
}

#[test]
fn explicit() {
    let number:i8 = 3;
    println!("Nomor {}", number);
}

#[test]
fn number() {
    let a:i8 = 4;
    println!("a = {}", a);

    let b:f32 = 12.4;
    println!("b = {}", b)
}

#[test]
fn number_conversion() {
    let a:i8 = 4;
    println!("a = {}", a);

    let b:i16 = a as i16;
    println!("b = {}", b);

    let c:i32 = b as i32;
    println!("c = {}", c);

    let d:i32 = 1000000;
    let e:i8 = d as i8; // int overflow
    println!("e = {}", e);
}

#[test]
fn numeric_operator() {
    let a = 7;
    let b = 3;
    let c = a * b;
    println!("a * b = {}", c);
    let d: f32 = a as f32 / b as f32;
    println!("a / b = {}", d);
    let e = a + b;
    println!("a + b = {}", e);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("a = {}", a);
    a += 7;
    println!("a = {}", a);
    a -= 3;
    println!("a = {}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b:bool = false;
    println!("a = {} dan b = {}", a, b);
}

#[test]
fn comparison() {
    let a: i8 = 10;
    let b: i8 = 20;
    let result = a > b;
    println!("result = {}", result);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("lulus final = {}", lulus_final);
}

#[test]
fn character() {
    let a = 'a';
    println!("a = {}", a);
}

#[test]
fn tuple() {
    let mut data:(i32, f32, bool, char) = (7, 2.8, true, 'z');
    println!("{:?}", data);
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
    // let d = data.3;
    let (a, b, c, _) = data;
    println!("a = {}, b = {}, c = {}", a, b, c);

    data.0 = 9;
    data.1 = 7.6;
    data.2 = false;
    data.3 = 'i';
    println!("{:?}", data);
}

