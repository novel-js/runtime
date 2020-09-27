use rusty_v8 as v8;
#[macro_use]
extern crate lazy_static;
mod kfs;
mod kstd;
use colored::*;
use isahc::prelude::*;
lazy_static! {
    static ref MODULE_MAP: std::sync::Mutex<std::collections::HashMap<i32, String>> =
        std::sync::Mutex::new(std::collections::HashMap::new());
}
//stolen directly from the rusty_v8 tests.rs
lazy_static! {
    static ref INIT_LOCK: std::sync::Mutex<u32> = std::sync::Mutex::new(0);
  }

#[must_use]
struct SetupGuard {}

impl Drop for SetupGuard {
  fn drop(&mut self) {
    // TODO shutdown process cleanly.
  }
}
#[cfg(test)]
fn setup() -> SetupGuard {
  let mut g = INIT_LOCK.lock().unwrap();
  *g += 1;
  if *g == 1 {
    v8::V8::initialize_platform(v8::new_default_platform().unwrap());
    v8::V8::initialize();
  }
  SetupGuard {}
}

pub fn compile_module<'a>(
    scope: &mut v8::HandleScope<'a>,
    code: String,
    name: String,
) -> Option<v8::Local<'a, v8::Module>> {
    // Register functions into object
    // println!("Name is {}", name);
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
        v8::String::new(scope, "fs_read").unwrap(),
        v8::Function::new(scope, kfs::read).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "fs_write").unwrap(),
        v8::Function::new(scope, kfs::write).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "fs_append").unwrap(),
        v8::Function::new(scope, kfs::append).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "tassert").unwrap(),
        v8::Function::new(scope, kstd::assert_or_panic).unwrap()
    ));

    let global_std_obj = v8::Object::new(scope);
    for funcs in funcs {
        global_std_obj
            .set(scope, funcs.0.into(), funcs.1.into())
            .unwrap();
    }

    let k = v8::String::new(scope, "$").unwrap().into();
    // Set global `std` to refer to our object that has print objects
    let global = scope.get_current_context().global(scope);
    global.set(scope, k, global_std_obj.into());
    //TODO: Change in.js to detect what file is being run (future)
    let script_origin_resource_name = v8::String::new(scope, &name).unwrap().into();
    let script_origin_line_offset = v8::Integer::new(scope, 0);
    let script_origin_column_offset = v8::Integer::new(scope, 0);
    let script_origin_is_cross_origin = v8::Boolean::new(scope, false);
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
    let v8str_code: v8::Local<v8::String> = v8::String::new(scope, &code).unwrap();
    let script_source = v8::script_compiler::Source::new(v8str_code, &script_origin);
    let tc = &mut v8::TryCatch::new(scope);
    tc.set_verbose(true);
    let module = v8::script_compiler::compile_module(tc, script_source);
    match module {
        Some(m) => {
            MODULE_MAP
                .lock()
                .unwrap()
                .insert(m.get_identity_hash(), name.clone());

            let im = m.instantiate_module(tc, resolver);
            if im.is_none() {
                println!("[Warning] Module {} failed to be instantiated.", name);
                return None
            }

            // println!("compile_module: is_none: {} name: {} src {}", im.is_none(),name, code);
            //TODO: figure out if this should stay
            // let _result = module.evaluate(scope).unwrap();
            Some(m)
        }
        None => {
            if tc.has_caught() {
                let teu = tc.exception().unwrap();
                let msg = v8::Exception::create_message(tc, teu);
                let name = msg.get_script_resource_name(tc).unwrap();
                let line = msg.get_source_line(tc).unwrap();
                let line_indicator =
                    format!("Line {}", (msg.get_line_number(tc).unwrap() as i32)).green();
                let line_offset = vec![b' '; line_indicator.len()];
                print!(
                    "\n\nFile {}\n{}{}\n{}",
                    name.to_string(tc).unwrap().to_rust_string_lossy(tc),
                    String::from_utf8(line_offset).unwrap(),
                    line.to_rust_string_lossy(tc).bright_white().bold(),
                    line_indicator,
                );
                let mut cols: Vec<u8> = vec![];
                // cols.resize(line_indicator.len(), b'%');
                cols.resize(msg.get_start_column(), b' ');
                cols.resize(msg.get_end_column(), b'^');
                println!(
                    "{} {}: {}\n\n",
                    // String::from_utf8(line_offset).unwrap(),
                    String::from_utf8(cols).unwrap().bold().bright_yellow(),
                    "".bright_green(),
                    tc.message().unwrap().get(tc).to_rust_string_lossy(tc).red()
                );
            }
            tc.rethrow();
            tc.reset();
            None
        }
        
    }
}
pub fn resolver<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    unsafe {
        let scope = &mut v8::CallbackScope::new(context);

        // println!("ref = {}\nspec = {}", referrer.get_identity_hash(), specifier.to_rust_string_lossy(scope));
        let r = specifier.to_rust_string_lossy(scope);
        if std::fs::read(format!(".cache/novel/pkgs/{}", r,)).is_ok() {
            let n = specifier.to_rust_string_lossy(scope);
            let src = std::fs::read(format!(".cache/novel/pkgs/{}", r)).unwrap();
            compile_module(scope, String::from_utf8(src).unwrap(), n)
        } else {
            // println!("mod map: {:?}", MODULE_MAP.lock().unwrap());
            // let mn = ;
            match MODULE_MAP
                .lock()
                .unwrap()
                .get(&referrer.get_identity_hash())
            {
                Some(s) => {
                    println!("Pulling dependency... GET {} for {}", &r, s);
                }
                None => {
                    println!("Pulling dependency... GET {} for an unknown dependency", &r);
                }
            }

            let mut response = isahc::get(r).unwrap();
            let n = specifier.to_rust_string_lossy(scope);
            let src = response.text().unwrap();
            let r = specifier.to_rust_string_lossy(scope);
            // println!("{}", format!(".cache/novel/pkgs/{}", r));
            let last = r.split('/').last().unwrap();
            let r_without_last = r.replace(last, "");
            // println!("last =  {} r without lsat = {}", &last, &r_without_last);
            std::fs::create_dir_all(format!(".cache/novel/pkgs/{}", r_without_last)).unwrap();
            std::fs::write(format!(".cache/novel/pkgs/{}", r), &src).unwrap();
            let module = compile_module(scope, src, n).unwrap();
            MODULE_MAP
                .lock()
                .unwrap()
                .insert(module.get_identity_hash(), r);
            Some(module)
        }
    }
}


