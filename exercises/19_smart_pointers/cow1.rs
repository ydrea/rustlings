// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let value = input[i];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -value;
        }
    }input
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(vec) => Ok(assert_eq!(*vec, vec![1, 0, 1])),
            _ => Err("Expected owned value"),
        }
    }
    #[test]
    
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(vec) => Ok(assert_eq!(*vec, vec![0, 1, 2])),
            _ => Err("Expected owned value"),
        }
    }
    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
            Cow::Borrowed(vec) => Ok(assert_eq!(*vec, vec![0, 1, 2])),
            _ => Err("Expected borrowed value"),
        }
    }
}
