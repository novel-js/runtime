use rusty_v8 as v8;

fn core_print(
    scope: &mut v8::HandleScope,
    func_callback_args: &v8::FunctionCallbackArguments,
) -> String {
    let mut acc: Vec<String> = vec![];
    
    for i in 0..func_callback_args.length() {
        let arg = func_callback_args.get(i);
        acc.push(arg.to_string(scope).unwrap().to_rust_string_lossy(scope))
    }
    return acc.join(" ");
}

pub fn print(scope: &mut v8::HandleScope,args: v8::FunctionCallbackArguments,mut ret: v8::ReturnValue,) {

    let formatted = core_print(scope, &args);
    print!("{}", formatted);

    let ret_val = v8::String::new(scope, &format!("{}", formatted)).unwrap();
    ret.set(ret_val.into())
}
pub fn println(scope: &mut v8::HandleScope,args: v8::FunctionCallbackArguments,mut ret: v8::ReturnValue,) {
    let formatted = core_print(scope, &args);
    print!("{}\n", formatted);

    let ret_val = v8::String::new(scope, &format!("{}\n", formatted)).unwrap();
    ret.set(ret_val.into())
}
