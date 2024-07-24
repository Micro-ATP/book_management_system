use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};

fn main() {
    let pool = get_db_connection().expect("无法连接到数据库");

    loop {
        println!("请选择操作：");
        println!("1: 注册");
        println!("2: 登录");
        println!("0: 退出");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");
        let choice = input.trim();

        match choice {
            "1" => register_user_prompt(&pool),
            "2" => {
                if let Some(user) = login_user_prompt(&pool) {
                    match user.user_type.as_str() {
                        "管理员" => admin_menu(&pool),
                        "普通用户" => user_menu(&pool, user.id),
                        _ => println!("未知用户类型"),
                    }
                }
            },
            "0" => break,
            _ => println!("无效的选项，请重试。"),
        }
    }
}

fn get_db_connection() -> Result<Pool> {
    let url = "mysql://ATP_Course:12345678@localhost/book_management";
    let pool = Pool::new(url)?;
    Ok(pool)
}

fn register_user_prompt(pool: &Pool) {
    let mut username = String::new();
    let mut password = String::new();
    let mut user_type = String::new();

    print!("请输入用户名：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("无法读取输入");

    print!("请输入密码：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("无法读取输入");

    print!("请输入用户类型（普通用户/管理员）：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_type).expect("无法读取输入");

    let username = username.trim();
    let password = password.trim();
    let user_type = user_type.trim();

    if user_type == "管理员" {
        print!("请输入邀请码：");
        io::stdout().flush().unwrap();
        let mut invite_code = String::new();
        io::stdin().read_line(&mut invite_code).expect("无法读取输入");
        let invite_code = invite_code.trim();

        if invite_code != "CUC_NB_666" {
            println!("邀请码无效。");
            return;
        }
    }

    if let Err(e) = register_user(pool, username, password, user_type) {
        println!("注册失败：{}", e);
    } else {
        println!("用户注册成功");
    }
}

fn login_user_prompt(pool: &Pool) -> Option<User> {
    let mut username = String::new();
    let mut password = String::new();

    print!("请输入用户名：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("无法读取输入");

    print!("请输入密码：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("无法读取输入");

    let username = username.trim();
    let password = password.trim();

    match login_user(pool, username, password) {
        Ok(Some(user)) => {
            println!("欢迎，{}！", user.username);
            Some(user)
        },
        Ok(None) => {
            println!("用户名或密码错误");
            None
        },
        Err(e) => {
            println!("登录失败：{}", e);
            None
        }
    }
}

fn admin_menu(pool: &Pool) {
    loop {
        println!("管理员菜单：");
        println!("1: 增加图书");
        println!("2: 删除图书");
        println!("3: 修改图书");
        println!("4: 查询图书");
        println!("0: 退出");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");
        let choice = input.trim();

        match choice {
            "1" => add_book_prompt(&pool),
            "2" => delete_book_prompt(&pool),
            "3" => update_book_prompt(&pool),
            "4" => query_books(&pool),
            "0" => break,
            _ => println!("无效的选项，请重试。"),
        }
    }
}

fn user_menu(pool: &Pool, user_id: u32) {
    loop {
        println!("用户菜单：");
        println!("1: 借书");
        println!("2: 还书");
        println!("3: 查询借书记录");
        println!("0: 退出");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");
        let choice = input.trim();

        match choice {
            "1" => borrow_book_prompt(&pool, user_id),
            "2" => return_book_prompt(&pool, user_id),
            "3" => query_borrowed_books(&pool, user_id),
            "0" => break,
            _ => println!("无效的选项，请重试。"),
        }
    }
}

fn add_book_prompt(pool: &Pool) {
    let mut title = String::new();
    let mut author = String::new();

    print!("请输入书名：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).expect("无法读取输入");

    print!("请输入作者：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut author).expect("无法读取输入");

    let title = title.trim();
    let author = author.trim();

    if let Err(e) = add_book(pool, title, author) {
        println!("添加图书失败：{}", e);
    } else {
        println!("图书添加成功");
    }
}

fn delete_book_prompt(pool: &Pool) {
    let mut id_str = String::new();

    print!("请输入要删除的图书ID：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_str).expect("无法读取输入");

    let id: u32 = id_str.trim().parse().expect("无效的图书ID");

    if let Err(e) = delete_book(pool, id) {
        println!("删除图书失败：{}", e);
    } else {
        println!("图书删除成功");
    }
}

fn update_book_prompt(pool: &Pool) {
    let mut id_str = String::new();
    let mut title = String::new();
    let mut author = String::new();

    print!("请输入要修改的图书ID：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_str).expect("无法读取输入");

    let id: u32 = id_str.trim().parse().expect("无效的图书ID");

    print!("请输入新的书名（留空则不修改）：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).expect("无法读取输入");

    print!("请输入新的作者（留空则不修改）：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut author).expect("无法读取输入");

    let title = if title.trim().is_empty() { None } else { Some(title.trim()) };
    let author = if author.trim().is_empty() { None } else { Some(author.trim()) };

    if let Err(e) = update_book(pool, id, title, author) {
        println!("修改图书失败：{}", e);
    } else {
        println!("图书修改成功");
    }
}

fn borrow_book_prompt(pool: &Pool, user_id: u32) {
    let mut book_id_str = String::new();

    print!("请输入要借阅的图书ID：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut book_id_str).expect("无法读取输入");

    let book_id: u32 = book_id_str.trim().parse().expect("无效的图书ID");

    if let Err(e) = borrow_book(pool, user_id, book_id) {
        println!("借书失败：{}", e);
    } else {
        println!("借书成功");
    }
}

fn return_book_prompt(pool: &Pool, user_id: u32) {
    let mut book_id_str = String::new();

    print!("请输入要归还的图书ID：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut book_id_str).expect("无法读取输入");

    let book_id: u32 = book_id_str.trim().parse().expect("无效的图书ID");

    if let Err(e) = return_book(pool, user_id, book_id) {
        println!("还书失败：{}", e);
    } else {
        println!("还书成功");
    }
}

fn query_borrowed_books(pool: &Pool, user_id: u32) {
    match get_borrowed_books(pool, user_id) {
        Ok(books) => {
            if books.is_empty() {
                println!("没有借阅记录");
            } else {
                println!("借阅记录：");
                for book in books {
                    println!("ID: {}, 书名: {}, 作者: {}", book.id, book.title, book.author);
                }
            }
        },
        Err(e) => println!("查询借阅记录失败：{}", e),
    }
}

fn query_books(pool: &Pool) {
    match get_books(pool) {
        Ok(books) => {
            if books.is_empty() {
                println!("没有图书");
            } else {
                println!("图书列表：");
                for book in books {
                    println!("ID: {}, 书名: {}, 作者: {}", book.id, book.title, book.author);
                }
            }
        },
        Err(e) => println!("查询图书失败：{}", e),
    }
}

fn register_user(pool: &Pool, username: &str, password: &str, user_type: &str) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO users (username, password, user_type) VALUES (:username, :password, :user_type)",
        params! {
            "username" => username,
            "password" => password,
            "user_type" => user_type,
        },
    )?;
    Ok(())
}

fn login_user(pool: &Pool, username: &str, password: &str) -> Result<Option<User>> {
    let mut conn = pool.get_conn()?;
    let user: Option<(u32, String, String, String)> = conn.exec_first(
        "SELECT id, username, password, user_type FROM users WHERE username = :username AND password = :password",
        params! {
            "username" => username,
            "password" => password,
        },
    )?;

    if let Some((id, username, password, user_type)) = user {
        Ok(Some(User { id, username, password, user_type }))
    } else {
        Ok(None)
    }
}

fn add_book(pool: &Pool, title: &str, author: &str) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO books (title, author) VALUES (:title, :author)",
        params! {
            "title" => title,
            "author" => author,
        },
    )?;
    Ok(())
}

fn delete_book(pool: &Pool, id: u32) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "DELETE FROM books WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}

fn update_book(pool: &Pool, id: u32, title: Option<&str>, author: Option<&str>) -> Result<()> {
    let mut conn = pool.get_conn()?;
    if let Some(title) = title {
        conn.exec_drop(
            "UPDATE books SET title = :title WHERE id = :id",
            params! {
                "title" => title,
                "id" => id,
            },
        )?;
    }
    if let Some(author) = author {
        conn.exec_drop(
            "UPDATE books SET author = :author WHERE id = :id",
            params! {
                "author" => author,
                "id" => id,
            },
        )?;
    }
    Ok(())
}

fn borrow_book(pool: &Pool, user_id: u32, book_id: u32) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO borrowed_books (user_id, book_id) VALUES (:user_id, :book_id)",
        params! {
            "user_id" => user_id,
            "book_id" => book_id,
        },
    )?;
    Ok(())
}

fn return_book(pool: &Pool, user_id: u32, book_id: u32) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "DELETE FROM borrowed_books WHERE user_id = :user_id AND book_id = :book_id",
        params! {
            "user_id" => user_id,
            "book_id" => book_id,
        },
    )?;
    Ok(())
}

fn get_books(pool: &Pool) -> Result<Vec<Book>> {
    let mut conn = pool.get_conn()?;
    let books: Vec<Book> = conn.query_map(
        "SELECT id, title, author FROM books",
        |(id, title, author)| {
            Book { id, title, author }
        },
    )?;
    Ok(books)
}

fn get_borrowed_books(pool: &Pool, user_id: u32) -> Result<Vec<BorrowedBook>> {
    let mut conn = pool.get_conn()?;
    let books: Vec<BorrowedBook> = conn.query_map(
        "SELECT b.id, b.title, b.author FROM borrowed_books bb JOIN books b ON bb.book_id = b.id WHERE bb.user_id = :user_id",
        |(id, title, author)| {
            BorrowedBook { id, title, author }
        },
    )?;
    Ok(books)
}

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u32,
    username: String,
    password: String,
    user_type: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

#[derive(Debug, PartialEq, Eq)]
struct BorrowedBook {
    id: u32,
    title: String,
    author: String,
}
