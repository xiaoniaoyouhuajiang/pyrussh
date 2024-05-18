import _pyssh as pyssh
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from utils import print_call_time

@print_call_time
def pyssh_serialize(num: int):
    for _ in range(num):
        client = pyssh.Client(args.host, str(args.port), args.username, args.password)

@print_call_time
def pyssh_thread_spawn(num: int, thread_num: int):
    with ThreadPoolExecutor(max_workers=thread_num) as executor:
        task_list = list()
        for _ in range(num):
            task_list.append(executor.submit(pyssh.Client, args.host, str(args.port), args.username, args.password))
        for future in as_completed(task_list):
            _ = future.result()


parser = argparse.ArgumentParser(description='Get auth info from cmd')
parser.add_argument('--host', type=str, help='host ip')
parser.add_argument('--port', type=int, help='port number')
parser.add_argument('--username', type=str, help='username')
parser.add_argument('--password', type=str, help='password')

args = parser.parse_args()

pyssh_serialize(10)
pyssh_thread_spawn(10, 4)

