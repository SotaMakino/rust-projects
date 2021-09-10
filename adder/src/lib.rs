#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let l = Rect {
            width: 10,
            height: 10,
        };
        let s = Rect {
            width: 5,
            height: 5,
        };

        assert!(l.can_hold(&s));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let l = Rect {
            width: 10,
            height: 10,
        };
        let s = Rect {
            width: 5,
            height: 5,
        };

        assert!(!s.can_hold(&l));
    }

    #[test]
    fn add_two_test() {
        assert_eq!(add_two(5), 7);
        assert_ne!(6, add_two(0));
    }

    #[test]
    fn greeting_test() {
        let result = greeting("jack");
        assert!(
            result.contains("jack"),
            "there is no name, the value is {}",
            result
        );
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}
