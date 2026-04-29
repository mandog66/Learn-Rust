# Note

## 什麼是**Prelude**?

這是一個可以讓專案把常使用的`項(Items)`預先打包，在編譯時自動引入的方法。

> [!NOTE]
> **項(Items)** : 是一個統稱。指的是在模組(Module)中包含的所有成員(Structs、Traits、Functions等)。

## 什麼是**關聯函式(Associated Functions)**?

這是指定義型別功能時不用接受self參數就能呼叫的函式。

> [!NOTE]
>
> * **型別(Type)** : 不只是數值型別(i32、d64、f64)，像是**struct**、**enum**、**trait**的自定義型別。
> * 關聯函式會用 **雙冒號加上括號**來呼叫。例如 :
>
> ```RUST
> String::new()
> ```
