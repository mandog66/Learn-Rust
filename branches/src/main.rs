fn main() {
    let nums = 5;

    // 使用 if 表達式執行數字比大小
    println!("{}", number_compare(nums));

    // 使用 if else 表達式取得判斷結果
    println!("{}", get_modulo(nums));

    // 使用 if 表達是直接賦值給變數
    // if 和 else 區塊的回傳型別必須相同
    let condition = true;
    let condition_nums = if condition { 5 } else { 6 };
    println!("condition_nums is {condition_nums}");
}

// 使用 String 型別回傳，避免字串參考（reference）的生命週期問題
fn number_compare(nums: i32) -> String {
    if nums >= 10 {
        String::from("true")
    } else {
        String::from("false")
    }
}

fn get_modulo(nums: i32) -> String {
    if nums % 2 == 0 {
        String::from("2 is OK")
    } else if nums % 3 == 0 {
        String::from("3 is OK")
    } else {
        String::from("2 and 3 not OK")
    }
}
