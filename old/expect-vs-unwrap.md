Expect vs unwrap: A tale of writing good code

In Rust, both unwrap() and expect() are methods that we use to extract values from Option
or Result types. If the value is None the program will panic. However their usage in production code is generally
discouraged unless justified circumstances.

There are 4 levels of writing a rust program:
1. Using unwrap()
2. Using expect()
3. Using expect() with should!
4. Using custom error types and handling every error properly

* unwrap() panics with a default message when called on a None or Err value. It's concise but provides limited context,
making debugging more challenging
* expect() is similar to unwrap() but allows you to provide a custom panic message. This additional context can be a 
very invaluable tool for debugging, as it clarifies why the panic occured.

According to "The Rust Programming Language" book, using unwrap() or expect() is acceptable in certain situations:

a) Protoyping and Testing: during the initial development phase or in test code, where quick iterations are needed and
robust error handling isn't yet implemented

b) Examples and Tutorials: to keep code examples concise and focused on the primary concept being taught

c) When Failure is impossible: In cases where logic guarantees that an Option is Some or a Result is Ok, and thus
panicking would indicate a bug. For example:

```rust
  let ip: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
```
Here, since the IP address is hardcoded and valid, using expect() is reasonable

In production, it's better to handle errors gracefully using pattern matching or the "?" operator, which propagates
errors to the caller.

Use expect() for Clarity: When you are certain that a value is valid and want to document this assumption, expect()
with a clear message is preferable. It serves both as documentation and as a safeguard, should the assumption prove 
incorrect in the future.



