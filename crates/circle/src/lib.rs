
use std::f64::consts::PI;
use std::fmt;

#[allow(dead_code)]
pub enum CircleEnum {
    R(f64), // 半径
    A(f64), // 面积
    P(f64), // 周长
    D(f64), // 直径
}

#[derive(Default, Clone, Copy)]
pub struct Circle {
    radius: f64,    // 半径
    area: f64,      // 面积
    perimeter: f64, // 周长
    diameter: f64,  // 直径
}

impl Circle {

    pub fn update(&mut self, circle_enum: CircleEnum) {
        let radius_to_area = |radius: f64| (radius * radius) * PI;
        let radius_to_perimeter = |radius: f64| radius * 2.0 * PI;
        let radius_to_diameter = |radius: f64| radius * 2.0;

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

    // 获取面积
    pub fn area(self) -> f64 {
        self.area
    }

    // 获取半径
    pub fn radius(self) -> f64 {
        self.radius
    }

    // 获取周长
    pub fn perimeter(self) -> f64 {
        self.perimeter
    }

    // 获取直径
    pub fn diameter(self) -> f64 {
        self.diameter
    }

}



impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle Info: \nRadius: {:.5}\nArea: {:.5}\nPerimeter: {:.5}\nDiameter: {:.5}", self.radius, self.area, self.perimeter, self.diameter)
    }
}
