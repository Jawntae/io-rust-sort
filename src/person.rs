
///represents a person
struct Person {
    first_name:String,
    last_name:String,
    address:Address,
    phone_number:u32
}

///Address of a person
struct Address {
    street:String,
    city:String,
    state:String,
    zip:String,
}