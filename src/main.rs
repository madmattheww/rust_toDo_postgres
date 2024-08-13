use std::io; // библиотека для ввода
use std::io::Write; // библиотека для работы с вводом и выводом данных
use tokio_postgres::{NoTls, Error, GenericClient}; // библиотека отсутсвия шифрования и возвращает ошибки

#[tokio::main]
async fn main() -> Result<(), Error> {
    struct Note { // структура, представляющая заметку
        name: String,
        number: i32,
        content: String,
    }

    // подключение к бд
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=YOUR_PASSWORD dbname=NAME_DB", NoTls).await?;
    // проверка на ошибки
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    // создание таблицы
    client.execute(
        "CREATE TABLE IF NOT EXISTS Rust_toDo (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        number INTEGER NOT NULL,
        content TEXT NOT NULL
        )",
        &[],
    ).await?;

    let mut input_name = String::new(); // ввод имени
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_name).unwrap();

    let mut input_number = String::new(); // ввод номера
    print!("Enter number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_number).unwrap();

    let mut input_note = String::new(); // ввод заметки
    print!("Enter note: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_note).unwrap();

    let notee = Note { // заполняем структуру
        name: input_name.trim().parse().unwrap(),
        number: input_number.trim().parse().unwrap(),
        content: input_note.trim().parse().unwrap()
    };

    client // вставка введенной информации в таблицу
        .execute("INSERT INTO Rust_toDo (name, number, content) VALUES ($1, $2, $3)", &[&notee.name, &notee.number, &notee.content],).await?;

    let query = client
        .query("SELECT * FROM Rust_toDo WHERE name = $1 AND number = $2 AND content = $3", &[&notee.name, &notee.number, &notee.content]).await?;

    for row in &query { // вывод
        //let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let number: i32 = row.get(2);
        let content: &str = row.get(3);
        println!("{}) {}: {}", number, name, content);
    }

    let mut name_all = String::new(); // создаем переменную для вывода всех заметок по имени
    println!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_all).unwrap();
    name_all = name_all.trim().to_string();


    let pink = client
        .query("SELECT * FROM Rust_toDo WHERE name = $1", &[&name_all]).await?;

    for row in &pink { // вывод всех заметок по имени
        //let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let number: i32 = row.get(2);
        let content: &str = row.get(3);
        println!("{}) {}: {}", number, name, content);
    }

    Ok(()) // Ok)))
}

/*let stmt = client.prepare("SELECT id, name, number, content FROM Rust_toDo").await?;
for row in client.query(&stmt, &[]).await? {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    let number: i32 = row.get(2);
    let content: &str = row.get(3);
    println!("{}) {}: {}", number, name, content); // выводит id, имя, номер, заметку в консоль
}*/