use miniconf_pest_parser::{Document, Value};

#[test]
fn parses_root_kv_and_section() {
    let src = r#"
    title = "Alpha"
    version = 2.5

    [db]
    host = "127.0.0.1"
    port = 5432
    "#;

    let doc = Document::parse(src).expect("parse ok");

    // root
    let root = doc.sections.get("root").expect("root section");
    assert!(matches!(root.get("title"), Some(Value::Str(s)) if s == "Alpha"));
    assert!(matches!(root.get("version"), Some(Value::Num(n)) if (*n - 2.5).abs() < 1e-9));

    // db
    let db = doc.sections.get("db").expect("db section");
    assert!(matches!(db.get("host"), Some(Value::Str(s)) if s == "127.0.0.1"));
    assert!(matches!(db.get("port"), Some(Value::Num(n)) if (*n - 5432.0).abs() < 1e-9));
}

#[test]
fn parses_arrays_objects_booleans_null() {
    let src = r#"
    arr = [1, 2, 3]
    flags = { debug: true, metrics: false }
    nothing = null
    "#;

    let doc = Document::parse(src).expect("parse ok");
    let root = doc.sections.get("root").unwrap();

    // array
    match root.get("arr").unwrap() {
        Value::Array(v) => {
            assert_eq!(v.len(), 3);
            assert!(matches!(v[0], Value::Num(n) if (n - 1.0).abs() < 1e-9));
        }
        _ => panic!("arr not array"),
    }

    // object
    match root.get("flags").unwrap() {
        Value::Object(m) => {
            assert!(matches!(m.get("debug"), Some(Value::Bool(true))));
            assert!(matches!(m.get("metrics"), Some(Value::Bool(false))));
        }
        _ => panic!("flags not object"),
    }

    // null
    assert!(matches!(root.get("nothing"), Some(Value::Null)));
}

#[test]
fn ignores_comments_and_whitespace() {
    let src = r#"
    # comment
    key = "value"   # trailing
    "#;

    let doc = Document::parse(src).expect("parse ok");
    let root = doc.sections.get("root").unwrap();
    assert!(matches!(root.get("key"), Some(Value::Str(s)) if s == "value"));
}
