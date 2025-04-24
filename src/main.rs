fn main() {
    println!("Hello, rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        assert_eq!(1 + 1, 2);
    }
}
