import tokenize as t
import token
from io import StringIO

# Reads all the constants from the token module and makes them into a dictionary.
def get_token_numbers():
    lookup = {}
    for k in token.__dict__:
        v = token.__dict__[k]
        if isinstance(v, int):
            lookup[v] = k
    return lookup

# Read the tokens from an input string and returns them as a list.
def tokenize(input):
    val = list(t.generate_tokens(StringIO(input).readline))
    return val

# Returns the list of tokens augmented with the text descriptions from the tokens module.
# The numbers of these tokens can change, so the only reliable way to refer to a token type
# is with this string, and it also makes the code more readable.
def augment_tokens(input):
    o = []
    token_lookup = get_token_numbers()
    for t in tokenize(input):
        t.__setattr__('token_text', token_lookup[t.type])
        o.append(t)
    return o
