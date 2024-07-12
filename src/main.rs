use std::error::Error;
//use sqlx::Connection;
use sqlx::FromRow;

#[derive(Debug,FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title,author,isbn) VALUES ($1,$2,$3)";

    sqlx::query(query)
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .execute(pool).await?;

    Ok(())
}

async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .execute(pool)
    .await?;

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";

    let query = sqlx::query_as::<_,Book>(q);
    let books = query.fetch_all(conn).await?;

    Ok(books)
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let url = "postgres://postgres:mysecretpassword@localhost:5432/bookstore";
    //let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let book = Book{
        title: "aloga".to_string(),
        author: "Tulio".to_string(),
        isbn: "1233323".to_string()
    };

    create(&book, &pool).await?;


    Ok(())
}
