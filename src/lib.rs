use anyhow::Result;
use pyo3::prelude::*;
use ssh2::Session;
use std::{io::Read, net::TcpStream};

#[pyclass]
struct Client {
    sess: ssh2::Session,
}

#[pymethods]
impl Client {
    #[new]
    fn new_session(host: &str, port: &str, user: &str, passwd: &str) -> PyResult<Self> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(user, passwd).unwrap();
        assert!(sess.authenticated());
        Ok(Client { sess: sess })
    }

    fn run_command(&mut self, command: &str) -> PyResult<String> {
        let mut result = String::new();
        let mut channel = self.sess.channel_session().unwrap();
        channel.exec(command).unwrap();
        channel.read_to_string(&mut result).unwrap();
        Ok(result)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyssh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
