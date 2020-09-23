use rusty_v8 as v8;
use v8::MapFnTo;
mod kfs;
mod kstd;
pub fn resolver<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    unsafe{
        let mut scope = v8::CallbackScope::new(context);
        println!("spec {}", specifier.to_rust_string_lossy(&mut scope));


        let script_origin_resource_name = v8::String::new(&mut scope, "in.js".into()).unwrap().into();
        let script_origin_line_offset = v8::Integer::new(&mut scope, 0).into();
        let script_origin_column_offset = v8::Integer::new(&mut scope, 0).into();
        let script_origin_is_cross_origin = v8::Boolean::new(&mut scope, true).into();
        let script_origin_script_id = v8::Integer::new(&mut scope, 123);
        let script_origin_sourcemap_url = v8::String::new(&mut scope, "").unwrap().into();
        let script_origin_opaque = v8::Boolean::new(&mut scope, true);
        let script_origin_is_wasm = v8::Boolean::new(&mut scope, false);
        let script_origin_is_es6_module = v8::Boolean::new(&mut scope, true);
        let script_origin = v8::ScriptOrigin::new(
            script_origin_resource_name,
            script_origin_line_offset,
            script_origin_column_offset,
            script_origin_is_cross_origin,
            script_origin_script_id,
            script_origin_sourcemap_url,
            script_origin_opaque,
            script_origin_is_wasm,
            script_origin_is_es6_module,
        );
        let code_input = std::fs::read(specifier.to_rust_string_lossy(&mut scope)).unwrap();

        let code = v8::String::new(&mut scope, &String::from_utf8(code_input).unwrap()).unwrap();

        let kcode = v8::String::new(&mut scope, "function a(){return 5+5}").unwrap();
        let script_source = v8::script_compiler::Source::new(code, &script_origin);



        let module = v8::script_compiler::compile_module(&mut scope, script_source).unwrap();
        let im: Option<bool> = module.instantiate_module(&mut scope, resolver);
        println!("Still running");
        Some(module)
    }
    
    // let scope = hs.enter();
    // None
    
    }
    

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    // isolate.set_host_import_module_dynamically_callback(test);
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
    funcs.push((
        v8::String::new(scope, "assert").unwrap(),
        v8::Function::new(scope, kstd::assert).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "read").unwrap(),
        v8::Function::new(scope, kfs::read).unwrap(),
    ));

    // funcs.push((
    //     v8::String::new(scope, "callback_test").unwrap(),
    //     v8::Function::new(scope, kstd::callback_test).unwrap(),
    // ));
    let global_std_obj = v8::Object::new(scope);
    for funcs in funcs {
        global_std_obj
            .set(scope, funcs.0.into(), funcs.1.into())
            .unwrap();
    }

    let k = v8::String::new(scope, "std").unwrap().into();
    // Set global `std` to refer to our object that has print objects
    global.set(scope, k, global_std_obj.into());

    let code_input = std::fs::read("example/in.js").unwrap();
    let code = v8::String::new(scope, &String::from_utf8(code_input).unwrap()).unwrap();
    // println!("javascript code: {}", code.to_rust_string_lossy(scope));
    //TODO: Change in.js to detect what file is being run (future)
    let script_origin_resource_name = v8::String::new(scope, "in.js".into()).unwrap().into();
    let script_origin_line_offset = v8::Integer::new(scope, 0).into();
    let script_origin_column_offset = v8::Integer::new(scope, 0).into();
    let script_origin_is_cross_origin = v8::Boolean::new(scope, true).into();
    let script_origin_script_id = v8::Integer::new(scope, 123);
    let script_origin_sourcemap_url = v8::String::new(scope, "").unwrap().into();
    let script_origin_opaque = v8::Boolean::new(scope, true);
    let script_origin_is_wasm = v8::Boolean::new(scope, false);
    let script_origin_is_es6_module = v8::Boolean::new(scope, true);
    let script_origin = v8::ScriptOrigin::new(
        script_origin_resource_name,
        script_origin_line_offset,
        script_origin_column_offset,
        script_origin_is_cross_origin,
        script_origin_script_id,
        script_origin_sourcemap_url,
        script_origin_opaque,
        script_origin_is_wasm,
        script_origin_is_es6_module,
    );
    let kcode = v8::String::new(scope, "function a(){return 5+5}").unwrap();
    let script_source = v8::script_compiler::Source::new(code, &script_origin);
    let /* mut*/  module = v8::script_compiler::compile_module(scope, script_source).unwrap();
    let im: Option<bool> = module.instantiate_module(scope, resolver);
    // println!("Module {}", module.);
    // for i in 0..module.get_module_requests_length(){
    //     let specifier = module.get_module_request(i);
    // }
    let result = module.evaluate(scope).unwrap();
    // println!("{}", result.to_string(scope).unwrap().to_rust_string_lossy(scope));
}
