
///represents a person
#[derive(Debug)]
pub struct Person {
    pub first_name:String,
    pub last_name:String,
    pub address:Address,
    pub phone_number:String
}

///Address of a person
#[derive(Debug)]
pub struct Address {
    pub street:String,
    pub city:String,
    pub state:String,
    pub zip:String,
}