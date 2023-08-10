## Some examples

#### Building

The Rust .a files are build <i>these do not need any downstream .a's during their building but will need these during executable building</i>

#### Testing

There are two profiles `dev` and `test_handler`

`dev` uses the C library a file

`test_handler` uses the Rust library that exposes the same things. This would be useful if someone were to be mocking a C library in Rust