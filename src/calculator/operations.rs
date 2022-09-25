use std::collections::HashMap;


pub type Operation = fn(&mut Vec<f64>) -> Result<(), &'static str>;

// fn addition(&mut Vec<f64>)

pub fn generate_operations() -> HashMap<&'static str, Operation>{
    let mut operations:HashMap<&'static str, Operation> = HashMap::new();

    // Addition
    operations.insert("+", |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Addition requires two arguements!")
        }

        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b+a);

        Ok(())
    });

    // Subtraction
    operations.insert("-", |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Subtraction requires two arguements!")
        }

        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b-a);

        Ok(())
    });

    // Multiplication
    operations.insert("*", |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Multiplication requires two arguements!")
        }

        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b*a);

        Ok(())
    });

    // Division
    operations.insert("/", |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Division requires two arguements!")
        }

        let a = stack.pop().unwrap();
        if a==0. {
            panic!("DIVISION BY ZERO")
        }
        let b = stack.pop().unwrap();

        stack.push(b/a);

        Ok(())
    });

    operations
}