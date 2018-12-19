# THANKS TO [Rust Language Milano](https://www.meetup.com/it-IT/rust-language-milano/) Group 

# Open Closed Principle FizzBuzz Kata

## Master Rule

Please, don't kid yourself... You can always find a way to
turn around the rules and build a non Open Closed solution.

## Rules

1. You can make test pass just by change `fizzbuzz()` factory.
2. `fizzbuzz()` factory cannot contains logic or conditional statements.
3. `fizzbuzz()` should just create objects and link them. You can retrun
a `struct` or an other aggregate but should be something that implement
`Apply` trait
4. While you're refactoring all tests should not change states
5. Use `#[should_panic]` to make sure that you don't implements
a new feature while refactoring

## Mechanics

1. Implement `Apply` for something that make the first test pass
and return it instead of `DefaultApply`
2. Uncomment next test and check if fail
3. Uncomment `#[should_panic]` to make it pass
4. **Refactor** your code to accept the new feature without
introduce it (don't forget to run tests).
5. Change `fizzbuzz()` factory and remove `#[should_panic]` to
check the new feature.
6. If tests pass
  - yes: if there are more features
    - yes: goto 2
    - no: **DONE**
  - no: Recover `#[should_panic]`, `fizzbuzz()` ad go back to 4

## Hints

You can:

- Use a builder that compose the rules
- Implement `Apply` for pair or `Rule`
- Implement `Apply` a new struct that own a vector of `Rule` trait objects
- Use closures
- ...
