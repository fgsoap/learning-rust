use std::fmt::Result;
use std::io::Result as IoResult;
#[allow(dead_code)]
fn function1() -> Result {
    println!("{}", "test1");
    Ok(())
}
#[allow(dead_code)]
fn function2() -> IoResult<()> {
    println!("{}", "test2");
    Ok(())
}
