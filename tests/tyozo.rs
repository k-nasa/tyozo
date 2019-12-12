use tyozo::Memdb;

#[test]
fn test_tyozo() {
    let mut db = Memdb::new();

    let result = db.exec("set hoge value");

    assert!(result.is_ok());
    assert_eq!(result, Ok(String::from("OK")));
}
