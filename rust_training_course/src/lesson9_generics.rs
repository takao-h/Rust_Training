// ここに `largest` 関数を実装してください。

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // ここを実装してください
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
