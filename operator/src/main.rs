fn main() {
    tambah(10, 5); 
    kurang(10, 5);
    kali(10, 5);
    bagi(10.0, 5.0);
}

fn tambah(a: u32, b: u32) {
    let hasil = a + b;
    println!("{} + {} = {}", a, b, hasil);
}

fn kurang(a: i32, b: i32) {
    let hasil = a - b;
    println!("{} - {} = {}", a, b, hasil);
}

fn kali(a: u32, b: u32) {
    let hasil = a * b;
    println!("{} x {} = {}", a, b, hasil);
}

fn bagi(a: f32, b: f32) {
    let hasil = a / b;
    println!("{:.2} / {:.2} = {:.2}", a, b, hasil);
}
