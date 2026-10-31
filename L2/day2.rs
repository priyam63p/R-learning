// fn main(){
//     println!("Hello World");

//     // add_number(3,4);
//     let str=String::from("Hello");
//     finder(str);
//     // println!("{}",str);
    
//     let a=10;
//     let b=20;
//     let ans=add_number(a,b);
//     println!("{}",ans);
// }

// fn add_number(x:i32,y:i32)->i32{
//     // println!("{}",x+y);
//     return x+y
// }


// fn finder(s:String){
//     println!("{}",s);
//     //drop
// }


fn main(){
    // let mut str=String::from("Hello, Good morning");

    // let r1=&str;
    // let r2=&str;
    // str.push_str( Hi);
    // println!("{} {}",r1,r2);
   // immutable_borrow(&str);//Only read the data

  //  mutable_borrow(&mut str);//Read and Write
    // println!("{}",str);

    // let mut arr=[10,20,30,40];
    // println!("{}",arr[0]);
    // arr[2]=90; 

    // for element in arr{
    //     println!("{}",element);
    // }

    // for num in 1..5{
    //     println!("{}",num);
    // }

    //  for num in 1..=5{
    //     println!("{}",num);
    // }



    // for num in (1..5).rev(){
    //     println!("{}",num);
    // }


    // let age=20;
    // if age>18{
    //     println!("You are eligible for vote");
    // }
    // else if age==18{
    //     println!("Get a royal pass");
    // }
    // else{
    //     println!("You are eligible");
    // }

    // let mut counter=0;
    // let result=loop{
    //     counter+=1;
    //     println!("Again");

    //     if counter ==10{
    //         break counter*2;
    //     }
    // };
    // println!("{}",counter);
    // println!("{}",result);

    let a=(10,20,true,"Priyam");
    println!("{:?}",a);
     println!("{:?}",a.2);
}

// fn immutable_borrow(s:&String){
//     println!("{}",s);
// }

// fn mutable_borrow(s:&mut String){
//     s.push_str(" Hi");
//     println!("{}",s);
// }