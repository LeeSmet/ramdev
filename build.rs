extern crate bindgen;
extern crate shlex;

use std::boxed::Box;
use std::env;
use std::path::PathBuf;

static CLANG_BLACKLIST_ARGS: [&str; 10] = [
    // Ignore args which cause clang errors.
    // If these are not ignored, clang will inform us
    // that it doesnt like them and they should be removed.
    "-mno-fp-ret-in-387",
    "-mpreferred-stack-boundary=3",
    "-mskip-rax-setup",
    "-mindirect-branch=thunk-extern",
    "-mindirect-branch-register",
    "-fno-var-tracking-assignments",
    "-fconserve-stack",
    "-fmacro-prefix-map=./=",
    "-mrecord-mcount",
    // This one is more difficult, not ignoring it will cause the kernel
    // to include `asm goto` statements, which clang does not yet understand
    // (clang 6). The error is however not indicating this to be the issue
    "-DCC_HAVE_ASM_GOTO",
];

fn main() {
    // tell cargo to only rerun the build script if linux_headers.h changes...
    println!("cargo:rerun-if-changed=linux_headers.h");

    println!("Generating kernel header bindings");

    // change to kernel working directory, save current wd first
    let curr_wd = env::current_dir().unwrap();
    let k_dir = PathBuf::from(env::var("STD_KERNEL_PATH").unwrap());
    assert!(env::set_current_dir(&k_dir).is_ok());
    let clang_args = match std::env::var("STD_CLANG_ARGS") {
        Ok(s) => s.into_boxed_str(),
        Err(err) => {
            println!("{}", err);
            Box::from("")
        }
    };

    // filter blacklisted args and empty strings, probably a better way to do this
    let blacklist = CLANG_BLACKLIST_ARGS.to_vec();
    let clang_args: Vec<&str> = clang_args
        .split(" ")
        .filter(|&s| s != "" && !blacklist.contains(&s))
        .collect();

    // There is currently a bug in the generation of bitfields, which causes errors if the bitfield
    // is too large (bigger than 32), because some traits are only implemented up untill this size.
    // The issue is tracked here: https://github.com/rust-lang-nursery/rust-bindgen/issues/1325
    // Untill it is finished, we DO derive copy for the types, even though this might not be safe -
    // we'll see if something explodes along the way. Note that the "timex" struct must not be generated
    // for it cannot implement copy, but bindgen will strill add a derive copy to it. This is arguably also
    // a bug. A safer approach would be to NOT derive copy, and then ignore all structs which have bitfields
    // that are largen than allowed. As said, lets see how this goes first
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Nightly) // compile for nightly
        .derive_copy(true) // could be unsafe, we'll see
        .derive_debug(false) // no need for this
        .use_core() // no std obviously
        .ctypes_prefix("::os::raw_c_types") // use instead of std... default prefix, manually provided
        // add some macros for clang
        .clang_arg("-Dfalse=__false")
        .clang_arg("-Dtrue=__true")
        .clang_arg("-Du64=__u64")
        // push the filtered compiler args to clang
        .clang_args(clang_args.iter())
        .header(curr_wd.join("linux_headers.h").to_string_lossy())
        // "timex" gives some issues when generated, in the embedded bitfield
        .opaque_type("timex")
        // uncommented the line below if you want to generate some fancy call/dependancy graph.
        // actually a dot file. If you want an image, you better have a lot of time to wait while it
        // generates ;S
        // .emit_ir_graphviz(curr_wd.join("graph.dot").to_string_lossy())
        .generate()
        .expect("Unable to generate bindings");

    // change back to old working directory (rust directory)
    assert!(env::set_current_dir(&curr_wd).is_ok());
    // Write the bindings to the $BINDING_DIR/kernel.rs file.
    let binding_path = PathBuf::from(env::var("BINDING_DIR").unwrap());
    bindings
        .write_to_file(binding_path.join("kernel.rs"))
        .expect("Couldn't write bindings!");
    println!("Wrote bindings to {:?}", binding_path.join("kernel.rs"));
}
