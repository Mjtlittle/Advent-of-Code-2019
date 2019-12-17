fn is_valid_password(number: usize) -> bool {

    // ensure number is 6 digits
    if number > 999999 || number < 100000 {
        return false;
    }

    let mut has_double: bool = false;
    let mut curr_double_count: u8 = 0;
    let mut current_number: usize = number;
    let mut prev_digit: u8 = 10; // not possible to have a 10 as digit
    let mut curr_digit: u8;      // and it is the greater than all digits

    loop {
        // if no digits left to read
        if current_number == 0 {
            break;
        }

        // get current digit and progress
        curr_digit = (current_number % 10) as u8;
        current_number /= 10;

        // count double
        if !has_double && curr_digit == prev_digit {
            curr_double_count += 1;
        }
        
        // if number changes check to see if the double count is valid
        // if not reset it
        if curr_digit != prev_digit {
            if curr_double_count == 1 {
                has_double = true;
            } else {
                curr_double_count = 0;
            }
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

    // edge case (ex. 112222): recheck last digit
    // (as it will not get picked up as there is not a digit after it)
    if !has_double && curr_double_count == 1 {
        has_double = true;
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
