use neon::prelude::*;
use neon::register_module;

fn hello_world(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from Neon!"))
}

register_module!(mut m, { m.export_function("helloWorld", hello_world) });
