from asyncio import get_running_loop


class Pool:
    def __init__(self):
        self._loop = get_running_loop()

