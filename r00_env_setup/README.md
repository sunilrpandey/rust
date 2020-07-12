# RUST - _A new one_

## What is RUST?
Rust, originally designed by Graydon Hoare at Mozilla Research, is a multi-paradigm programming language which is fast, memory efficient with no runtime or garbage collector, can run on embedded systems and can easily integrate with other language.

## Let us install it first
For now, my favorite platform WSL

` curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh `

After installation to PATH, which has binaries such as rustc, cargo, and rustup.

` export PATH="$HOME/.cargo/bin:$PATH"`

And source the environemnt 

`source ~/.cargo/env`

Check the version

`rustc --version`
## Are we ready???
To test if we are ready to explore, create a `hello_world.rs` and type 
```
fn main()
{
    println!("Hello Rust!!");

}
```
and compile 

`rustc hello_rust.rs`

Which will give you hello_rust.exe, execute the binary to seee the output 
```
run ./hello_rust.exe
Hello Rust!!
```

## Dont we have make?? We have Cargo, the package manager
### What Cargo does exactly?
Cargo downloads your Rust package’s dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io, the Rust community’s package registry. 

Generally we create hello-rust.rs inside `src` folder, come out of src and create Cargo.toml (manifest file,  it contains all of the metadata that Cargo needs to compile your package.)and write
```
[package]
name = 'hello_rust'
version = "0.0.1"
authors = ["Your Name<email id>"]
```
into it and run following command from the command line 
> cargo build

to compile.On Successful compilation you will notice target folder, run the executable to see the result
> ./target/debug/hello_rust

Do compile run in one step by 
>cargo run

However `cargo` helps us doing everything in one go, execute
> cargo new hello-rust --bin

--bin is default, we can give --lib as well
Above will create hello_rust directory which will have Cargo.toml and src direatory and main.rs inside src.
Well, Now go to newly created directory(hello_rust in our case, also it is the directory which has Cargo.toml) and run 
```
cargo build --release
cargo run // Now we will have exe inside target/release
```

### But, what is this Cargo.lock? Which I see along with Cargo.toml?
Well, it contains dependency info. Suppose we need regex in our source file. we need to add below in Cargo.toml
```
[dependencies]
regex = "0.1.41"
```
 `cargo build` will fetch all the dependencies, build it and update `Cargo.lock`(detailed one, should not be edited manually)
 
 If there is any update this is how Cargo updates itself and its dependent modules
 ```
 $ cargo update    # updates all dependencies
$ cargo update -p rand   # updates just “rand”
 ```

### A look at Cargo Package layout
```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs

```
In above structure, 
- Benchmarks go in the benches directory.
- Examples go in the examples directory.
- Integration tests go in the tests directory.

## What bout Test framework?
Here we have two kind of tests, Cargo checks unit tests under `src ` directory whereas checks integration tests under `test` directory. And here is the command
```
cargo test // for all the tests
cargo test specific_test 
```

## More to come ...

## References

[RUST the org](http://rust-lang.org)

[Rust Online Editor](play.rust-lang.org)

[Install Path](https://www.rust-lang.org/tools/install)

[Cargo](https://doc.rust-lang.org/cargo/index.html)
