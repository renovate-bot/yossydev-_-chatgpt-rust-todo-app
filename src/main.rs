// main.rsが、src/todo.rsを使うことを宣言
mod todo;

/**
 * todoモジュールのTodoList構造体を現在のスコープに導入している。
 * useキーワードを使用することで、別のモジュール内で定義された要素を簡単に参照できる。
 */
use todo::TodoList;

// main関数は、プログラムのエントリーポイントとなっている。プログラムが実行されると、まずこの関数が呼び出される.
fn main() {
    // TodoList::new()は、TodoList構造体のnew関連関数を呼び出して、新しいTodoListオブジェクトを作成している。
    // TypeScriptで言うところの、`const todo = new TodoList()`みたいな感じ。
    let mut todo_list = TodoList::new();

    // ユーザーからの入力を受け取り、Todoを追加
    todo_list.add_todo_from_user_input();

    // 全てのTodoを一覧表示
    todo_list.display_todos();
}
