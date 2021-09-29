use std::thread;
use std::sync::mpsc;

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }
    
    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }
    
    fn balance(&self) -> i32 {
        self.0
    }
}

/*
    Basicamente uso channels para comunicar a todos los threads
    haciendo que funcionen en el orden que a mi me interesa
*/

fn main() {

    let account =  Account(0);

    let (sender1, receiver1): (mpsc::Sender<Account>, mpsc::Receiver<Account>)  = mpsc::channel();
    let (sender2, receiver2): (mpsc::Sender<Account>, mpsc::Receiver<Account>)  = mpsc::channel();
    let (sender3, receiver3): (mpsc::Sender<Account>, mpsc::Receiver<Account>)  = mpsc::channel();
    


    let customer1_handle = thread::spawn(move || -> Result<(), mpsc::RecvError> {
        let mut account = account;
        account.deposit(40);
        if let Err(_error) = sender1.send(account){
            return Err(mpsc::RecvError);
        }
        Ok(())
    });
    
    let customer2_handle = thread::spawn(move || -> Result<(), mpsc::RecvError>{
        let mut account = receiver1.recv()?;
        account.withdraw(30);
        if let Err(_error) = sender2.send(account){
            return Err(mpsc::RecvError);
        }
        Ok(())
    });
    
    let customer3_handle = thread::spawn(move || -> Result<(), mpsc::RecvError> {
        let mut account = receiver2.recv()?;
        account.deposit(60);
        if let Err(_error) = sender3.send(account){
            return Err(mpsc::RecvError);
        }
        Ok(())
    });
    
    let customer4_handle = thread::spawn(move ||  -> Result<Account, mpsc::RecvError>{
        let mut account = receiver3.recv()?;
        account.withdraw(70);
        
        Ok(account)
    });
    
    let handles = vec![
    customer1_handle,
    customer2_handle,
    customer3_handle,
    ];
    
    for handle in handles {
        if let Err(_error) = handle.join().unwrap(){
            panic!("Hubo algun error con los threads");
        }
    }
    

    if let Ok(final_thread) = customer4_handle.join(){
        if let Ok(cuenta) = final_thread{
            let savings = cuenta.balance();
            println!("Balance: {:?}", savings);
        }
        
    }
    
}
