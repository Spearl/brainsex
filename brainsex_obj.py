#!/usr/bin/env python
from sys import stdout, stdin, argv

class Interpreter(object):
    def __init__(self, chars):
        self.code = [c for c in chars if c in self.lookup]
        self.loop_map = create_loop_map(self.code)
        self.data = bytearray(30000)
        self.data_view = memoryview(self.data)
        self.ip = self.dp = 0

    def dp_right(self):
        self.dp += 1

    def dp_left(self):
        self.dp -= 1

    def dp_inc(self):
        self.data[self.dp] = (self.data[self.dp] + 1) % 256

    def dp_dec(self):
        self.data[self.dp] = (self.data[self.dp] - 1) % 256

    def put_dp(self):
        stdout.write(self.data_view[self.dp])

    def get_dp(self):
        self.data_view[self.dp] = stdin.read(1)

    def cond_jump_fwd(self):
        if not self.data[self.dp]:
            self.ip = self.loop_map[self.ip]

    def cond_jump_back(self):
        if self.data[self.dp]:
            self.ip = self.loop_map[self.ip]

    lookup = {
        '>': dp_right,
        '<': dp_left,
        '+': dp_inc,
        '-': dp_dec,
        '.': put_dp,
        ',': get_dp,
        '[': cond_jump_fwd,
        ']': cond_jump_back,
    }

    def run(self):
        while self.ip < len(self.code):
            self.lookup[self.code[self.ip]](self)
            self.ip += 1


def create_loop_map(code):
    loop_stack, loop_map = [], {}
    for pos, i in enumerate(code):
        if i == '[': 
            loop_stack.append(pos)
        elif i == ']':
            entrance = loop_stack.pop()
            loop_map[entrance], loop_map[pos] = pos, entrance
    return loop_map


if __name__ == '__main__':
    Interpreter(open(argv[1]).read()).run()
