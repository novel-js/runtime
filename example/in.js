import {println} from "https://novel-js.github.io/pkgs/std/std/println.js";

import std from "https://novel-js.github.io/pkgs/std/std/dist.js";
std.file.write("test.txt", "test file")
std.file.append("test.txt", "A new line!");
std.file.exists("test.txt").then(exi => {
    std.print(exi)
})
