#[derive(Clone)]
pub enum Address {
    Address(Box<WordList>),
    Nil,
}

#[derive(Clone)]
pub struct WordList {
    pub value: String,
    pub count: u32,
    pub next: Address,
}

impl WordList {
    pub fn append(&mut self, elem: String){
        if self.value == elem {
            self.count = self.count +1;
        } else {
            match self.next {
                Address::Address(ref mut next_address) =>{
                    next_address.append(elem);
                }
                Address::Nil => {
                    let node = WordList {
                        value: elem,
                        count: 1,
                        next: Address::Nil
                    };
                    self.next = Address::Address(Box::new(node))
                }
            }
        }
    }

    pub fn print_list(&self){
        print!("{}{}",self.value,self.count);
        match self.next {
            Address::Address(ref next_address) =>{
                next_address.print_list()
            }
            Address::Nil => {}
        }
    }
}