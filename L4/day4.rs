// struct Book{
//     name:String,
//     author:String,
//     page_count:i32,
//     price:i32
//}

struct Rectangle{
    width:i32,
    height:i32
}

// struct Square{
//     length:i32
// }

// fn area(rec: &Rectangle)->i32{
//     rec.height*rec.width
// }

impl Rectangle{
    fn new(width:i32,height:i32)->Self{
        Self {width,height}
    }
    
    fn area(&self)->i32{
        self.height*self.width
    }
    fn doublt(&mut self){
        self.height*=2;
        self.width*=2;
    }
    fn compare(&self,other: &Rectangle)->bool{
        self.height*self.width>other.height*other.width
    }
}


fn main() {
    // println!("Try clicking the Run button.");

    // let book1=Book{
    //     name:String::from("How to win friends and influence people"),
    //     author:String::from("Dale Carneggie"),
    //     page_count:120,
    //     price:350
    // };
    // let book2=Book{
    //     name:String::from("Rich Dad and Poor Dad"),
    //     author:String::from("Robert"),
    //     page_count:250,
    //     price:550
    // };
    // let book3=Book{
    //     name:String::from("Demo book"),
    //     author:String::from("Demo Author"),
    //     page_count:130,
    //     price:85
    // };

    // let books=vec![book1,book2,book3];

    // for temp  in &books{
    //     println!("{}",temp.author);
    // }


    // let a=(32,98,"Priyam");
    // println!("{}",a.1);

    // println!("{}",book1.name);


    // let c=(String::from("Psychology of Money"),"author",300,430);
    // println!("{}",c.0);
    // let name=String::from("How t win friendds and influence people");
    // let author=String::from("Dale Carneggie");
    // let page_count=120;
    // let price=350;


    // let name=String::from("How t win friendds and influence people");
    // let author=String::from("Dale Carneggie");
    // let page_count=120;
    // let price=350;
    let mut r1=Rectangle::new(10,20);
    let mut r2=Rectangle::new(15,18);
    // let mut r1=Rectangle{
    //     width:100,
    //     height:20
    // };
    // r1.doublt();
    println!("{}",r1.area());
    println!("{}",r1.compare(&r2));
}