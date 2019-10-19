extern crate wasmer_runtime;

use std::env;
use std::str;
use std::fs::File;
use std::io::prelude::*;

use wasmer_runtime::{
    imports,
    instantiate,
    error,
    func,
    Ctx,
};

fn main() -> error::Result<()> {
    let wasm_file_name = env::args().skip(1).next().unwrap();
    println!("wasm input module: {}", wasm_file_name);

    let mut file = File::open(wasm_file_name).expect("Failed to open file");
    let mut wasm_binary = Vec::new();
    file.read_to_end(&mut wasm_binary).expect("Failed to read WASM file");

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // We've defined a macro that makes it super easy.
    //
    // The signature tells the runtime what the signature (the parameter
    // and return types) of the function we're defining here is.
    // The allowed types are `i32`, `u32`, `i64`, `u64`,
    // `f32`, and `f64`.
    //
    // Make sure to check this carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our sample application.
        "env" => {
            // name        // the func! macro autodetects the signature
            "print_str" => func!(print_str),
        },
    };

    // Compile our webassembly into an `Instance`.
    let instance = instantiate(&wasm_binary, &import_object)?;

    // Call our exported function!
    instance.call("hello_wasm", &[])?;

    Ok(())
}

// Let's define our "print_str" function.
//
// The declaration must start with "extern" or "extern "C"".
// TODO: Should these be u64? since they're pointers and lengths of bytes?
// Their example seems to work ok with u32 though
// It also seems like the extern "C" or extern qualifier should not be used here
fn print_str(ctx: &mut Ctx, ptr: u32, len: u32) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = ctx.memory(0);

    // Get a subslice that corresponds to the memory used by the string.
    let str_vec: Vec<_> = memory.view()[ptr as usize..(ptr + len) as usize]
        .iter().map(|cell| cell.get()).collect();

    // Convert the subslice to a `&str`.
    let string = str::from_utf8(&str_vec).unwrap();

    // Print it!
    println!("{}", string);
}

