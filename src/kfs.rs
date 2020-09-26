use rusty_v8 as v8;
use std::fs;

pub fn read(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments,mut ret:  v8::ReturnValue,){
    let file_name = args.get(0);
    let file_handle = fs::read(file_name.to_string(scope).unwrap().to_rust_string_lossy(scope));
    match file_handle{
        Ok(f) => {
            ret.set(v8::String::new(scope, &String::from_utf8(f).unwrap()).unwrap().into())
   
        }
        Err(e) => {
            let msg = v8::String::new(scope, &e.to_string()).unwrap();
            // let exc = v8::Exception::error(scope, msg.into());
            scope.throw_exception(msg.into());
            // ret.set(exc);
            // ret.set(v8::String::new(scope, &String::from_utf8(f).unwrap()).unwrap().into())
        }
    }    
}