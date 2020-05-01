// https://rosettacode.org/wiki/Pi#Rust
// https://rust-num.github.io/num/num/struct.BigInt.html
// https://doc.rust-lang.org/stable/rust-by-example/trait/iter.html
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
// http://www.cs.ox.ac.uk/people/jeremy.gibbons/publications/spigot.pdf
// https://en.wikipedia.org/wiki/Spigot_algorithm
// https://www.quora.com/How-is-the-next-digit-of-pi-calculated
// https://doc.rust-lang.org/std/iter/trait.Iterator.html
extern crate num_bigint;
use num_bigint::BigInt;
use std::char;

#[derive(Debug)]
struct PiIterator {
    given_first: bool,
    given_second: bool,
    q: BigInt,
    r: BigInt,
    t: BigInt,
    k: BigInt,
    n: BigInt,
    l: BigInt,
}

impl Default for PiIterator {

    #[inline]
    fn default() -> PiIterator {
        PiIterator{
            given_first: false,
            given_second: false,
            q: BigInt::from(1),
            r: BigInt::from(0),
            t: BigInt::from(1),
            k: BigInt::from(1),
            n: BigInt::from(3),
            l: BigInt::from(3),
        }
    }
}

impl PiIterator {

    fn has_found_digit(&self) -> bool {
        &self.q * 4 + &self.r - &self.t < &self.n * &self.t
    }

    fn has_not_found_digit(&self) -> bool {
        !self.has_found_digit()
    }
}

impl Iterator for PiIterator {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if self.given_first && !self.given_second {
            self.given_second = true;
            return Option::from('.');
        }
        while self.has_not_found_digit() {
            let nr = (&self.q * 2 + &self.r) * &self.l;
            let nn = (&self.q * &self.k * 7 + 2 + &self.r * &self.l) / (&self.t * &self.l);
            self.q *= &self.k;
            self.t *= &self.l;
            self.l += 2;
            self.k += 1;
            self.n = nn;
            self.r = nr;
        }
        if !self.given_first {
            self.given_first = true;
        }
        let value = format!("{}", self.n);
        let nr = (&self.r - &self.n * &self.t) * 10;
        self.n = (&self.q * 3 + &self.r) * 10 / &self.t - &self.n * 10;
        self.q *= 10;
        self.r = nr;
        if !self.given_first {
            self.given_first = true
        }
        return value.chars().next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn three_digits() {
        let expected_value = "3.14".to_owned();
        let pi_iterator = PiIterator::default();
        let actual_value: String = pi_iterator.take(4).collect();
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn some_digits() {
        let expected_value = "3.1415926535897932384626433832795028841971693".to_owned();
        let pi_iterator = PiIterator::default();
        let actual_value: String = pi_iterator.take(expected_value.len()).collect();
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    // #[ignore]
    fn many_digits() {
        let expected_value: String = vec!(
            "3.1415926535897932384626433832795028841971693",
            "993751058209749445923078164062862089986280348",
            "253421170679821480865132823066470938446095505",
            "822317253594081284811174502841027019385211055",
            "596446229489549303819644288109756659334461284",
            "756482337867831652712019091456485669234603486",
            "104543266482133936072602491412737245870066063",
            "155881748815209209628292540917153643678925903",
            "600113305305488204665213841469519415116094330",
            "572703657595919530921861173819326117931051185",
            "480744623799627495673518857527248912279381830",
            "119491298336733624406566430860213949463952247",
            "371907021798609437027705392171762931767523846",
            "748184676694051320005681271452635608277857713",
            "427577896091736371787214684409012249534301465",
            "495853710507922796892589235420199561121290219",
            "608640344181598136297747713099605187072113499",
            "999983729780499510597317328160963185950244594",
            "553469083026425223082533446850352619311881710",
            "100031378387528865875332083814206171776691473",
            "035982534904287554687311595628638823537875937",
            "5195778185778053217",
        ).iter().map(|s| s.to_owned()).collect();
        
        let pi_iterator = PiIterator::default();
        let actual_value: String = pi_iterator.take(expected_value.len()).collect();
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn rosetta_code() {
        // https://rosettacode.org/wiki/Pi#Rust
        let expected_length: usize = 256;
        let expected_value = calc_pi(expected_length);
        let pi_iterator = PiIterator::default();
        let actual_value: String = pi_iterator.take(expected_length).collect();
        assert_eq!(expected_value, actual_value);
    }

    fn calc_pi(length: usize) -> String {
        let mut result_builder: Vec<String> = Vec::default();

        let mut q = BigInt::from(1);
        let mut r = BigInt::from(0);
        let mut t = BigInt::from(1);
        let mut k = BigInt::from(1);
        let mut n = BigInt::from(3);
        let mut l = BigInt::from(3);
        let mut first = true;
        while result_builder.len() < length {
            if &q * 4 + &r - &t < &n * &t {
                result_builder.push(format!("{}", n));
                if first {
                    result_builder.push(".".to_owned());
                    first = false;
                }
                let nr = (&r - &n * &t) * 10;
                n = (&q * 3 + &r) * 10 / &t - &n * 10;
                q *= 10;
                r = nr;
            } else {
                let nr = (&q * 2 + &r) * &l;
                let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
                q *= &k;
                t *= &l;
                l += 2;
                k += 1;
                n = nn;
                r = nr;
            }
        }

        result_builder.join("")
    }
}
