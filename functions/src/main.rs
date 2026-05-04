fn main() {
    println!("Hello, world!");

    // 沒有參數的函式
    another_function();

    // 帶有參數的函式
    another_function_parameters(5);

    // 多個引數的函式，並帶入字元型別引數
    another_function_mutable_parameters(100, 'A');

    // 這裡的大括號是一個表達式，最終將 b 的值（也就是 1）賦值給 a
    let a = {
        let b = 1;
        b
    };
    println!("a is {a}");

    // 利用 if 表達式進行條件判斷並賦值
    let num = 100;
    let result = if num > 10 { "大於" } else { "小於" };
    println!("Base in expression. return {result}");

    // 呼叫函式 five 取得回傳值
    let num_five = five();
    println!("return {num_five} is expression");
}

// 一般函式
fn another_function() {
    println!("Another function!");
}

// 包含引數的函式
fn another_function_parameters(x: i32) {
    println!("Another function {x} is parameters!");
}

// 包含多個引數的函式
fn another_function_mutable_parameters(x: i32, name: char) {
    println!("another_function x is {x} and name is {name}");
}

// 用表達式的方式回傳（注意結尾沒有分號代表回傳）
fn five() -> i32 {
    5
}
