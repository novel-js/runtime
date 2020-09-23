function main(){
    let x = 5;
    let y = 9;
    std.assert(x == y, "ABC");
    std.errln("A red error");
    std.println("Next print isnt red");
    let f = std.format("test", "a", "b")
    std.print(f);
}
main();