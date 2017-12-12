def readFile():
    with open('C:/Users/ablancov/Desktop/sandbox/AOC4/testInput') as f:
        return [l.strip() for l in f]

def main():
    lines = readFile()
    for line in lines:
        words = line.split()
        valid = False
        while len(words) > 1:
            word = words.pop()
            if word in words:
                break
        else:
            valid = True
        if valid:
            print(line)

if __name__ == '__main__':
    main()
