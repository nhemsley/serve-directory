# Serve Directory
Allows the user to serve the current directory by launching a single executable.

This project is a simple web server used to serve files in the current working directory. For 
instance, the following command could be used to serve all files in the current directory and its
children. 
```bash
$ serve-directory
```

Of course, for those not command-line inclined, running the program by clicking on the executable
should provide the exact same functionality

This project is based off of the excellent NodeJS [serve](https://www.npmjs.com/package/serve) 
application. However, while serve is convenient to use, it requires having NodeJS installed. 
Because this project is implemented in Rust, the compiled result is a native binary which can be 
run without any dependencies or installation.

## Acknowledgment
A special thank you to the [serve](https://www.npmjs.com/package/serve) project for providing
inspiration for this tool!

This project is made possible by the work of the following great projects:
* [build_html](https://crates.io/crates/build_html): Joseph Skubal
* [clap](https://crates.io/crates/clap): Kevin K. and Contributors
* [env_logger](https://crates.io/crates/env_logger) -- The Rust Project Developers
* [log](https://crates.io/crates/log) -- The Rust Project Developers
* [tokio](https://crates.io/crates/tokio): Tokio Contributors
* [warp](https://crates.io/crates/warp): Sean McArthur
* [rust-lang](https://rust-lang.org): The Rust Contributors

## License
This project is licensed under the [MIT license](https://mit-license.org). In other words, it's 
free for you to use for whatever purpose you want. However, to the maximum extent allowed under the 
law, this software has NO WARRANTY. 

Copyright (C) 2020-21 Joseph Skubal