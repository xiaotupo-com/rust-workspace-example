use rusqlite::{Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?; // 创建连接

    // 创建一个数据库 person
    conn.execute(
        "CREATE TABLE person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )", (),
    )?;

    // 创建一个结构体
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };

    // 把 me rust 结构体的信息插入 person 库
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    // 查询数据
    let mut stmt = conn.prepare(
        "SELECT id, name, data FROM person"
    )?;

    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}
