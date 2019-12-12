use tyozo::{tyozo, Memdb};

fn main() -> Result<(), String> {
    let mut db = Memdb::new();
    tyozo("hoge", &mut db)
}
