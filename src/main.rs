mod db;

use db::Database;

struct App{
    db: Database
}

impl App {

    pub fn init() -> App {
        return App { db: Database::init() };
    }

}

fn main() {
    
    let app = App::init();

}
