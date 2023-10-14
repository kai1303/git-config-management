use prettytable::Table;

use crate::config::Config;

pub fn print_configs(configs: Vec<Config>) {
    let mut table = Table::new();
    table.add_row(row!["Host", "Host Name", "User", "SSH Key"]);
    for c in configs {
        table.add_row(row![c.host, c.host_name, c.user, c.identity_file]);
    }
    table.printstd();
}

pub fn print_config(config: Config) {
    let mut table = Table::new();
    table.add_row(row!["Host", "Host Name", "User", "SSH Key"]);
    table.add_row(row![config.host, config.host_name, config.user, config.identity_file]);
    table.printstd();
}
