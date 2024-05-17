import pyssh
import argparse
from utils import print_call_time

@print_call_time
def pyssh_serialize(num: int):
    for _ in range(num):
        client = pyssh.Client(args.host, str(args.port), args.username, args.password)


parser = argparse.ArgumentParser(description='Get auth info from cmd')
parser.add_argument('--host', type=str, help='host ip')
parser.add_argument('--port', type=int, help='port number')
parser.add_argument('--username', type=str, help='username')
parser.add_argument('--password', type=str, help='password')

args = parser.parse_args()

pyssh_serialize(10)

