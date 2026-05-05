fn main() {
    one_more_time();

    loop_check();

    loop_label();

    while_loop();

    read_elements_with_while();

    read_elements_with_for();

    for_loop();
}

// loop 迴圈會一直執行直到我們手動結束
fn one_more_time() {
    loop {
        println!("再一次!");
        // break;
    }
}

// 使用 loop 來檢查程式是否順利執行
// 使用 break 來跳出 loop 迴圈
fn loop_check() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is {result}!");
}

// 使用 loop 標籤(loop label)的方法來控制 break 特定的迴圈層級
fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count is {count}");

        let mut remaining = 10;

        loop {
            println!("remaining is {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End is {count}");
}

// 使用 while 倒數
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("number is {number}");
        number -= 1;
    }

    println!("number is {number}");
}

// 使用 while 遍歷集合元素
// 可能會因為某些問題出現錯誤，像是 index 超出陣列大小，效能也差
fn read_elements_with_while() {
    let mut index = 0;
    let arr = [10, 20, 30, 40, 50];

    while index < 5 {
        println!("number is {}", arr[index]);
        index += 1;
    }
}

// 使用 for 遍歷集合元素
// 避免錯誤讀取超出陣列大小的數值
fn read_elements_with_for() {
    let arr = [10, 20, 30, 40, 50];

    for nums in arr {
        println!("number is {nums}");
    }
}

// 使用 for 搭配 Range 與 rev() 反轉範圍來倒數
fn for_loop() {
    for nums in (0..4).rev() {
        println!("number is {nums}");
    }
}
