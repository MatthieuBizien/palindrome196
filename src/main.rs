/*
Conjecture: Take any number, reverse its digits and add it to the original number.
Repeat this process and you will eventually produce a palindrome.

This is an open problem. 196 is the smallest number for which a palindrome hasn't been found through this iterative process.

  13
+ 31
----
  44
*/

use std::vec;

struct BigDec {
    digits: vec::Vec<u8>,
}

impl BigDec {
    fn new() -> BigDec {
        return BigDec { digits: vec::Vec::new() };
    }

    fn reverse(&self) -> BigDec {
        let digits = self.digits.iter().rev().cloned().collect();
        return BigDec { digits: digits };
    }
}

// Display is simply the digits
impl ::std::fmt::Display for BigDec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut result = String::new();
        for digit in self.digits.iter() {
            result.push_str(&digit.to_string());
        }
        return write!(f, "{}", result);
    }
}

impl ::std::convert::Into<BigDec> for u128 {
    fn into(self) -> BigDec {
        let mut n = self;
        let mut result = vec::Vec::new();
        while n > 0 {
            result.push((n % 10) as u8);
            n /= 10;
        }
        return BigDec { digits: result };
    }
}

impl PartialEq for BigDec {
    fn eq(&self, other: &BigDec) -> bool {
        return self.digits == other.digits;
    }
}


fn add(n: &Vec<u8>, m: &Vec<u8>) -> Vec<u8> {
    let mut carry = 0;
    let mut result = Vec::new();
    let mut n = n.clone();
    let mut m = m.clone();
    while n.len() > 0 || m.len() > 0 {
        let mut sum = carry;
        if n.len() > 0 {
            sum += n.pop().unwrap();
        }
        if m.len() > 0 {
            sum += m.pop().unwrap();
        }
        carry = sum / 10;
        result.push(sum % 10);
    }
    if carry > 0 {
        result.push(carry);
    }
    return result;
}


impl ::std::ops::Add for BigDec {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return BigDec {
            digits: add(&self.digits, &other.digits),
        };
    }
}


fn next_palindrome(n: BigDec) -> (BigDec, bool) {
    let rev = n.reverse();
    if n == rev {
        (n, true)
    } else {
        (n + rev, false)
    }
}

fn main() {
    let mut n = BigDec::from(196.into());
    let mut is_palindrome = false;
    let mut count = 0;
    while !is_palindrome {
        let (next, palindrome) = next_palindrome(n);
        n = next;
        is_palindrome = palindrome;
        count += 1;
        if count % 10_000 == 0 {
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
        assert_eq!(BigDec::from(0).reverse(), BigDec::from(0));
        assert_eq!(BigDec::from(1).reverse(), BigDec::from(1));
        assert_eq!(BigDec::from(12).reverse(), BigDec::from(21));
        assert_eq!(BigDec::from(123).reverse(), BigDec::from(321));
        assert_eq!(BigDec::from(1234).reverse(), BigDec::from(4321));
        assert_eq!(BigDec::from(12340).reverse(), BigDec::from(4321));
    }
}
