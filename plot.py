from collections import defaultdict
import csv
import matplotlib.pyplot as plt
import sys
import time

# Occupancy counts are grouped into 15-minute bins
# using the following time encoding as a key:
#
# ```
# (W * 24 * 60) + (H * 60) + M
# ```
#
# Where:
# - W is the day of the week
# - H is the hour of the day
# - M is the minute of the hour
#
# Each bucket contains the running sum and
# total count for averaging.
buckets = defaultdict(lambda: [0, 0])

with open(sys.argv[1], "r") as file:
    reader = csv.reader(file)
    for [t, c] in reader:
        t = time.localtime(int(t))
        c = int(c)

        key = t.tm_wday * 24 * 60 + t.tm_hour * 60 + ((t.tm_min // 15) * 15)

        buckets[key][0] += c
        buckets[key][1] += 1

times = sorted([key for key in buckets.keys()])
counts = [buckets[key][0] / buckets[key][1] for key in times]

ticks = []
labels = []
prev_hour = 0
for time in times:
    hour = (time % (24 * 60)) // 60
    if hour > prev_hour + 2 or hour < prev_hour:
        prev_hour = hour
        ticks.append(time)
        labels.append("{}:00{}".format(
            hour % 12 if hour % 12 != 0 else 12,
            "AM" if hour < 12 else "PM"
        ))

plt.scatter(times, counts)
plt.title("Climbing Gym Occupancy")
plt.xlabel("Time (Mon - Sun)")
plt.xticks(ticks, labels, rotation='vertical')
plt.ylabel("Occupants")
plt.show()
