/// Simple function
fn f() -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::f; // access to non-public members.
    #[test]
    fn ff() {
        assert_eq!(f(), 0);
    }
}
