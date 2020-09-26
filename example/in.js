/// <reference>
import {println, print, file} from "http://pkgs.io:3030/pkg/std/std";

file.read("example/test.txt").then(content => {
    print("content", content)
}).catch(err => {
    print("err", err)
})
// import {read} from "http://pkgs.io:3030/pkg/std/file";
// println(read)
