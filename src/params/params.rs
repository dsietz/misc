use clap::{Arg, App, ArgMatches};

//set the default configuration values
matches = App::new("Test Data Generation")	
                          .version("1.0")
                          .author("dsietz")
                          .about("Made just for you!")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true)
                               .default_value("./config/tdg.yaml"))
                          .arg(Arg::with_name("log")
                               .short("l")
                               .long("log")
                               .value_name("FILE")
                               .help("set a custom logging configuration file - format YAML")
                               .takes_value(true)
                               .default_value("./config/log4rs.yaml"))     
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .help("explain what is being done"))
                          .get_matches();    
                                    
