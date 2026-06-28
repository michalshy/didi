pub fn new_database() -> String {
    "CREATE TABLE IF NOT EXISTS entries(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    command TEXT NOT NULL,
    path TEXT NOT NULL,
    exit_code INTEGER NOT NULL,
    timestamp TEXT NOT NULL)".to_string()
}

pub fn new_insert() -> String {
    "INSERT INTO entries(session_id, command, path, exit_code, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)".to_string() 
}

pub struct QueryBuilder {
    prefix: String,
    conditions: Vec<String>,
    postfix: String,
    params: Vec<Box<dyn rusqlite::ToSql>>,
    current_param: u32
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder { 
            prefix: "SELECT command, path, MAX(timestamp) as latest FROM entries ".to_string(), 
            conditions: Vec::new(), 
            postfix: "GROUP BY command, path ORDER BY latest ASC".to_string(),
            params: Vec::new(),
            current_param: 1
        }
    }

    pub fn add_like(&mut self, term: &str) {
        self.conditions.push(format!("WHERE command LIKE ?{} ", self.current_param));
        self.params.push(Box::new(term.to_string()));
    }

    pub fn add_time_check(&mut self) {

    }

    pub fn add_state_check(&mut self) {

    }

    pub fn finalize(self) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
        let mut final_string = self.prefix;
        for condition in &self.conditions {
            final_string += condition;
        }
        final_string += &self.postfix;
        (final_string, self.params)
    }
}