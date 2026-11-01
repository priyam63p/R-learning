// enum state{
//     connecting,
//     disconnected,
//     connected
// }

// fn main(){
//     let systemConnection=state::disconnected;

//     match systemConnection{
//         state::connected=>println!("Your system is connected"),
//         state::disconnected=>println!("Your system is disconnected"),
//         state::connecting=>println!("I am connecting")
//     };
// }



// enum Traffic{
//     Red,
//     Green,
//     Yellow
// }

// fn main(){
//     let light=Traffic::Red;
//     let update=match light{
//         Traffic::Red=>String::from("You have to wait"),
//         Traffic::Green=>String::from("You have to go"),
//         Traffic::Yellow=>String::from("Please Stop"),
        
//     };
//     println!("{}",update);
// }



// enum Message{
//     Quit,//no data
//     Text(String,i32,i32),//tuple
//     Move{x:i32,y:i32}//structure
// }
// fn matching(msg:&Message){
//     match msg{
//         Message::Quit=>println!("There is no message"),
//         _=>{},
//         // Message::Text(content,x,y)=>println!("Our {} {} {}",content,x,y),
//         // Message::Move{x,y}=>println!("Value is {} {}",x,y),
//     }
    
// }
// fn main(){
//     let msg1=Message::Text(String::from("Priyam Patra",),32,56);
//     let msg2=Message::Quit;
//     let msg3=Message::Move{x:20,y:40};
//     matching(&msg1);
//     matching(&msg2,);
//     matching(&msg3);
    
// }


// enum Option<T>{
//     Some(T),
//     None,
//     Hell{x:i32,y:i32},
// }

// fn main(){
//     // let msg1=Option::Some(String::from("Hello"));
//     let msg1:Option<i32>=Option::Hell{x:40,y:90};
//     let msg2=Option::Some(String::from("Hello"));
//     let msg3:Option<i32>=Option::None;
    
//     match msg2{
//         Option::Some(value)=>println!("{}",value),
//         Option::None=>println!("Nothing to show"),
//         Option::Hell{x,y}=>println!("{} {}",x,y),
//     }
// }


// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }

// fn divide(x:i32,y:i32)->Result<i32,String>{
//     if y==0{
//         return Result::Err(String::from("Division is not possible"));
//     }else{
//         return Result::Ok(x/y);
//     }
// }

// fn main(){
//     let result=divide(10,11); 
//     match result{
//         Result::Ok(value)=>println!("{}",value),
//         Result::Err(error)=>println!("{}",error),
//     }
// }

// fn main(){
//     let m1=Option::Some(10);

    
//     match m1{
//         Option::Some(value)=>println!("{}",value),
//         Option::None=>println!("Nothing to display"),
//         Option::Hell{x,y}=>println!("{} {}",x,y),
//     }
    
// }
    // let m2=Option::Some(String::from("Hello"));
    // let m3=Option::None;
    // let m4=Option::Hell{x:20,y:40};

    // matching(m1);
    // matching(m2);
    // matching(m3);
    // matching(m4);


enum Traffic{
    Red,
    Yellow,
    Green
}
  
impl Traffic{
     // constructor:value initilise
    fn new()->Self{
        Traffic::Red
    }
    fn next(&self)->Traffic{
        match self{
            Traffic::Red=>Traffic::Green,
            Traffic::Green=>Traffic::Yellow,
            Traffic::Yellow=>Traffic::Red
        }
    }
    fn duration(&self)->u32{
        match self{
        Traffic::Red=>60,
        Traffic::Green=>45,
        Traffic::Yellow=>60
        }
    }

    fn can_cross(&self)->bool{
        match self{
            Traffic::Green=>true,
            _=>false
        }
    }
}

fn main(){
    let mut light=Traffic::new();
    // light=light.next();
    light=light.next();
    let dur=light.duration();

    println!("{}",dur);
    println!("{}",light.can_cross());
}


    



