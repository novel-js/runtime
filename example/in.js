import {println} from "https://novel-js.github.io/pkgs/std/std/println.js";
/// <reference path="./" />
import std from "https://novel-js.github.io/pkgs/std/std/dist.js";
// import test from "https://novel-js.github.io/pkgs/std/testing/dist.js";

(async function(){
    // synx err;
    // const a = 5;
    // let a =  b;
    std.file.write("test.txt", "test file")
    std.file.append("test.txt", "A new line!");
    // std.file.exists("test.txt").then(exi => {
    //     std.print(exi)
    // })
    let exi = await std.file.exists("test.txt");
    std.println(exi)
    let a = {
        a: "Letter a",
    }
    let b = {
        "c": a
    }
    std.println(b)
    std.file.write("/test.txt", "test").then(c => {
        std.println(c)
    }).catch(e => {
        std.println("err", e)
    })
    std.println(5/0)
})()
// import { print } from "../pkgs/buckets/std/projects/std/dist";
