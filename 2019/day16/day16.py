import numpy as np

pattern =[0,1,0,-1]

with open("input.in") as f:
    signal = f.readline()
    signal=np.array([int(c) for c in signal])
    signal = np.repeat(signal,10_000)
    phase_matrix=np.array([ [pattern[(j // i)%4] for j in range(1,len(signal)+1) ]for i in range(1,len(signal)+1)])
    for i in range(1,101):
        signal = list(map(lambda x: np.abs(x)%10,(phase_matrix)@signal))
    print(signal)

# signal =np.array([1,2,3,4,5,6,7,8])
# phase_matrix=np.array([ [pattern[(j // i)%4] for j in range(1,len(signal)+1) ]for i in range(1,len(signal)+1)])
# signal = list(map(lambda x: np.abs(x)%10,(phase_matrix)@signal))
# print(signal)