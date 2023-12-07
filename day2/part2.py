import argparse
import re

def main(input):
    f = open(input, 'r')
    res_sum = 0
    for line in f.readlines():
        line = line.replace('\n', '')
        game = re.findall(r'Game (\d+)', line)
        test=True
        sums_set = {'red' : 0,
                'blue' : 0,
                'green' : 0,
                }
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
                # if sums['red'] < 13 and sums['blue'] < 15 and sums['green'] < 14:
                #     pass
                # else:
                #     test=False
                for key, _ in sums_set.items():
                    if sums_set[key] > sums[key]  and sums_set[key] != 0:
                        sums_set[key] = sums_set[key]
                    else :
                        sums_set[key] = sums[key]
        print(line)
        print(sums_set)
        res_sum += int(sums_set['red']*sums_set['blue']*sums_set['green'])
        # if test and game:
        #     print(line)
        #     print(game)
        #     print(int(game[0]))
    print(res_sum)
    pass

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='',
                    description='')
    parser.add_argument('filename')
    args = parser.parse_args()
    main(args.filename)
