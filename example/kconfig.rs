use rusty_v8 as v8;

pub fn set(scope: &mut v8::HandleScope,args: v8::FunctionCallbackArguments,_ret: v8::ReturnValue,) {
    // let context = scope.get_current_context();
    // let global = context.global(scope);
    // let dollar = v8::String::new(scope, "$".into()).unwrap();
    // let f = global.get(scope, dollar.into()).unwrap().to_object(scope).unwrap();

    // let test = v8::String::new(scope, "test").unwrap().into();
    // println!("{}", f.get(scope, test).unwrap().to_string(scope).unwrap().to_rust_string_lossy(scope));

    let context = scope.get_current_context();
    let global = context.global(scope);
    let dollar = v8::String::new(scope, "$".into()).unwrap();
    let global_config = global.get(scope,dollar.into()).unwrap().to_object(scope).unwrap();
    global_config.set(scope, args.get(0), args.get(1));
}
pub fn get(scope: &mut v8::HandleScope,args: v8::FunctionCallbackArguments,mut ret: v8::ReturnValue,) {
    // let context = scope.get_current_context();
    // let global = context.global(scope);
    // let dollar = v8::String::new(scope, "$".into()).unwrap();
    // let f = global.get(scope, dollar.into()).unwrap().to_object(scope).unwrap();

    // let test = v8::String::new(scope, "test").unwrap().into();
    // println!("{}", f.get(scope, test).unwrap().to_string(scope).unwrap().to_rust_string_lossy(scope));

    let context = scope.get_current_context();
    let global = context.global(scope);
    let dollar = v8::String::new(scope, "$".into()).unwrap();
    let global_config = global.get(scope,dollar.into()).unwrap().to_object(scope).unwrap();
    ret.set(global_config.get(scope, args.get(0)).unwrap());
}
pub fn get_functions(){
    return (get)
}