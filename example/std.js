// import $ from "./$";
function println (...a) {
  $.println(...a)
}
function print (...a) {
  $.print(...a)
}
class File {
  constructor (filename) {
    this.filename = filename
    }

  read () {
    return $.file_read(this.filename)
  }

  write () {

  }
}
const file = {
  open (filename) { // => Promise<File>
    return new File(filename)
    }

}

export { println, print, file }
