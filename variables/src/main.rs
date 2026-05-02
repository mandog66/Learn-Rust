fn main() {
    // 變數預設是不可變的
    let a = 10;
    // a = 20; Error

    // 可以透過 mut 來宣告可變變數
    let mut b = 10;
    b = 20; // OK

    // 當宣告變數為常數(constants)則無論如何都無法更改
    const C: u32 = 100;
    // C = 10;  Error
    // const mut C : u32= 100;  Error

    // 遮蔽(Shadowing)
    // 變數名稱可以在重複使用
    let d = 100;
    println!("Number is {d}."); // 100
    let d = 200;
    println!("Number is {d}."); // 200

    // 遮蔽(Shadowing)和 mut 差異
    let e = "    ";
    let e = e.len();
    println!("length is {}", e); // OK

    // 無法變成另一種型別
    let mut f = "    ";
    // f = f.len(); Error
}
