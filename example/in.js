async function a(){
    
}

function main(){
    // let x = 5;
    // let y = 9;
    // std.assert(x == y, "ABC");
    // std.errln("A red error");
    // std.println("Next print isnt red");
    // let f = std.format("test", "a", "b", "\n")
    // let f2 = std.formatln("test", "a", "b")
    // std.assert(f == f2, "Something went wrong, `formatln` is not the same as format followed by \"\\n\".")
    // std.callback_test().then(x => {
    //     std.print(x)
    // })
    // f = std.callback_test();
    
    // std.println("Calling")
    // std.callback_test().then(r => std.println(r))
    // std.println("Called")

    std.read("test.txt").then(e => {
        std.println("Successful: ", e);
    }).catch(e => {
        std.println("Error: ", e)
    })
    // std.print(x)
}
main();