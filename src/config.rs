pub struct SshConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub passwd: String,
}

impl SshConfig {
    fn new(host: String, port: String, user: String, passwd: String) -> Self {
        SshConfig { host, port, user, passwd }
    }
}
