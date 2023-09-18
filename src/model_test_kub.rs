pub mod test_mod;


pub struct Login {
   username: String,
   password: String,
}

impl Login {
    pub fn new(username:String , password:String)->Self{
        Self{
            username,
            password
        }
    }

    pub fn get_username(&self) ->&String{
        return &self.username;
    }
    pub fn get_password(&self) ->&String{
        return &self.password;
    }
}


