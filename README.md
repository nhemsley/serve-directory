Serve Directory
===============

Allows the user to serve a directory statically by launching a single executable.

This project is a simple web server used to serve files in the current working directory. For 
instance, the following command could be used to serve all files in the current directory and its
children. 

```bash
$ serve-directory
```

The idea for this project is based off of the excellent [serve](https://www.npmjs.com/package/serve)
application for NodeJS. While this application was primarily built as a personal challenge, it does
provide some value. While `serve` is quite convenient, it requires NodeJS to be installed. Because
this project is implemented in Rust, the compiled native binary can be run without any dependencies
or complicated installation.

If the user browses to a valid file, then that file will be served. Folders display a custom HTML 
page allowing the user to traverse the filesystem.

## Usage
This application is made to be relatively easy to use, and is controlled by simple command line 
arguments. 

| Argument | Short Flag | Long Flag   | Description                                 |
| :------- | :--------- | :---------- | :------------------------------------------ |
| Help     | `-h`       | `--help`    | Print help information                      |
| Port     | `-p`       | `--port`    | The port used by the server [default: 8080] |
| Version  | `-V`       | `--version` | Prints version information                  |

You can specifying which folder will be served as the root directory by listing it as an argument.
For example, to serve the `~/www/` folder on port `3000`:

```bash
$ serve-directory -p 3000 ~/www/
```

## Acknowledgment
A special thank you to the [serve](https://www.npmjs.com/package/serve) project for providing
inspiration for this tool!

This project is made possible by the work of the following great projects:
| Crate                                                         | Owners                      |
| :------------------------------------------------------------ | :-------------------------- |
| [build_html](https://crates.io/crates/build_html)             | Joseph Skubal               |
| [env_logger](https://crates.io/crates/env_logger)             | The Rust Project Developers |
| [lazy_static](https://crates.io/crates/lazy_static)           | The Rust Libs Team          |
| [local-ip-address](https://crates.io/crates/local-ip-address) | Esteban Borai               |
| [log](https://crates.io/crates/log)                           | The Rust Project Developers |
| [structopt](https://crates.io/crates/structopt)               | Guillaume Pinot             |
| [tokio](https://crates.io/crates/tokio)                       | Tokio Contributors          |
| [warp](https://crates.io/crates/warp)                         | Sean McArthur               |

And, of course, the [Rust language](https://rust-lang.org)!

## License
This project is licensed under the [MIT license](https://mit-license.org).

Copyright (C) 2020-21 Joseph Skubal