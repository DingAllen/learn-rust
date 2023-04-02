// 编写函数，展示访问vector中一个值的两种方式
pub fn read_vector_item_example() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

// 遍历vector中的值
pub fn iterate_vector_example() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}