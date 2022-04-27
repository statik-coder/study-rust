#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_five(a: i32) -> i32 {
    return a + 5;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_should_hold_smaller() {
        let large = Rectangle {
            width: 12,
            height: 10,
        };

        let small = Rectangle {
            width: 5,
            height: 3,
        };

        assert!(large.can_hold(&small));
    }

    #[test]
    fn smaller_shouldnot_hold_larger() {
        let larger = Rectangle {
            width: 12,
            height: 10,
        };

        let small = Rectangle {
            width: 5,
            height: 3,
        };

        assert!(!small.can_hold(&larger));
    }

    #[test]
    fn should_add_five() {
        assert_eq!(add_five(1), 6);
    }
}
