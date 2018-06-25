// Copyright 2018 David Sietz and [`test-data-generator` contributors](https://github.com/dsietz/test-data-generator/blob/master/CONTRIBUTORS.md).
// Licensed under the MIT license
// (see LICENSE or <https://opensource.org/licenses/Apache-2.0>)
//
//!
//! The are multiple ways to use the Test Data Generation library. It all depends on your intent.
//!
//! ### Profile
//!
//! The easiest way is to use a Profile. The `profile` module provides functionality to create a profile on a data sample (Strings).
//! Once a profile has been made, data can be generated by calling the _pre_generate()_ and _generate()_ functions, in that order.
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::profile::Profile;
//!
//! fn main() {
//!     // analyze the dataset
//! 	let mut data_profile =  Profile::new();
//!
//!     // analyze the dataset
//! 	data_profile.analyze("Smith, John");
//! 	data_profile.analyze("Doe, John");
//! 	data_profile.analyze("Dale, Danny");
//! 	data_profile.analyze("Rickets, Ronney");
//!
//!     // confirm 4 data samples were analyzed
//!    	assert_eq!(data_profile.patterns.len(), 4);
//!
//!     // prepare the generator
//!     data_profile.pre_generate();
//!
//!     // generate some data
//!    	println!("The generated name is {:?}", data_profile.generate());
//! }
//! ```
//!
//! You can also export (archive as JSON file) the profile for later use.
//! This allows for the algorithm to be retrieved without having to store the actual data that was analyzed.
//!
//!	```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::profile::Profile;
//!
//! fn main() {
//!		//create a profile and analyze some data
//!		let mut old_profile =  Profile::new();
//!		old_profile.analyze("Smith, John");
//!		old_profile.analyze("O'Brian, Henny");
//!		old_profile.analyze("Dale, Danny");
//!		old_profile.analyze("Rickets, Ronney");
//!
//!		old_profile.pre_generate();
//!
//!		//save the profile for later
//!		assert_eq!(old_profile.save("./tests/samples/sample-00-profile").unwrap(), true);
//!
//!		// create a new profile from the archive json file
//!		let mut new_profile = Profile::from_file("./tests/samples/sample-00-profile");
//!
//!		// generate some data. NOTE that the pre-generate() was already called prior to saving
//!     println!("The generated name is {:?}", new_profile.generate());
//! }
//! ```
//!
//! ### Data Sample Parser
//!
//! If you are using CSV files of data samples, then you may wish to use a Data Sample Parser.
//! The `data_sample_parser` module provides functionality to read sample data, parse and analyze it, so that test data can be generated based on profiles.
//!
//! ```
//! extern crate test_data_generation;
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!     let mut dsp = DataSampleParser::new();
//!     dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
//!
//!     println!("My new name is {} {}", dsp.generate_record()[0], dsp.generate_record()[1]);
//!     // My new name is Abbon Aady
//! }
//! ```
//!
//! You can also save the Data Sample Parser (the algorithm) as an archive file (json) ...
//!
//! ```
//! extern crate test_data_generation;
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!     let mut dsp =  DataSampleParser::new();
//!     dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
//!
//!     assert_eq!(dsp.save(&String::from("./tests/samples/sample-01-dsp")).unwrap(), true);
//! }
//! ```
//!
//! and use it at a later time.
//!
//! ```
//! extern crate test_data_generation;
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!     let mut dsp = DataSampleParser::from_file(&String::from("./tests/samples/sample-01-dsp"));
//!
//! 	println!("Sample data is {:?}", dsp.generate_record()[0]);
//! }
//! ```
//!
//! You can also generate a new csv file based on the data sample provided.
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!     let mut dsp =  DataSampleParser::new();
//!
//!    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
//!    	dsp.generate_csv(100, &String::from("./tests/samples/generated-01.csv")).unwrap();
//! }
//! ```
//!

#![crate_type= "lib"]
#![crate_name = "test_data_generation"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;
extern crate regex;
extern crate rand;
extern crate crossbeam;
extern crate csv;
extern crate oozie;

#[macro_use]
pub mod macros;
pub mod shared;
pub mod data_sample_parser;
pub mod configs;
pub mod profile;
