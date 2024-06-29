import paramiko
import argparse
import pyssh
from .utils import print_call_time 

test_str = "h" * 10000
test_file = "/tmp/test_file"

@print_call_time
def paramiko_auth():
    ssh = paramiko.SSHClient()
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    ssh.connect(args.host, args.port, args.username, args.password)

@print_call_time
def pyssh_auth(): 
    client = pyssh.Client(args.host, str(args.port), args.username, args.password)

@print_call_time
def paramiko_upload():
    ssh = paramiko.SSHClient()
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    ssh.connect(args.host, args.port, args.username, args.password)
    ftp = ssh.open_sftp()
    file = ftp.file(test_file, "a", -1)
    file.write(test_str)
    file.flush()
    ftp.close()
    ssh.close()

@print_call_time
def pyssh_upload():
    client = pyssh.Client(args.host, str(args.port), args.username, args.password)
    client.upload_file(test_str, test_file)

parser = argparse.ArgumentParser(description='Get auth info from cmd')
parser.add_argument('--host', type=str, help='host ip')
parser.add_argument('--port', type=int, help='port number')
parser.add_argument('--username', type=str, help='username')
parser.add_argument('--password', type=str, help='password')

args = parser.parse_args()

paramiko_auth()
pyssh_auth()
paramiko_upload()
pyssh_upload()

