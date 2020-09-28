import std from 'https://novel-js.github.io/pkgs/std/std/dist.js'


class Option {
  constructor (inner) {
    this.inner = inner
    this.isSome = false
    this.isNone = false
  }

  isSome () {
    return this.isSome
  }

  isNone () {
    return this.isNone
  }
}
class None extends Option {
  constructor () {
    super(null)
    this.is_none = true
  }

  match (obj) {
    obj.none()
  }

  unwrap () {
    throw new Error('Could not unwrap() on a None() value')
  }
  // match(...x) {
  //   super().match(...x)
  // }
}
class Some extends Option {
  constructor (inner) {
    super(inner)
    this.is_some = true
  }

  match (obj) {
    obj.some(this.inner)
  }

  unwrap () {
    return this.inner
  }
}
const something = new Some(2)
// const a = 6
// let something
// if (a === 6) {
//   something = new Some(16)
// } else {
//   something = new None()
// }
something.match(
  {
    some: function (s) {
      std.println('Something is some with variable s', s)
    },
    none: function () {
      std.println('Something is none with no variagble')
    }
  }
)
