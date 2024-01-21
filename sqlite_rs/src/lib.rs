use defs::Repository;
use rusqlite::Connection;

struct SqRepo {
    connection: Connection,
}

impl Repository for SqRepo {
    fn new() -> Self where Self: Sized {
        Self { connection: Connection::open("./test_db.db3").unwrap() }
    }
    fn add(&mut self, todo: defs::Todo) -> defs::Id {
        todo!()
    }
    fn check(&mut self, id: defs::Id) -> Option<()> {
        todo!()
    }
    fn list(&self) -> Vec<defs::Id> {
        todo!()
    }
}
