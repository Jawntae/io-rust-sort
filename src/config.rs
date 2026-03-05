//! Defines configuration info to run project

///represents configuration info
pub struct Config {
    ///input file
    pub input: String,

    ///output file
    pub output: String,

}


impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err ("input file not found"),
        };


        let output = match args.next() {
            Some(arg) => arg,
            None => return Err("output file cannot be created"),
        };

        Ok(Config {
            input,
            output
        })
    }
}
