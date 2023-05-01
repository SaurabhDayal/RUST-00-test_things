use pwhash::bcrypt;

fn main(){

    let pwd =String::from("lucknow");
    let h = bcrypt::hash(&pwd).unwrap();

    println!("Password <{}> hashed value:<{}>", pwd, h);
    
    assert_eq!(bcrypt::verify("lucknow", &h), true);

}