use rusty_v8 as v8;

mod kstd;

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);

    let context = v8::Context::new(scope);

    let global = &mut context.global(scope);

    let scope = &mut v8::ContextScope::new(scope, context);


    // Register functions into object
    let mut funcs: Vec<(v8::Local<v8::String>, v8::Local<v8::Function>)> = vec![];

    funcs.push((
        v8::String::new(scope, "print").unwrap(),
        v8::Function::new(scope, kstd::print).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "println").unwrap(),
        v8::Function::new(scope, kstd::println).unwrap(),
    ));

    let std_obj = v8::Object::new(scope);
    for funcs in funcs {
        std_obj.set(scope, funcs.0.into(), funcs.1.into()).unwrap();
    }


    let k = v8::String::new(scope, "std").unwrap().into();
    // Set global `std` to refer to our object that has print objects
    global.set(scope,k , std_obj.into());

    let code_input = std::fs::read("example/in.js").unwrap();
    let code = v8::String::new(scope, &String::from_utf8(code_input).unwrap()).unwrap();
    // println!("javascript code: {}", code.to_rust_string_lossy(scope));

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
