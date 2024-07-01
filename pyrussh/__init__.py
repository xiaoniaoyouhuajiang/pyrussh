__version__ = '0.1.0'

from ._pyssh import *

class SshPool():
    def __init__(self, host_list, user, passwd, port='22', thread_num=6):
        pool = Pool()
        for host in host_list:
            pool.add_config(host=host, port=port, user=user, passwd=passwd)
        pool.connect(thread_num)
        self.pool

    def execute(self, command):
        return self.pool.execute_all(command)


__all__ = ('Client', 'SshPool')
