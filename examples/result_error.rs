use anyhow::anyhow;
use anyhow::anyhow_error;

pub fn err1() -> anyhow::Result<()> {
    Err(anyhow!("1_1"))
}

pub fn err1_2() -> anyhow::Result<()> {
    Err(anyhow!())
}

pub fn err1_3() -> anyhow::Result<()> {
    let err = anyhow!("1_3");
    Err(anyhow!(err))
}

pub fn err1_4() -> anyhow::Result<()> {
    Err(anyhow!("1_4:{}", "1_4"))
}

pub fn err4() -> anyhow::Result<()> {
    err1_3().map_err(anyhow_error!("4_1"))?;
    Ok(())
}

pub fn err4_2() -> anyhow::Result<()> {
    err4().map_err(anyhow_error!("4_2:{}", "4_2"))?;
    Ok(())
}

pub fn err4_3() -> anyhow::Result<()> {
    let err = anyhow!("4_3");
    err4_2().map_err(anyhow_error!(err))?;
    Ok(())
}

pub fn err4_4() -> anyhow::Result<()> {
    err4_3().map_err(anyhow_error!())?;
    Ok(())
}

fn main() {
    let ret = err4_4();
    if let Err(e) = ret {
        println!("{:?}", e);
    }
}
