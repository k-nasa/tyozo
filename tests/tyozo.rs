use tyozo::Memdb;

#[test]
fn test_tyozo() {
    let mut db = Memdb::new();

    let result = db.exec("set hoge value");
    assert_eq!(result, Ok(String::from("OK")));

    let result = db.exec("get hoge");
    assert_eq!(result, Ok(String::from("value")));

    db.exec("set fuga value").unwrap();
    let result = db.exec("del hoge fuga");
    assert_eq!(result, Ok(String::from("2")));

    let result = db.exec("setnx hoge value");
    assert!(result.is_ok());

    let result = db.exec("setnx hoge value");
    assert!(result.is_err());
}
