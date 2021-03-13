#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    name: String,
    email: String,
    company: String,
}

impl Person {
    pub fn new(name: String,
               email: String,
               company: String) -> Person {
        Person { name, email, company }
    }
}