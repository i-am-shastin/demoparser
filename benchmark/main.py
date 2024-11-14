from demoparser2 import DemoParser
import time
import glob
import statistics
import tracemalloc

tracemalloc.start()

files = glob.glob("D:/Work/cs2-demo-stats/.demos/*")
results = []

for i in range(10):
    before = time.time()

    for file in files:
        parser = DemoParser(file)
        df = parser.parse_event("player_death", player=["X", "Y"])

    result = time.time() - before
    results.append(result)
    print(result)

print('\033[35m-- Median: ' + str(statistics.median(results)) + ', mean: ' + str(statistics.mean(results)) + '\033[0m')
current, peak = tracemalloc.get_traced_memory()
print(f"\033[35m-- Memory usage: {int(current / 16**3) / 10}MB; {int(peak / 16**3) / 10}MB\033[0m")

tracemalloc.stop()