[![HitCount](http://hits.dwyl.com/pepsi/v8test.svg)](http://hits.dwyl.com/pepsi/v8test)
![Lines of Code](https://tokei.rs/b1/github/pepsi/v8test?category=code)

`v8test` (Future `key`) is an experiment im doing with rust. Trying to make a small time node/deno "clone".
If you have any code improvements, or general ideas feel  free to make a pull request/issue respectively.


Current plans for std

std
===

* [X] println(...data: Any[]) => void

* [X] print(...data: Any[]) => void

* [X] assert(assertion, message) => void



std.file
====
* [X] read()           => Promise<String>
* [ ] write(content)   => Promise<void>
* [ ]  append(content)  => Promise<void>
* [ ] open() => File
* [ ] exists(filename) => Promise<boolean>
