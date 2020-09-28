import std from 'https://novel-js.github.io/pkgs/std/std/dist.js'
import {Some, None} from 'https://novel-js.github.io/pkgs/std/option/dist.js'

function somethingThatMightReturnNull(input){
  if(input < 50){
    return new None()
  }else{
    return new Some(input)
  }
}
let s = somethingThatMightReturnNull(20);
let v = somethingThatMightReturnNull(150);

std.println(v.isNone, v.isSome);
std.println(s.isNone, s.isSome);
 (async fvnction(){
   
 })

// std.println(s.unwrap());
// let p = new std.file.path("testing", "testing2");
// std.println(p.toNative())
// const a = 5;
// a = 1;

// std.println(std.file.path)
// std.println($.is_nix())