os
----

[![Travis Build Status](https://travis-ci.org/camp4climber/os.svg?branch=master)](https://travis-ci.org/camp4climber/os)
[![crates.io](https://img.shields.io/crates/v/os.svg)](https://crates.io/crates/os)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A rust library for various os utilities.

Not for production use yet!

The module uses the following command line tools and assumes they are installed on your system.

- uname

Example
-------

Get the kernel name and os name.

```
extern crate os;

fn main() {
    let os_info = os::get_info();
    println!("{:?}", os_info);
}
```
