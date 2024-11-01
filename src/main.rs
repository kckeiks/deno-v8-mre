use crate::runtime::Runtime;
use deno_core::v8::{Global, Value};
use deno_core::{serde_v8, v8, ModuleSpecifier};

mod loader;
mod runtime;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let module_url = "blake3://foo".parse::<ModuleSpecifier>().unwrap();
    let mut runtime = Runtime::new(module_url.clone(), 1).unwrap();
    println!("Executing the script");
    let value = runtime.exec(&module_url, None).await.unwrap().unwrap();
    parse_and_respond(&mut runtime, value).await.unwrap();
    println!("Finished executing the script");
}

async fn parse_and_respond(runtime: &mut Runtime, res: Global<Value>) -> anyhow::Result<()> {
    // Handle the return data
    let scope = &mut runtime.deno.handle_scope();
    let local = v8::Local::new(scope, res);

    if local.is_uint8_array() || local.is_array_buffer() {
        // If the return type is a U8 array, send the raw data directly to the client
        let bytes = match deno_core::_ops::to_v8_slice_any(local) {
            Ok(slice) => slice.to_vec(),
            Err(e) => panic!("failed to parse bytes: {e}"),
        };
        println!("Response: {bytes:?}");
    } else if local.is_string() {
        // Likewise for string types
        let string = serde_v8::from_v8::<String>(scope, local).unwrap();
        println!("Response: {string}");
    } else {
        // Parse the response into a generic json value
        let value = serde_v8::from_v8::<serde_json::Value>(scope, local)
            .unwrap()
            .clone();

        // Otherwise, send the data as a json string
        let res = serde_json::to_string(&value).unwrap();
        println!("Response: {res:?}");
    }

    Ok(())
}
