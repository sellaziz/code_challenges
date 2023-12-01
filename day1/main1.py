import argparse
import re

converter = {
        '1' : '1',
        '2' : '2',
        '3' : '3',
        '4' : '4',
        '5' : '5',
        '6' : '6',
        '7' : '7',
        '8' : '8',
        '9' : '9',
        # '0' : '0',
        'one' : '1',
        'two' : '2',
        'three' : '3',
        'four' : '4',
        'five' : '5',
        'six' : '6',
        'seven' : '7',
        'eight' : '8',
        'nine' : '9',
        # 'zero' : '0',
        }
def main(input):
    f = open(input, 'r')
    results=[]
    sum = 0
    for line in f.readlines():
        # print(line)
        digits=[]
        for key, _ in converter.items():
            digits.extend(re.finditer(key, line))

        tmp=[]
        for m in digits:
            tmp.append((m.start(), m.group(0)))

        print(tmp)
        tmp.sort(key=lambda x: x[0])
        digits = [temp[1] for temp in tmp]
        # digits = re.findall(r'(one|two|three|four|five|six|seven|eight|nine|zero|\d)', line)
        # print(digits)
        if digits:
            result=int(converter[digits[0]]+converter[digits[-1]])
            # print(f'number: {result}')
            print(f'number: {line[:-2]}#{digits}#{result}')
            results.append(result)
            sum = sum + result
    print(sum)
    print(results)
    pass

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='',
                    description='')
    parser.add_argument('filename')
    args = parser.parse_args()
    main(args.filename)
