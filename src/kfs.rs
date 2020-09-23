use rusty_v8 as v8;
use std::fs;

pub fn read(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments,mut ret:  v8::ReturnValue,){
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);

    let file_name = args.get(0);
    assert!(file_name.is_string());

    let file_name_rstr = file_name.to_string(scope).unwrap().to_rust_string_lossy(scope);

    let file_handle = fs::read(file_name_rstr);
    match file_handle{
        Ok(f) => {
            let value = v8::String::new(scope, std::str::from_utf8(&f).unwrap()).unwrap();
            resolver.resolve(scope, value.into()).unwrap();
        }
        Err(e) => {
            let msg = v8::String::new(scope, &e.to_string()).unwrap();
            resolver.reject(scope, msg.into());
        }
    }
    ret.set(promise.into());
}