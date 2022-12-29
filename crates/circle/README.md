# 介绍

该 `crate` 的功能是提供了一个 `Circle`的累，并且实现了通过园的其中一个参数来计算其它参数的功能。

## Circle 结构体

```rust
#[derive(Default)]
pub struct Circle {
    radius: f64,    // 半径
    area: f64,      // 面积
    perimeter: f64, // 周长
    diameter: f64,  // 直径
}
```

该结构体应用了 `Default` Trait，这样我们就可以通过 `let mut c1: Circle = Default::default();` 的方式来创建一个带默认值的对象。

## CircleEnum

```rust
#[allow(dead_code)]
pub enum CircleEnum {
    R(f64), // 半径
    A(f64), // 面积
    P(f64), // 周长
    D(f64), // 直径
}
```

该枚举的作用是作为 `Circle` 的方法 `update`的参数，通过传入不同的枚举成员来执行不同的计算，并且可以用枚举成员的值。

## 实现 update 方法

```rust
impl Circle {

    pub fn update(&mut self, circle_enum: CircleEnum) {
        let radius_to_area = |radius: f64| (radius * radius) * PI;
        let radius_to_perimeter = |radius: f64| radius * 2.0 * PI;
        let radius_to_diameter = |radius: f64| radius * 2.0;

        // 根据 circle_enum 的枚举类型执行不同的计算
        match circle_enum {
            // 有半径获取园的其他信息
            CircleEnum::R(radius) => {
                
                let area = radius_to_area(radius);
                let perimeter = radius_to_perimeter(radius);

                self.area = area;
                self.diameter = radius_to_diameter(radius);
                self.perimeter = perimeter;
                self.radius = radius;
            },
            // 有周长获取园的其他信息
            CircleEnum::P(perimeter) => {
                let radius = perimeter / PI / 2.0;
                let area = radius_to_area(radius);
                
                self.area = area;
                self.diameter = radius_to_diameter(radius);
                self.perimeter = perimeter;
                self.radius = radius;
            },
            // 有面积获取园的其他信息
            CircleEnum::A(area) => {
                let radius = (area / PI).sqrt();
                let perimeter = radius_to_perimeter(radius);
                
                self.area = area;
                self.diameter = radius_to_diameter(radius);
                self.perimeter = perimeter;
                self.radius = radius;
            },
            // 有直径获取园的其他信息
            CircleEnum::D(diameter) => {
                let radius = diameter / 2.0;
                let area = radius_to_area(radius);
                let perimeter = radius_to_perimeter(radius);
                
                self.area = area;
                self.diameter = diameter;
                self.perimeter = perimeter;
                self.radius = radius;

            }

        }
    }

}
```

## 实现 Display

```rust
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle Info: \nRadius: {:.5}\nArea: {:.5}\nPerimeter: {:.5}\nDiameter: {:.5}", self.radius, self.area, self.perimeter, self.diameter)
    }
}
```

## 简单使用

```rust
use circle::{Circle, CircleEnum};

fn main() {
    let mut c1: Circle = Default::default();
    c1.update(CircleEnum::A(12.5));
    println!("{}", c1);
}
```