fn BasicFunction() {
    println!("This Function should be detected by Clippy")
}


fn main() {
    let foo: i32 = 90;
    let bar: i32 = 10;
    sum(foo, bar);
    println!("Hello, world!");
}

fn sum(a: i32, b:i32) -> i32 {
    return a + b;
} 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_add() {
        let result = sum(10, 20);
        assert_eq!(result, 30)
    }
}
