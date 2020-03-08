pub fn hello_world() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