#[test]
fn math_test(){
    let _setup_guard = setup();
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    let code_input = std::fs::read("tests/math.js").unwrap();
    let module = compile_module(
        scope,
        String::from_utf8(code_input).unwrap(),
        "tests/math.js".into(),
    )
    .unwrap();
    let tc = &mut v8::TryCatch::new(scope);
    let _result = module.evaluate(tc);
    assert_eq!(tc.has_caught(), false);
}
#[test]
fn non_existent_function(){
    let _setup_guard = setup();

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    let code_input = std::fs::read("tests/non_existent_function.js").unwrap();
    let module = compile_module(
        scope,
        String::from_utf8(code_input).unwrap(),
        "tests/non_existent_function.js".into(),
    );
    assert!(module.is_none());
 }
 #[test]
fn fails_to_compile(){
    let _setup_guard = setup();

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    let code_input = std::fs::read("tests/fails_to_compile.js").unwrap();
    let module = compile_module(
        scope,
        String::from_utf8(code_input).unwrap(),
        "tests/fails_to_compile.js".into(),
    );
    assert!(module.is_some());
 }
fn main() {
    // let mut module_map: std::collections::hash_map::HashMap<v8::Module, String> = std::collections::hash_map::HashMap::new();
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    // TODO Implement dynamic module imports
    // isolate.set_host_import_module_dynamically_callback(resolver);
    let scope = &mut v8::HandleScope::new(isolate);

    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    //TODO: Support different file names
    let code_input = std::fs::read("example/in.js").unwrap();
    // println!("example/in.js = {}", std::str::from_utf8(&code_input).unwrap());
    let module = compile_module(
        scope,
        String::from_utf8(code_input).unwrap(),
        "example/in.js".into(),
    );
    module.unwrap().evaluate(scope).unwrap();
}
