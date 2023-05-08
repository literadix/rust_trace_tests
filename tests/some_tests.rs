#[derive(Debug, Copy, Clone, PartialEq)]
struct Centimeters {
    val: u64,
}

// This function will ne used as function pointer
fn add_one(x: Centimeters) -> Centimeters {
    x + Centimeters { val: 1 }
}

/// Define max function
fn max(x: Centimeters, y: Centimeters) -> Centimeters {
    if x.val > y.val {
        x
    } else {
        y
    }
}

/// Define add operation for Centimeter
impl ::core::ops::Add for Centimeters {
    type Output = Centimeters;
    fn add(self, rhs: Centimeters) -> Centimeters {
        Centimeters {
            val: self.val + rhs.val,
        }
    }
}

/// Define minus operation for Centimeter
impl ::core::ops::Sub for Centimeters {
    type Output = Centimeters;
    fn sub(self, rhs: Centimeters) -> Centimeters {
        Centimeters {
            val: self.val - rhs.val,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*; // access to non-public members.
    #[test]
    fn test_function_pointer() {
        let ptr: fn(Centimeters) -> Centimeters = add_one;
        let before = Centimeters { val: 5 };
        let wanted = Centimeters { val: 6 };

        assert_eq!(ptr(before), wanted);
        assert_eq!(ptr(before) + before, Centimeters { val: 11 });
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            Centimeters { val: 5 } + Centimeters { val: 1 },
            Centimeters { val: 6 }
        );
    }

    #[test]
    fn test_substraction() {
        assert_eq!(
            Centimeters { val: 5 } - Centimeters { val: 1 },
            Centimeters { val: 4 }
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            max(Centimeters { val: 1 }, Centimeters { val: 2 }),
            Centimeters { val: 2 }
        );
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

    #[test]
    fn test_slices() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let num_slice = &mut nums[..]; // make a slice out of the Vector

        for num in num_slice.iter() {
            println!("{}", num)
        }
        for num in num_slice.iter_mut() {
            println!("{}", num)
        }
    }
}
