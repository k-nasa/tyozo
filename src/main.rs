use tyozo::Memdb;

fn main() -> Result<(), String> {
    let mut db = Memdb::new();
    db.exec("hoge")?;

    Ok(())
}
