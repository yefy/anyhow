use anyhow::anyhow;
use anyhow::anyhow_error;
use anyhow::anyhow_line;
use anyhow::AnyhowError;

pub fn err1() -> anyhow::Result<()> {
    Err(anyhow!("111"))
}

pub fn err2() -> anyhow::Result<()> {
    err1().error(anyhow_line!())?;
    Ok(())
}

pub fn err3() -> anyhow::Result<()> {
    err2().errors(anyhow_line!(), "222")?;
    Ok(())
}

pub fn err3_2() -> anyhow::Result<()> {
    err3().errors(anyhow_line!(), &format!("333:{}", "33333"))?;
    Ok(())
}

pub fn err4() -> anyhow::Result<()> {
    err3_2().map_err(anyhow_error!("4444"))?;
    Ok(())
}

pub fn err4_2() -> anyhow::Result<()> {
    err4().map_err(anyhow_error!("5555:{}", "55555"))?;
    Ok(())
}

pub fn err4_3() -> anyhow::Result<()> {
    let err = anyhow::anyhow!("66666");
    err4_2().map_err(anyhow_error!(err))?;
    Ok(())
}

fn main() {
    let ret = err4_3();
    if let Err(e) = ret {
        println!("{:?}", e);
    }
}
