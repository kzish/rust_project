use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
#[macro_use]
extern crate schema;

use schema::tasks;//brings the tasks table into scope
use schema::tasks::dsl::tasks as all_tasks;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "tasks"]
pub struct task {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
// #[table_name("tasks")]
pub struct newTask {
    pub description: String,
    pub done: bool,
}


impl task {
    pub fn show(id: i32, conn: &MysqlConnection) -> Vec<Book> {
        all_tasks
            .find(id)
            .load::<task>(conn)
            .expect("Error loading task")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<task> {
        all_tasks
        .order(tasks::id.desc())
        .load::<task>(conn)
        .expect("Error loading the books")
    }

    pub fn updateById(id: i32, conn: &MysqlConnection, task: newTask) -> bool {
        use schema::books::dsl::{description as a, done as b};
        let newTask {
            description,
            done
        } = task;

        diesel::update(all_tasks.find(id))
        .set((a.eq(description), b.eq(done)))
        .get_result::<task>(conn)
        .is_ok()
    }


    pub fn insert(task: newTask, conn: &MysqlConnection) -> bool {
        diesel::insert_into(tasks::table)
        .values(&task)
        .execute(conn)
        .is_ok()
    }

    pub fn deleteById(id: i32, conn: &MysqlConnection) -> bool {
        if(task::show(id, conn).is_empty()) {
            return false;
        }
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }
}
