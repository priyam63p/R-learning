fn main(){
//     let a="Priyam";
//     println!("{}",a);

    // let mut arr=vec![10,20,30];

    // arr.push(50);
    // arr.push(80);
    // arr.push(60);
    // println!("{:p}",&arr);
    // println!("{:p}",&arr1); 

    // let mut v:Vec<String>=Vec::new();

    // v.push(String::from("Priyam"));
    // v.push(String::from("Patra"));
    // v.push(String::from("Hello"));

    // println!("{:?}",v);

    let sum=add_number(&v[0..4]);
    println!(sum);


     let add_number(arr:&i32)->{
        let mut sum=0;
        for num in arr{
            sum+=num;
        } 
     sum;
    }
   
}

