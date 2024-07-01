mod pool;
pub use pool::SshPool;
mod config;
pub use config::SshConfig;
use pyo3::{exceptions::PyValueError, prelude::*};
use ssh2::Session;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{i32, io::Read, net::TcpStream};

#[pyclass]
struct Client {
    sess: ssh2::Session,
}

#[pyclass]
struct Pool {
    inner: Option<SshPool>,
    configs: Vec<SshConfig>,
}

#[pymethods]
impl Pool {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Pool {
            inner: None,
            configs: vec![],
        })
    }

    fn add_config(&mut self, host: &str, port: &str, user: &str, passwd: &str) {
        self.configs.push(SshConfig {
            host: host.to_string(),
            port: port.to_string(),
            user: user.to_string(),
            passwd: passwd.to_string(),
        });
    }

    fn connect(&mut self, thread_number: u64) {
        if self.inner.is_none() {
            self.inner = Some(SshPool::new(&self.configs, thread_number));
        }
    }

    fn execute_all(&mut self, command: &str) -> PyResult<Vec<(String, (String, i32))>> {
        if self.inner.is_none() {
            return Err(PyValueError::new_err("not connected"));
        }

        let results = self.inner.as_mut().unwrap().run_command(command);
        Ok(results)
    }
}

#[pymethods]
impl Client {
    // todo: Agent/Pubkey auth
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

    fn run_command(&mut self, command: &str) -> PyResult<(String, i32)> {
        let mut result = String::new();
        let mut channel = self.sess.channel_session().unwrap();
        channel.exec(command).unwrap();
        channel.read_to_string(&mut result).unwrap();
        let exit_code = channel.exit_status().unwrap();
        Ok((result, exit_code))
    }

    fn upload_file(&mut self, content: &str, target_path: &str) -> PyResult<bool> {
        let content_bytes = content.as_bytes();
        let mut remote_file = self
            .sess
            .scp_send(
                Path::new(target_path),
                0o644,
                content_bytes.len().try_into().unwrap(),
                None,
            )
            .unwrap();
        remote_file.write_all(content_bytes).unwrap();
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();
        Ok(true)
    }

    fn download_file(&mut self, target_path: &str) -> PyResult<String> {
        let (mut remote_file, _) = self.sess.scp_recv(Path::new(target_path)).unwrap();
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        match String::from_utf8(contents) {
            Ok(result) => Ok(result),
            _ => Err(PyValueError::new_err("String Conversion failed.")),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyssh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    m.add_class::<Pool>()?;
    Ok(())
}
