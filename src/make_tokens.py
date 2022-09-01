import tokenize as t
import token
from io import BytesIO

def get_token_numbers():
    lookup = {}
    for k in token.__dict__:
        v = token.__dict__[k]
        if isinstance(v, int):
            lookup[v] = k
    return lookup


def tokenize(input):
    val = list(t.tokenize(BytesIO(input.encode('utf-8')).readline))
    return val
