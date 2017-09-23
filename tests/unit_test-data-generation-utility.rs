extern crate test_data_generation;

use test_data_generation::test_data_generator::{data_sample_parser,profile};

// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
	use data_sample_parser::DataSampleParser;
	//use test_data_generation::test_data_generator::{data_sample_parser,configs, params};
	use test_data_generation::test_data_generator::params::Params;
	use test_data_generation::test_data_generator::profile::pattern::Pattern;
    
    #[test]
    // ensure the the default parameters are set
    fn parameters_defaults(){
    	let params =  Params::new();    	
    	assert!(true);
    	//assert!(params.get_config_file().is_empty());
    	//assert!(params.get_log_file().is_empty());
    	//assert!(params.get_tool().is_empty());
    	//assert!(params.get_verbose().is_empty());
    }
    
    #[test]
    // special characters for profiling
    fn profile_analyze(){
    	let mut pattrn =  Pattern::new();
    	let word = pattrn.analyze("H3Ll0?");    		
    	assert_eq!(word, "[A-Z][0-9][A-Z][a-z][0-9][\\\\\\^\\$\\.\\|\\?\\*\\+\\(\\)\\[\\]\\{\\}]");
    }
    
    #[test]
    // special characters for profiling
    fn profile_special_characters(){
    	let pattrn =  Pattern::new();    		
    	assert_eq!(pattrn.special_char.len(), 14);
    }
}