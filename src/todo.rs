// Rustの標準ライブラリ（std）からioモジュールをインポートしている。
// ioモジュールは、標準入出力などのI/O操作を行うための機能を提供する。Writeトレイトも同時にインポートしており、書き込み操作に関する機能を利用するために使用される。
use std::io::{self, Write};

// pubがついてるので、他のモジュールからアクセス可能なパブリックな構造体として定義されている。
pub struct Todo {
    id: u32,
    title: String,
    description: String,
}

// pubがついてるので、他のモジュールからアクセス可能なパブリックな構造体として定義されている。
pub struct TodoList {
    todos: Vec<Todo>,
}

impl Todo {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Todo {
            id,
            title,
            description,
        }
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: vec![] }
    }

    // 全てのTodoを一覧表示する
    pub fn display_todos(&self) {
        for todo in &self.todos {
            println!("ID: {}", todo.id);
            println!("Title: {}", todo.title);
            println!("Description: {}", todo.description);
            println!("-----------------------");
        }
    }

    // ユーザーからの入力を受け取り、Todoを追加する
    pub fn add_todo_from_user_input(&mut self) {
        println!("新しいTodoを追加してください。");
        print!("タイトル: ");
        io::stdout().flush().unwrap();
        let mut title = String::new();
        io::stdin().read_line(&mut title).unwrap();

        print!("説明: ");
        io::stdout().flush().unwrap();
        let mut description = String::new();
        io::stdin().read_line(&mut description).unwrap();

        // 改行文字の除去
        title = title.trim().to_string();
        description = description.trim().to_string();

        let id = self.todos.len() as u32 + 1;
        let todo = Todo::new(id, title, description);
        self.todos.push(todo);
        println!("\n新しいTodoを追加しました。");
    }
}
