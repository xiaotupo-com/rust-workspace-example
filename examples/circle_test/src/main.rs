use circle::{Circle, CircleEnum};

fn main() {
    let mut c1: Circle = Default::default();
    c1.update(CircleEnum::A(12.5));
    println!("{}", c1.clone());



    println!("c1 的半径：{:.5}", c1.radius());
    println!("c1 的直径：{:.5}", c1.diameter());
    println!("c1 的面积：{:.5}", c1.area());
    println!("c1 的周长：{:.5}", c1.perimeter());

    c1.update(CircleEnum::R(10.5));
    println!("c1 的半径：{:.5}", c1.radius());
    println!("c1 的直径：{:.5}", c1.diameter());
    println!("c1 的面积：{:.5}", c1.area());
    println!("c1 的周长：{:.5}", c1.perimeter());
}
