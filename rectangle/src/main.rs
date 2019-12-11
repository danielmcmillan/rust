#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width <= self.width && rect.height <= self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let rect4 = Rectangle::square(40);

    println!(
        "The area of {:?} is {} square pixels.",
        rect4,
        rect4.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle { width: 10, height: 6 };
        let rect2 = Rectangle { width: 9, height: 6 };

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let rect1 = Rectangle { width: 10, height: 6 };
        let rect2 = Rectangle { width: 9, height: 6 };

        assert!(!rect2.can_hold(&rect1));
    }
    
    #[test]
    fn can_hold_equal() {
        let rect1 = Rectangle { width: 10, height: 6 };
        let rect2 = Rectangle { width: 10, height: 6 };

        assert!(rect1.can_hold(&rect2));
        assert!(rect1.can_hold(&rect1));
    }

    #[test]
    fn area_is_width_times_height() {
        let rect1 = Rectangle { width: 10, height: 6 };

        let result = rect1.area();
        let expected = 60;
        assert_eq!(result, expected, "The area of {:?} was {} instead of {}", rect1, result, expected);
    }
}