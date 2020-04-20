pub enum Database {
    Local(LocalDatabase)
}

impl Database {
    pub fn local() -> Self {
        Self::Local(LocalDatabase::new())
    }
}

struct LocalDatabase {

}

impl LocalDatabase {
    fn new() -> Self {
        LocalDatabase {

        }
    }
}