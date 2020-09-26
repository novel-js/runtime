// import { print } from "../std/dist"

// import {println, print, file} from "http://pkgs.io:3030/pkg/std/std";

function read(filename) {
    // print('filename', filename);
    return new Promise((resolve, reject) => {
        try {
            resolve($.fs_read(filename))
        } catch (e) {
            reject(e);
            // $.print("error occured")
        }
    })
}
export { read }

