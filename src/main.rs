use std::io; // библиотека для ввода
use std::io::Write; // библиотека для работы с вводом и выводом данных
use tokio_postgres::{NoTls, Error}; // библиотека отсутсвия шифрования и возвращает ошибки

#[tokio::main]
async fn main() -> Result<(), Error> {
    // подключение к бд
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=YOUR_PASSWORD dbname=NAME_DATABASE", NoTls).await?;
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
    let input_name = input_name.trim();

    let mut input_number = String::new(); // ввод номера
    print!("Enter number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_number).unwrap();
    let input_number: i32 = input_number.trim().parse().unwrap();

    let mut input_note = String::new(); // ввод заметки
    print!("Enter note: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_note).unwrap();
    let input_note = input_note.trim();


    client // вставка введенной информации в таблицу
        .execute("INSERT INTO Rust_toDo (name, number, content) VALUES ($1, $2, $3)", &[&input_name, &input_number, &input_note],).await?;

    let stmt = client.prepare("SELECT id, name, number, content FROM Rust_toDo").await?;
    for row in client.query(&stmt, &[]).await? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let number: i32 = row.get(2);
        let content: &str = row.get(3);
        println!("{}, {}, {}, {}", id, name, number, content); // выводит id, имя, номер, заметку в консоль
    }

    Ok(())
}