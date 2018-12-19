trait Apply {
    fn apply(&self, i: i32) -> String;
}

trait Rule {
    fn resolve(&self, i: i32) -> Option<String>;
}

struct Stringify;

impl Rule for Stringify {
    fn resolve(&self, i: i32) -> Option<String> {
        Some(format!("{}", i))
    }
}

struct StringIfDivisible {
    n: i32,
    s: &'static str
}

impl Rule for StringIfDivisible {
    fn resolve(&self, i: i32) -> Option<String> {
        if i % self.n == 0 {
            Some(format!("{}", self.s))
        }
        else{
            None
        }
    }
}

struct FizzBuzz;

impl Rule for FizzBuzz {
    fn resolve(&self, i: i32) -> Option<String> {
        if i % 5 == 0 && i % 3 == 0 {
            Some(format!("{}", "FizzBuzz"))
        }
        else{
            None
        }
    }
}

impl<A: Rule> Apply for A {
    fn apply(&self, i: i32) -> String {
        self.resolve(i).unwrap()
    }
}

impl<A: Rule, B: Rule> Rule for (A, B) {
    fn resolve(&self, i: i32) -> Option<String> {
        self.0.resolve(i).or(self.1.resolve(i))
    }
}

pub fn fizzbuzz() -> impl Apply {
    (FizzBuzz, (StringIfDivisible{n: 3, s: "Fizz"}, (StringIfDivisible{n: 5, s: "Buzz"}, Stringify)))
}

fn main() {
    let fb = fizzbuzz();

    for i in 1..101 {
        println!("{}", fb.apply(i));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn otherwise_should_return_the_number() {
        let fb = fizzbuzz();

        assert_eq!("2", &fb.apply(2));
        assert_eq!("8", &fb.apply(8));
    }

   #[test]
   //#[should_panic]
   fn should_return_fizz_for_multiple_of_3() {
       let fb = fizzbuzz();

       assert_eq!("Fizz", &fb.apply(3));
       assert_eq!("Fizz", &fb.apply(36));
   }

   #[test]
   //#[should_panic]
   fn should_return_buzz_for_multiple_of_5() {
       let fb = fizzbuzz();

       assert_eq!("Buzz", &fb.apply(5));
       assert_eq!("Buzz", &fb.apply(25));
   }

   #[test]
   //#[should_panic]
   fn should_return_fizzbuzz_for_multiple_of_3_and_5() {
       let fb = fizzbuzz();

       assert_eq!("FizzBuzz", &fb.apply(15));
       assert_eq!("FizzBuzz", &fb.apply(45));
   }
}
