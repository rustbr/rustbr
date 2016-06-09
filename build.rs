extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    let mut cg = includedir_codegen::start("FILES");

    cg.dir("assets", Compression::Gzip)
      .build("assets.rs")
      .unwrap();
}
