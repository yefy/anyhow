use anyhow::anyhow;
use anyhow::fline;
use anyhow::AnyhowError;

pub fn err1() -> anyhow::Result<()> {
    Err(anyhow!("err1"))
}

pub fn err2() -> anyhow::Result<()> {
    err1().error(fline!())?;
    Ok(())
}

pub fn err3() -> anyhow::Result<()> {
    err2().errors(fline!(), "err2")?;
    Ok(())
}

pub fn err4() -> anyhow::Result<()> {
    err3().errors(fline!(), "err3")?;
    Ok(())
}

fn main() {
    let ret = err4();
    if let Err(e) = ret {
        println!("{:?}", e);
    }
}
