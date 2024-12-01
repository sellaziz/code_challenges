import argparse
import re

def main(input):
    f = open(input, 'r')
    results=[]
    sum = 0
    for line in f.readlines():
        # print(line)
        digits = re.findall(r'([0-9])', line)
        # print(digits)
        if digits:
            result=int(digits[0]+digits[-1])
            # print(f'number: {result}')
            results.append(result)
            sum = sum + result
    print(sum)
    pass

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='',
                    description='')
    parser.add_argument('filename')
    args = parser.parse_args()
    main(args.filename)
