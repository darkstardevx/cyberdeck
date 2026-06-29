import time
import json
import subprocess
import os

METRICS = "/tmp/cyberdeck_metrics.json"
STATE = "/tmp/cyberdeck_state.json"

def run(cmd):
    return subprocess.getoutput(cmd)

def load_state():
    if os.path.exists(STATE):
        return json.load(open(STATE))
    return {"mode": "balanced"}

def cpu():
    return run("grep 'cpu ' /proc/stat")

def temp():
    return run("sensors")

def mem():
    return run("free -m")

def disk():
    return run("df -h")

def net():
    return run("cat /proc/net/dev")

while True:
    state = load_state()

    data = {
        "state": state,
        "cpu": cpu(),
        "temp": temp(),
        "mem": mem(),
        "disk": disk(),
        "net": net()
    }

    json.dump(data, open(METRICS, "w"))

    time.sleep(1)
