# Q20のテストケース作成
import random

N_MIN = 1
# N_MAX = 10 ** 5
N_MAX = 5

ABC_MIN = 1
ABC_MAX = 10 ** 9
# ABC_MAX = 6

# N = random.randint(N_MIN, N_MAX)
N= 6
A = [str(random.randint(ABC_MIN, ABC_MAX)) + " " for i in range(N)]
B = [str(random.randint(ABC_MIN, ABC_MAX)) + " " for i in range(N)]
C = [str(random.randint(ABC_MIN, ABC_MAX)) + " " for i in range(N)]

with open("tekito.txt", "w") as f:
    f.writelines(str(N))
    f.writelines("\n")
    f.writelines(A)
    f.writelines("\n")
    f.writelines(B)
    f.writelines("\n")
    f.writelines(C)
