import argparse
import re

def main(input):
    f = open(input, 'r')
    res_sum = 0
    for line in f.readlines():
        line = line.replace('\n', '')
        game = re.findall(r'Game (\d+)', line)
        test=True
        for tmp in line.split(";"):
            res = re.findall(r'(\d+) (blue|red|green)', tmp)
            if len(res)>0:
                sums = {'red' : sum([int(x[0]) for x in res if x[1] == 'red']),
                        'blue' : sum([int(x[0]) for x in res if x[1] == 'blue']),
                        'green' : sum([int(x[0]) for x in res if x[1] == 'green']),
                        }
                # print(line)
                # print(res)
                # print(sums)
                if sums['red'] < 13 and sums['blue'] < 15 and sums['green'] < 14:
                    pass
                else:
                    test=False
        if test and game:
            print(line)
            print(game)
            print(int(game[0]))
            res_sum += int(game[0])
    print(res_sum)
    pass

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='',
                    description='')
    parser.add_argument('filename')
    args = parser.parse_args()
    main(args.filename)
