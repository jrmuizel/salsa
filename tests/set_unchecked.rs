use salsa::Database;

salsa::query_group! {
    trait HelloWorldDatabase: salsa::Database {
        fn input() -> String {
            type Input;
            storage input;
        }

        fn length() -> usize {
            type Length;
        }

        fn double_length() -> usize {
            type DoubleLength;
        }
    }
}

fn length(db: &impl HelloWorldDatabase) -> usize {
    let l = db.input().len();
    assert!(l > 0); // not meant to be invoked with no input
    l
}

fn double_length(db: &impl HelloWorldDatabase) -> usize {
    db.length() * 2
}

#[derive(Default)]
struct DatabaseStruct {
    runtime: salsa::Runtime<DatabaseStruct>,
}

impl salsa::Database for DatabaseStruct {
    fn salsa_runtime(&self) -> &salsa::Runtime<DatabaseStruct> {
        &self.runtime
    }
}

salsa::database_storage! {
    struct DatabaseStorage for DatabaseStruct {
        impl HelloWorldDatabase {
            fn input() for Input;
            fn length() for Length;
            fn double_length() for DoubleLength;
        }
    }
}

#[test]
fn normal() {
    let mut db = DatabaseStruct::default();
    db.query_mut(Input).set((), format!("Hello, world"));
    assert_eq!(db.double_length(), 24);
    db.query_mut(Input).set((), format!("Hello, world!"));
    assert_eq!(db.double_length(), 26);
}

#[test]
#[should_panic]
fn use_without_set() {
    let db = DatabaseStruct::default();
    db.double_length();
}

#[test]
fn using_set_unchecked_on_input() {
    let mut db = DatabaseStruct::default();
    db.query_mut(Input)
        .set_unchecked((), format!("Hello, world"));
    assert_eq!(db.double_length(), 24);
}

#[test]
fn using_set_unchecked_on_input_after() {
    let mut db = DatabaseStruct::default();
    db.query_mut(Input).set((), format!("Hello, world"));
    assert_eq!(db.double_length(), 24);

    // If we use `set_unchecked`, we don't notice that `double_length`
    // is out of date. Oh well, don't do that.
    db.query_mut(Input)
        .set_unchecked((), format!("Hello, world!"));
    assert_eq!(db.double_length(), 24);
}

#[test]
fn using_set_unchecked() {
    let mut db = DatabaseStruct::default();

    // Use `set_unchecked` to intentionally set the wrong value,
    // demonstrating that the code never runs.
    db.query_mut(Length).set_unchecked((), 24);

    assert_eq!(db.double_length(), 48);
}
