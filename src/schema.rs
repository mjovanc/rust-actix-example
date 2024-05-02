#[derive(Table)]
#[table_name = "users"]
pub struct User {
    id: usize,
    username: String,
    email: String,
    address: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct Category {
    id: usize,
    name: String,
}
