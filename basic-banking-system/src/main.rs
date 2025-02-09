trait Account{
    fn deposit(&mut self,amount:u32);
    fn withdraw(&mut self,amount:u32);
    fn balance(&self)->u32;
}

struct BankAccount{
    account_number:String,
    holder_name:String,
    balance:u32
}

impl Account for BankAccount{
    fn deposit(&mut self,amount:u32){
        self.balance+=amount;
    }
    fn withdraw(&mut self,amount:u32){
        self.balance-=amount;
    }
    fn balance(&self)->u32{
        self.balance
    }
}

fn main() {
    let mut account1=BankAccount{
        account_number:String::from("12345"),
        holder_name:String::from("Mollik"),
        balance:3000
    };
    let mut account2=BankAccount{
        account_number:String::from("12346"),
        holder_name:String::from("Zahid"),
        balance:10000

    };
    account1.deposit(5000);
    account1.withdraw(2500);
    println!("{}'s Balance:{}",account1.holder_name,account1.balance());
    account2.deposit(10000);
    account2.withdraw(5000);
    println!("{}'s Balance:{}",account2.holder_name,account2.balance());
    
}
