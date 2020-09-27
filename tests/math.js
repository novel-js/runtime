import std from "https://novel-js.github.io/pkgs/std/std/dist.js"

const a = 5;
const b = 10;
const c = a * b;
const d = b / a;
const e = a - b;
const f = 5/0; // => Infinity
const bucket = a * b +b * c + c * d * d + e * e + f * f;

std.println(bucket)