/*
Conjecture: Take any number, reverse its digits and add it to the original number.
Repeat this process and you will eventually produce a palindrome.

This is an open problem. 196 is the smallest number for which a palindrome hasn't been found through this iterative process.

  13
+ 31
----
  44
*/

use num_bigint::BigInt;


fn reverse(n: &BigInt) -> BigInt {
    let mut n = (*n).clone();
    let mut rev = BigInt::from(0);
    let zero = BigInt::from(0);
    let ten = BigInt::from(10);
    while n > zero {
        rev = rev * 10 + n.clone() % 10;
        n = n.checked_div(&ten).unwrap();
    }
    rev
}

fn next_palindrome(n: BigInt) -> (BigInt, bool) {
    let rev = reverse(&n);
    if n == rev {
        (n, true)
    } else {
        (n + rev, false)
    }
}

fn main() {
    let mut n = BigInt::from(196);
    let mut is_palindrome = false;
    let mut count = 0;
    while !is_palindrome {
        let (next, palindrome) = next_palindrome(n);
        n = next;
        is_palindrome = palindrome;
        count += 1;
        if count % 1000 == 0 {
            println!("{}: {}", count, n);
        }
    }
    println!("The next palindrome is {}", n);
}


// Extensive tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse() {
        assert_eq!(super::reverse(0), 0);
        assert_eq!(super::reverse(1), 1);
        assert_eq!(super::reverse(12), 21);
        assert_eq!(super::reverse(123), 321);
        assert_eq!(super::reverse(1234), 4321);
        assert_eq!(super::reverse(12340), 4321);
    }
}
