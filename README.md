[![HitCount](http://hits.dwyl.com/pepsi/v8test.svg)](http://hits.dwyl.com/pepsi/v8test)
![Lines of Code](https://tokei.rs/b1/github/pepsi/v8test?category=code)
![build](https://github.com/novel-js/runtime/workflows/build/badge.svg)
![Run tests](https://github.com/novel-js/runtime/workflows/Run%20tests/badge.svg)
![Clippy check](https://github.com/novel-js/runtime/workflows/Clippy%20check/badge.svg)
`Novel-js` is an experiment im doing with rust. Trying to make a small time node/deno "clone".
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


Cli
======
novel run \<file\>
  - Run \<file\> with the novel runtime.

novel clean
  - Clears cache, so on next `run`, all packages are refetched

novel help
  - Prints a help menu



std.file ✖ ✔
====
| Status       | Function name    | Args     | Returns  |
| -------------: | -----------: | :---------- | :---: |
|  ✔             | read   | filePath           | Promise\<String\> |
|  ✔             | write   | filePath, content |  undefined |
|  ✔            | append   | filePath, content |  undefined |
|  ✔           | exists   | filePath |  Promise<boolean> |
|  ✔           | delete   | filePath |  undefined |
|  ✖           | copy   | source, destinatioon |  Promise |
|  ✖           | mkdir   | pathname, recursive? |  Promise |
|  ✖           | rename/move   | oldName, newName |  Promise |


std.http
========
| Status       | Function name    | Args     | Returns  |
| -------------: | -----------: | :---------- | :---: |
|  ✖             | get   | url, options           | Promise\<Response\> |
|  ✖             | post   | url, options           | Promise\<Response\> |
|  ✖             | put   | url, options           | Promise\<Response\> |

- Maybe a http library at some point?