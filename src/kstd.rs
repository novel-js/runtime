use rusty_v8 as v8;
// use std::convert::TryFrom;
//TODO: Make this support configs.
fn core_print(
    scope: &mut v8::HandleScope,
    func_callback_args: &v8::FunctionCallbackArguments,
) -> String {
    let mut acc: Vec<String> = vec![];

    for i in 0..func_callback_args.length() {
        let arg = func_callback_args.get(i);
        if arg.is_object() && !arg.is_function(){
            let a_obj = arg.to_object(scope).unwrap().into();
            if let Some(stringified) = v8::json::stringify(scope, a_obj) {
                acc.push(stringified.to_rust_string_lossy(scope))
            }
        } else {
            acc.push(arg.to_string(scope).unwrap().to_rust_string_lossy(scope))
        }
    }
    acc.join(" ")
}

pub fn print(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    let formatted = core_print(scope, &args);
    print!("{}", formatted);
}
pub fn println(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    let formatted = core_print(scope, &args);
    println!("{}", formatted);
}

pub fn assert(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    if args.get(0) == v8::Boolean::new(scope, false) {
        if args.length() >= 2 {
            //Todo, find out why this needs to be 2.
            println!(
                "Assertion failed: {}",
                args.get(1)
                    .to_string(scope)
                    .unwrap()
                    .to_rust_string_lossy(scope)
            )
        } else {
            println!("Assertion failed");
        }
    }
}
pub fn is_nix(scope: &mut v8::HandleScope, _args: v8::FunctionCallbackArguments,mut ret: v8::ReturnValue,){
    if cfg!(windows){
        ret.set(v8::Boolean::new(scope, false).into());
    }else{
        ret.set(v8::Boolean::new(scope, true).into());
    }
}
    pub fn assert_or_panic(scope: &mut v8::HandleScope,args: v8::FunctionCallbackArguments,_ret: v8::ReturnValue,){
// if args.get(0) == v8::Boolean::new(scope, false){
// if args.length() >= 2{ //Todo, find out why this needs to be 2.
//     // assert()
//     // panic!("Assertion failed: {}", args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope))
// }else{
//     // panic!("Assertion failed");
// }
// }
    assert_eq!(args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope), "false")
}
