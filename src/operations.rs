use phf::phf_map;

pub type Operation = fn(&mut Vec<f64>) -> Result<(), &'static str>;

pub static OPERATIONS: phf::Map<&'static str, Operation> = phf_map! {
    "+" => |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Addition requires two arguements!")
        }

        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b+a);

        Ok(())
    },

    "-" => |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Subtraction requires two arguements!")
        }

        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b-a);

        Ok(())
    },

    "*" => |stack|{
        if stack.len() < 2 {
            return Err("ERROR: Multiplication requires two arguements!")
        }

        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b*a);

        Ok(())
    },

    "/" => |stack|{
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
    }
};
