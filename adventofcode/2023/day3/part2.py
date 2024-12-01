import argparse
import numpy as np
from scipy import signal
import re

def main(input):
    f = open(input, 'r')
    content = f.readlines()
    n_lines = len(content)
    n_column = len(content)
    
    arr= np.array([[0 for _ in range(n_column)] for _ in range(n_lines)])
    for i, line in enumerate(content):
        res = re.finditer(r"\d+", line)
        for m in res:
            for x in range(m.start(), m.end()):
                arr[i,x] = int(m.group(0))

    numbers=[]
    for i in range(1,n_lines):
        for j in range(1,len(content[i])):
            if content[i][j] in "*":
                # print(content[i][j])
                # print(set([ arr[i-k][j-l] for k in range(-1,2) for l in range(-1,2) ]))
                res = set([ arr[i-k][j-l] for k in range(-1,2) for l in range(-1,2) ])
                res.remove(0)
                if len(res)>=2:
                    mul = 1
                    for x in res:
                        mul*=x
                    numbers.extend( [mul] )
    print(sum(numbers))
    pass

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='',
                    description='')
    parser.add_argument('filename')
    args = parser.parse_args()
    main(args.filename)

