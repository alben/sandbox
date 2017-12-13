TEST1 = '1122'
SOL1 = 3
TEST2 = '1111'
SOL2 = 4
TEST3 = '1234'
SOL3 = 0
TEST4 = '91212129'
SOL4 = 9


target = TEST2
total = 0
for i, item in enumerate(target, 1):
    i = i % len(target)
    if item == target[i]:
        total += int(item)

print(total)

