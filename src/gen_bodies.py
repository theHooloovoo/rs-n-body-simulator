#!/usr/bin/env python3
import random
import sys

if len(sys.argv) != 2:
    sys.stderr.write("Please state how many bodies you want generated\n")
    sys.exit()

# n = int(sys.argv[1])

for n in range(1, int(sys.argv[1])+1 ):
    print( "body{}".format(n),
           # Mass
           random.expovariate(1.0),
           # Location
           random.uniform(-1000, 1000),
           random.uniform(-1000, 1000),
           random.uniform(-1000, 1000),
           # Velocity
           random.uniform(-0.05, 0.05),
           random.uniform(-0.05, 0.05),
           random.uniform(-0.05, 0.05)
         )
