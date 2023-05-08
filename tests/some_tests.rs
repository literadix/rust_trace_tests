#[derive(Debug, Copy, Clone, PartialEq)]
struct Centimeters(u64);

// This function will ne used as function pointer
fn add_one(x: Centimeters) -> Centimeters {
    x + Centimeters(1)
}

/// Define max function
fn max(x: Centimeters, y: Centimeters) -> Centimeters {
    if x.0 > y.0 {
        x
    } else {
        y
    }
}

/// Define add operation for Centimeter
impl ::core::ops::Add for Centimeters {
    type Output = Centimeters;
    fn add(self, rhs: Centimeters) -> Centimeters {
        Centimeters(self.0 + rhs.0)
    }
}

/// Define minus operation for Centimeter
impl ::core::ops::Sub for Centimeters {
    type Output = Centimeters;
    fn sub(self, rhs: Centimeters) -> Centimeters {
        Centimeters(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod test {
    use super::*; // access to non-public members.
    #[test]
    fn test_function_pointer() {
        let ptr: fn(Centimeters) -> Centimeters = add_one;
        let before = Centimeters(5);
        let wanted = Centimeters(6);

        assert_eq!(ptr(before), wanted);
        assert_eq!(ptr(before) + before, Centimeters(11));
    }

    #[test]
    fn test_addition() {
        assert_eq!(Centimeters(5) + Centimeters(1), Centimeters(6));
    }

    #[test]
    fn test_substraction() {
        assert_eq!(Centimeters(5) - Centimeters(1), Centimeters(4));
    }

    #[test]
    fn test_max() {
        assert_eq!(max(Centimeters(1), Centimeters(2)), Centimeters(2));
    }

    #[test]
    fn test_iter() {
        let mut numbers = vec![1, 2, 3];
        for num in numbers.iter() {
            println!("{}", num)
        }
        for num in numbers.iter_mut() {
            println!("{}", num)
        }
    }
}
