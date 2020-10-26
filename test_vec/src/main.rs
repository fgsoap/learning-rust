fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Hello, world! {:?}", v);

    {
        let v = vec![1,2,3,4,5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        match v.get(2) {
            Some(third)=>println!("The third element is {}", third),
            None=>println!("There is no third element."),
        }
        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
        println!("{:?}", does_not_exist);
    }

    {
        let mut v = vec![1,2,3,4,5];
        v.push(6);
        let first = &v[5];
        // v.push(6);
        println!("This first element is: {}.", first);
    }
}
