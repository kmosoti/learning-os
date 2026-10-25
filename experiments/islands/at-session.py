"""Private D-Bus/AT-SPI session, never replaces the user's screen reader."""
import os
import pathlib
import subprocess
import time
import ast

output = pathlib.Path(os.environ.get("OUT", "/tmp/learningos-at-initial"))
output.mkdir(parents=True, exist_ok=True)
environment = dict(os.environ, DISPLAY=":99", XDG_CONFIG_HOME="/tmp/learningos-at-config",
                   XDG_CACHE_HOME="/tmp/learningos-at-cache", XDG_DATA_HOME="/tmp/learningos-at-data",
                   PLAYWRIGHT_BROWSERS_PATH="/home/kmosoti/.cache/ms-playwright",
                   GTK_USE_PORTAL="0", NO_AT_BRIDGE="0", OUT=str(output), ORCA_WAIT_MS="6000")
bus = subprocess.Popen(["/usr/libexec/at-spi-bus-launcher", "--launch-immediately"], env=environment,
                       stdout=open(output / "bus.stdout", "w"), stderr=open(output / "bus.stderr", "w"))
time.sleep(1)
address = subprocess.check_output(["gdbus", "call", "--session", "--dest", "org.a11y.Bus",
                                  "--object-path", "/org/a11y/bus", "--method", "org.a11y.Bus.GetAddress"], text=True)
environment["AT_SPI_BUS_ADDRESS"] = ast.literal_eval(address)[0]
handles = []
processes = []
try:
    browser_process = None
    for name, command in [("registry", ["/usr/libexec/at-spi2-registryd"]),
                          ("orca", ["orca", f"--debug-file={output / 'orca.log'}"])]:
        if name == "orca":
            browser_process = subprocess.Popen(["node", "experiments/islands/at-firefox.mjs"], env=environment)
            time.sleep(4)
        handle = open(output / f"{name}.stderr", "w")
        handles.append(handle)
        processes.append(subprocess.Popen(command, env=environment, stdout=handle, stderr=handle))
        time.sleep(1)
    browser_process.wait(timeout=120)
    print({"browser_exit": browser_process.returncode, "orca_status": processes[1].poll()})
finally:
    for process in processes + [bus]:
        if process.poll() is None:
            process.terminate()
            process.wait(timeout=10)
    for handle in handles:
        handle.close()
