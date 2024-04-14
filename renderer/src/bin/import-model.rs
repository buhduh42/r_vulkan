#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {
        assert_eq!("1", "1");
    }

    #[test]
    fn another() {
        panic!("it failed");
    }
}

fn main() {
    println!("hello import-model");
}
