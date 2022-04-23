pub fn run() {
    use std::collections::HashMap;

    // init (array store on heap)
    let rust_array = [1,2,3];
    println!("array = {}", rust_array[0]);

    let tuple: (i32, f64, char) = (1, 3.14, 'K');
    println!("tuple = {}", tuple.1);

    // Vec = arraylist
    let mut v: Vec<i32> = Vec::new();
    let init_vector = vec![1,2,3];

    println!("vector = {}", init_vector[0]);
    println!("unwrap = {}", init_vector.get(2).unwrap());

    let mut articles = HashMap::new();
    articles.insert(String::from("test1"), "art1");
    articles.insert(String::from("test2"), "art2");

    // if have test3 return it, if not insert test3 as art3
    articles.entry(String::from("test3")).or_insert("art3");
    
    println!("hashmap = {}", articles.get(&String::from("test1")).unwrap())
}