
import sys
import time

print "WE are in Python script"

line = sys.stdin.readline()
while line:
    print line
    break;

print "Waiting ...."
time.sleep(10)
print "Waiting Ended"