
use std;


/// Builds a comma seperated list of objects as a string.
pub fn build_list_str<T: std::fmt::Show, I: Iterator<T>>(mut it: I) -> String
{
    let mut result = String::new();
    
    let mut is_first = true;
    
    for obj in it {
    
        // prepend a comma if we aren't the first object.
        if !is_first {
            result.push_str(", ");
        }
    
        result.push_str(format!("{}", obj).as_slice());
        
        is_first = false;
    }
    
    result
}
