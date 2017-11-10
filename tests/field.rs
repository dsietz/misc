extern crate test_data_generation;

#[cfg(test)]
mod tests {
	use test_data_generation::data::field::{Field, FieldType};
    
    #[test]
    // ensure a field can be set to a String value
    fn set_field_string(){
    	let mut fld =  Field::new("field_1".to_string());
    	
    	fld.set("my value".to_string());
    }
    
    #[test]
    // ensure a field can be set to a integer value
    fn set_field_integer(){
    	let mut fld =  Field::new("field_1".to_string());
    	
    	let n1 :u8 = 100;
    	fld.set(n1);
    }
    
    #[test]
    // ensure the data type for the field can be set
    fn set_field_type(){
    	let mut fld =  Field::new("field_1".to_string());
    	
    	let n1 :u8 = 100;
    	fld.set_field_type(&n1);
    }    
    
    #[test]
    // ensure a field can retrieve a String value
    fn get_field(){
    	let mut fld =  Field::new("field_1".to_string());
    	
    	fld.set("my value".to_string());
    	assert_eq!(fld.get().downcast_ref::<String>().unwrap().len(), 8);
    }
}