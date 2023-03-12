# rstk - in progress

A Rust binding for the Tk graphics toolkit.

## Overview

rstk opens and communicates with Tk's wish program as a separate process.
The library provides:

* low-level functions to directly communicate with wish, suitable for 
  writing additional extensions
* high-level API to write GUI applications with minimal knowledge of Tk.

## Example

A simple hello-world example:

```
use rstk::*;

fn main() {
  let root = rstk::start_wish().unwrap();

  let hello = rstk::make_label(&root);
  hello.text("Hello from Rust/Tk");

  hello.grid().layout();

  rstk::mainloop();
}
```

## MIT License

Copyright (c) 2021, Peter Lane <peterlane@gmx.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

