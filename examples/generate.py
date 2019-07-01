import csv
import random

f = open("data.txt", "a")
for _ in range(1000000):
    f.write(str(random.randint(0, 100)) + " " +
            str(random.randint(0, 10000)) + "\n")
f.close()
