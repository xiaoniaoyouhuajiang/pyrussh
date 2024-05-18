import paramiko
import argparse
import _pyssh as pyssh
from .utils import print_call_time 

@print_call_time
def paramiko_auth():
    ssh = paramiko.SSHClient()
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    ssh.connect(args.host, args.port, args.username, args.password)

@print_call_time
def pyssh_auth():
    client = pyssh.Client(args.host, str(args.port), args.username, args.password)


parser = argparse.ArgumentParser(description='Get auth info from cmd')
parser.add_argument('--host', type=str, help='host ip')
parser.add_argument('--port', type=int, help='port number')
parser.add_argument('--username', type=str, help='username')
parser.add_argument('--password', type=str, help='password')

args = parser.parse_args()

paramiko_auth()
pyssh_auth()

