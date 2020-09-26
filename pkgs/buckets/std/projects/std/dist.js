import {println} from "http://pkgs.io:3030/pkg/std/std/file/println.js"
import {print} from "http://pkgs.io:3030/pkg/std/std/file/print.js"
import {read} from "http://pkgs.io:3030/pkg/std/std/file/read.js"

const file = {
    read: read
}
export {println, print, file}
// export {file};