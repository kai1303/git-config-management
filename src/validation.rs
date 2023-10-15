use crate::cli::Cli;

pub fn validate_args_test_command(args: Cli){
     if args.key.is_none() {
        return;
    }
    let k = match args.key {
        Some(s) => s,
        None => String::from(""),
    };
}

pub fn validate_args_list_command() {

}

