fn is_valid_password(number: usize) -> bool {

    // ensure number is 6 digits
    if number > 999999 || number < 100000 {
        return false;
    }

    let mut has_double: bool = false;
    let mut current_number: usize = number;
    let mut prev_digit: u8 = 10; // not possible to have a 10 as digit
    let mut curr_digit;          // and it is the greater than all digits

    loop {
        // if no digits left to read
        if current_number == 0 {
            break;
        }

        // get current digit and progress
        curr_digit = (current_number % 10) as u8;
        current_number /= 10;
        // if two digits in a row it has a double
        if !has_double && curr_digit == prev_digit {
            has_double = true;
        }

        // check decending
        // (never decrease going left to right means that
        // they never increase going right to left)
        if curr_digit > prev_digit {
            return false;
        }

        // set previous
        prev_digit = curr_digit;
    }

    // if has double
    return has_double;
}

fn main() {

    let mut count: usize = 0;
    for n in 347312..805915 {
        if is_valid_password(n) {
            count += 1;
        }
    }

    println!("{}", count);
}
