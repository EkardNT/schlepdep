// DB operations needed:
// - Create batch idempotency record
//      - Report and do not edit if already exists by HK
// - Create command definition record
//      - Do nothing if already exists
// - Create command batch record
//      - Report and do not edit if already exists by HK
// - Read connection record by target name (HK) and account id

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