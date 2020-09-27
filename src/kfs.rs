use rusty_v8 as v8;
use std::fs;
use std::io::prelude::*;
pub fn read(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut ret: v8::ReturnValue,
) {
    let file_name = args.get(0);
    let file_handle = fs::read(
        file_name
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope),
    );
    match file_handle {
        Ok(f) => ret.set(
            v8::String::new(scope, &String::from_utf8(f).unwrap())
                .unwrap()
                .into(),
        ),
        Err(e) => {
            let msg = v8::String::new(scope, &e.to_string()).unwrap();
            scope.throw_exception(msg.into());
        }
    }
}

pub fn write(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut ret: v8::ReturnValue,
) {
    let file_name = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let content = args
        .get(1)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let file_handle = fs::write(file_name, content);
    if let Err(e) = file_handle {
        let msg = v8::String::new(scope, &e.to_string()).unwrap();
        scope.throw_exception(msg.into());
    }
    ret.set(v8::undefined(scope).into());
}
pub fn append(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut ret: v8::ReturnValue,
) {
    let file_name = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let content = args
        .get(1)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let file_handle = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name);

    // let file_handle = fs::app(file_name, content);
    match file_handle {
        Ok(mut f) => {
            match writeln!(f, "{}", content) {
                Ok(_) => {
                    ret.set(v8::undefined(scope).into());
                }
                Err(e) => {
                    let msg = v8::String::new(scope, &e.to_string()).unwrap();
                    scope.throw_exception(msg.into());
                }
            };
        }
        Err(e) => {
            let msg = v8::String::new(scope, &e.to_string()).unwrap();
            scope.throw_exception(msg.into());
        }
    }
}
