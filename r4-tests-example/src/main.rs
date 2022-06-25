use std::process;

fn main() {
    println!("Hello, world!");
}

fn get_process_id() -> u32 {
    process::id()
}



#[cfg(test)]
mod tests {
    use super::get_process_id;
    
    #[test]
    fn test_get_process_id() -> () {
        assert!(get_process_id() > 0);
    }

    #[test]
    fn test_get_process_id_2() -> () {
        assert_ne!(get_process_id(), 0, "PID of this process cannot be zero.")
    }
}