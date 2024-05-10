/*
fn main(){
    // let x=5;
    // let y=x;//copy
    let s1=String::from("hello");
    // let s2=s1;//Move
    let _s2=s1.clone();//Copy
    println!("{},world!",s1);
 }
 */

 fn main(){
    let s=String::from("hello");
    takes_ownership(s);
    // println!("{}",s); : causes error because string s moves
    
    let x=5;
    makes_copy(x);
    println!("Integer in main : {}",x);

    let s2=String::from("Priya");
    let s1=gives_ownership(s2);
    println!("After returning from function String : {}",s1); 

    let mut s3=String::from("Attitude");
    let change_s3=change(&mut s3);
    println!("After changing str : {}",change_s3);
/*
    let mut s4=String::from("4");
    let r1=&s4;
    let r2=&s4;
    let r3=&mut s4; 
    let r4=&mut s4; 

    println!("{}, {}, {}, {}", r1, r2,r3,r4);
    //cannot have mutable reference if already immutable reference is present
    //and cannot have more than one mutable refernce
    // cannot borrow `s4` as mutable because it is also borrowed as immutable
*/
    let mut s5 = String::from("5");

    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    println!("Immutable used : {} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s5; // no problem
    println!("Mutable is accessed even after immutable declared because of out of scope : {}", r3);


 }

 fn change(s:&mut String)->&mut String{
    s.push_str(" is ornament!!");
    s
 }
 fn takes_ownership(s:String){
    println!("String in function : {}",s);
    
 }

 fn gives_ownership(s:String)->String{
    s   
 }

 fn makes_copy(x:i32){
    println!("Integer in function : {}",x);
    
 }