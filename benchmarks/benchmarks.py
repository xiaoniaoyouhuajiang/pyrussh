import paramiko
import argparse
import pyrussh
from utils import print_call_time 
from concurrent.futures import ThreadPoolExecutor, as_completed

test_str = "h" * 10000
test_file = "/tmp/test_file"

@print_call_time
def paramiko_batch_exec(num: int, thread_num:int):
    def paramiko_execute():
        ssh = paramiko.SSHClient()
        ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        ssh.connect(args.host, args.port, args.username, args.password)
        stdin, stdout, stderr = ssh.exec_command('ls -al')
        output = stdout.read().decode('utf-8')
        error = stderr.read().decode('utf-8')
        if error:
            print(error)

    with ThreadPoolExecutor(max_workers=thread_num) as executor:
        task_list = list()
        for _ in range(num):
            task_list.append(executor.submit(paramiko_execute))
        for future in as_completed(task_list):
            _ = future.result()

@print_call_time
def pyrussh_batch_exec(num: int, thread_num: int):
    hosts = ['127.0.0.1'] * num
    pool = pyrussh.SshPool(hosts, user=args.username, passwd=args.password, thread_num=thread_num)
    results = pool.execute('ls -al')
    for result in results:
        assert result[1][1] == 0

@print_call_time
def paramiko_auth():
    ssh = paramiko.SSHClient()
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    ssh.connect(args.host, args.port, args.username, args.password)

print_call_time
def pyrussh_auth(): 
    client = pyrussh.Client(args.host, str(args.port), args.username, args.password)

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
def pyrussh_upload():
    client = pyrussh.Client(args.host, str(args.port), args.username, args.password)
    client.upload_file(test_str, test_file)

parser = argparse.ArgumentParser(description='Get auth info from cmd')
parser.add_argument('--host', type=str, help='host ip')
parser.add_argument('--port', type=int, help='port number')
parser.add_argument('--username', type=str, help='username')
parser.add_argument('--password', type=str, help='password')

args = parser.parse_args()

paramiko_auth()
pyrussh_auth()
paramiko_upload()
pyrussh_upload()
paramiko_batch_exec(10, 6)
pyrussh_batch_exec(10, 6)

