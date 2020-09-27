[![HitCount](http://hits.dwyl.com/pepsi/v8test.svg)](http://hits.dwyl.com/pepsi/v8test)
![Lines of Code](https://tokei.rs/b1/github/pepsi/v8test?category=code)
![build](https://github.com/novel-js/runtime/workflows/build/badge.svg)
![Run tests](https://github.com/novel-js/runtime/workflows/Run%20tests/badge.svg)
![Clippy check](https://github.com/novel-js/runtime/workflows/Clippy%20check/badge.svg)
`Nonvel-js` is an experiment im doing with rust. Trying to make a small time node/deno "clone".
If you have any code improvements, or general ideas feel  free to make a pull request/issue respectively.


Current plans for std

std
===

* [X] println(...data: Any[]) => void
  - Print `...data` joined by a space, ended with ``\n``.
  * [X] TODO: Support deep printing JSON objects.
* [X] print(...data: Any[]) => void
  - Print `...data` joined by a space.
  * [X] TODO: Support deep printing JSON objects.
* [X] assert(assertion, message) => void
  - Check `assertion == true`, if its not, print `message`. 



std.file
====
| Status       | Function name    | Args     | Returns  |
| :------------- | :----------: | :----------: | :---: |
|  ✔             | read   | filename           | Promise\<String\> |
|  ✔             | write   | filename, content |  undefined |
|  ✔            | append   | filename, content |  undefined |
|  ✔           | exists   | filename |  Promise<boolean> |

std.http
========
- I dont have many plans for http, But soon I want a basic `get` function.
