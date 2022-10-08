line0 = "[[bin]]"

for i in range(100):
    line1 = 'name = "q{:0=3}"'.format(i + 1)
    line2 = 'path = "src/q{:0=3}.rs"'.format(i + 1)
    print(line0)
    print(line1)
    print(line2)
    print("")
