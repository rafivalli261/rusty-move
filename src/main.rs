

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

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("result = {:?}", result);

    let test = ();
    println!("test = {:?}", test);
}

#[test]
fn array() {
    let mut data: [i16; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("data = {:?}", data);

    let a: i16 = data[0];
    let b: i16 = data[1];
    println!("{} {}", a, b);

    data[0] = 10;
    data[1] = 20;
    println!("{:?}", data);
    let length = data.len();
    println!("length = {}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 3] = [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("matrix[0][0] = {}", matrix[0][0]);
    println!("matrix[1][1] = {}", matrix[1][1]);
}

#[test]
fn constant() {
    const PHI: f32 = 3.14;
    const GRAVITY: f32 = 9.8;
    println!("PHI = {}", PHI);
    println!("GRAVITY = {}", GRAVITY);
}

const MAX_SCORE: i8 = 100;

#[test]
fn variable_scope() {
    println!("Max Score = {}", MAX_SCORE);
    let number = 2;
    {
        let id = 9;
        println!("Halo {}", id);
    }
    println!("Halo {}", number);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let id = 10;
    let country = String::from("Indonesia");
    println!("ID = {}, Country = {}", id, country);
}

fn function_b() {
    let number = 7;
    let name = String::from("Rafi Valli");
    println!("Number = {}, Name = {}", number, name);
}

#[test]
fn string_slice() {
    let mut name: &str = "Rafi Valli";
    println!("Halo {}", name);
    name = "Valli Rafi info";
    println!("Halo {}", name);
}


#[test]
fn string() {
    let country: &str = "       Indonesia      ";
    let trimmed = country.trim();
    println!("Country = '{}'", country);
    println!("Trimmed = '{}'", trimmed);
}

#[test]
fn string_mutation() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s = {}", s);
}

#[test]
fn string_type() {
    let mut name = String::from("Rafi");
    println!("Halo {}", name);

    name.push_str(" Valli");
    println!("Halo {}", name);

    let new_name:String = name.replace("Rafi", "Renjana");
    println!("Halo {}", new_name);
}

#[test]
fn ownership_rules() {
    let a = 12;

    {
        let b = 34;
        println!("{}, {}", a, b);
    }

    println!("{}", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let mut b = a;
    b += 5;
    println!("a = {}, b = {}", a, b);
}

#[test]
fn ownership_movement() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
}

#[test]
fn clone() {
    let name1 = String::from("Hello");
    let name2 = name1.clone();
    println!("name1 = {}, name2 = {}", name1, name2);
}