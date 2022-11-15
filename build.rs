fn main() -> Result<(), Box<dyn std::error::Error>> {
  let ignore_case = std::env::var("OUT_DIR").is_ok();
  println!("env out {}",ignore_case);
  tonic_build::compile_protos("proto/helloworld.proto")?;
  Ok(())
}