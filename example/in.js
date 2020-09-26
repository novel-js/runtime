import {println} from "https://novel-js.github.io/pkgs/std/std/println.js";

import std from "https://novel-js.github.io/pkgs/std/std/dist.js";
(async function(){
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
})()
// import { print } from "../pkgs/buckets/std/projects/std/dist";
