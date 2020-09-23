use rusty_v8 as v8;


fn get_from_object<'a>(
    scope: &mut v8::HandleScope<'a>,
    obj: &v8::Local<v8::Object>,
    key: String,
) -> Option<v8::Local<'a, v8::String>> {
    let lkey = v8::String::new(scope, &key).unwrap();
    let val = obj.get(scope, lkey.into()).unwrap().to_string(scope);

    if obj.is_object(){
        println!("obj get {:?}", obj.get(scope, lkey.into()).unwrap().to_detail_string(scope).unwrap().to_rust_string_lossy(scope));
    }else{
        return None
    }
    return match val {
        Some(v) => Some(v),
        None => None,
    };
    // match val {
    //     Some(v) => {return Some(v.to_rust_string_lossy(scope))}
    //     None => {return None}
    // }
    // return val
}
fn core_print(
    scope: &mut v8::HandleScope,
    func_callback_args: &v8::FunctionCallbackArguments,
) -> String {
    let mut acc: Vec<String> = vec![];
    let potential_config = func_callback_args.get(func_callback_args.length() - 1);
    let sep: get_from_object(scope, potential_config.to_object(), "sep");
    
    // if potential_config.is_object() {
    //     let config = potential_config.to_object(scope).unwrap();
    //     let sep_str = v8::String::new(scope, "sep").unwrap();


    //     for i in 0..func_callback_args.length() {
    //         let arg = func_callback_args.get(i);
    //         acc.push(arg.to_string(scope).unwrap().to_rust_string_lossy(scope))
    //     }
    //     return acc.join(&sep);



    // }
    for i in 0..func_callback_args.length() {
        let arg = func_callback_args.get(i);
        acc.push(arg.to_string(scope).unwrap().to_rust_string_lossy(scope))
    }
    return acc.join(&sep);
}

fn print(
    scope: &mut v8::HandleScope,
    func_callback_args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    let formatted = core_print(scope, &func_callback_args);
    print!("{}", formatted);

    let ret_val = v8::String::new(scope, &format!("{}", formatted)).unwrap();
    return_value.set(ret_val.into())
}
fn println(
    scope: &mut v8::HandleScope,
    func_callback_args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    let formatted = core_print(scope, &func_callback_args);
    print!("{}\n", formatted);

    let ret_val = v8::String::new(scope, &format!("{}\n", formatted)).unwrap();
    return_value.set(ret_val.into())
}

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);

    let context = v8::Context::new(scope);

    let global = &mut context.global(scope);

    let scope = &mut v8::ContextScope::new(scope, context);
    let mut funcs: Vec<(v8::Local<v8::String>, v8::Local<v8::Function>)> = vec![];
    funcs.push((
        v8::String::new(scope, "print").unwrap(),
        v8::Function::new(scope, print).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "println").unwrap(),
        v8::Function::new(scope, println).unwrap(),
    ));
    for funcs in funcs {
        global.set(scope, funcs.0.into(), funcs.1.into());
    }
    // let key = v8::String::new(scope, "print").unwrap();
    // let value = v8::Function::new(scope, print).unwrap();

    // global.set(scope, key.into(), value.into());

    let code_input = std::fs::read("in.js").unwrap();
    let code = v8::String::new(scope, &String::from_utf8(code_input).unwrap()).unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let /* mut*/  script_o = v8::Script::compile(scope, code, None);
    match script_o {
        Some(script) => {
            let result = script.run(scope).unwrap();
            let result = result.to_string(scope).unwrap();
            println!("result: {}", result.to_rust_string_lossy(scope));
        }
        None => panic!("Compilation failed"),
    }
}
