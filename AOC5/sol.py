def readFile():
    with open('C:/Users/ablancov/Desktop/sandbox/AOC5/testInput') as f:
        return [int(l.strip()) for l in f]


def main():
    inst = readFile()
    i = 0
    steps = 0
    try:
        while True:
            if i < 0:
                raise
            mov = inst[i]
            inst[i] += 1
            i += mov
            steps += 1
    except:
        print(steps)


if __name__ == '__main__':
    main()
