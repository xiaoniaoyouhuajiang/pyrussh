use pyo3::{exceptions::PyValueError, prelude::*};
use ssh2::Session;
use std::io::Write;
use std::path::Path;
use std::{i32, io::Read, net::TcpStream};

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
        remote_file.write(content_bytes).unwrap();
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
            Ok(result) => return Ok(result),
            _ => return Err(PyValueError::new_err("String Conversion failed.")),
        };
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyssh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
