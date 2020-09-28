import std from "https://novel-js.github.io/pkgs/std/std/dist.js";
let t = false;
std.file.write("read_write_test.txt", "hello world").then(() => {
    std.file.read("read_write_test.txt").then(content => {
        if(content == "hello world"){
            t = true;
        }
    })
});

if(t){
    throw ''
}

