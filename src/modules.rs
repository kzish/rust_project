use diesel;
use diesel::prelude::*;
use diesel::mysql::MySqlConnection;

use schema::tasks;
use schema::tasks::dsl::tasks as all_tasks;

#[derive(Queryable)]
pub struct task {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name("tasks")]
pub struct newTask {
    pub description: String,
    pub done: bool,
}


impl task {
    pub fn show(id: i32, conn: &MySqlConnection) -> Vec<Book> {
        all_tasks
            .find(id)
            .load::<task>(conn)
            .expect("Error loading task")
    }

    pub fn all(conn &MySqlConnection) -> Vec<task> {
        all_tasks
        .order(tasks::id.desc())
        .load::<task>(conn)
        .expect("Error loading the books")
    }

    pub fn updateById(id: i32, conn, $MySqlConnection, task: newTask) -> bool {
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


    pub fn insert(task: newTask, conn: &MySqlConnection) -> bool {
        diesel::insert_into(tasks::table)
        .values(&task)
        .execute(conn)
        .is_ok()
    }

    pub fn deleteById(id: i32, conn: &MySqlConnection) -> bool {
        if(task::show(id, conn).is_empty()) {
            return false;
        }
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }
}
