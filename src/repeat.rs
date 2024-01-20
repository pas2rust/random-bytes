pub fn repeat(input: &mut String, add: &mut u32, breaker: u32, push: char) {
    loop {
        *add += 1;
        match *add == breaker || breaker == 0 {
            true => break,
            false => input.push(push),
        }
    }
}
