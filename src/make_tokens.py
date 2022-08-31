import tokenize as t
from io import BytesIO

def tokenize(input):
    val = list(t.tokenize(BytesIO(input.encode('utf-8')).readline))
    print(val)
    return val
