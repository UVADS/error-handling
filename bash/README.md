# Error Handling in `bash`

## Exit Codes

Every process in a Linux system returns an exit code upon execution. In cases of a successful process the exit code is `0` (much like the `HTTP 200 OK` reponse code in the web HTTP protocol) and unsuccessful exit codes are represented by other integers. To determine the exit code returned for the last process in a bash shell, use:

```bash
$ date
Mon Feb 12 16:51:55 EST 2024

$ echo $?
0
```
To test for a non-zero exit code, you can invoke an error deliberately:
```bash
$ datez
bash: datez: command not found

$ echo $?
127
```
Here `127` represents not only an unsuccessful execution (i.e. non-zero) but specifically a "command not found" message since `datez` does not exist in the path.

The following are [reserved codes](https://tldp.org/LDP/abs/html/exitcodes.html) determined by the Linux Documentation Project:

- `1` - Catch-all for general errors
- `2` - Misuse of shell built-ins
- `126` - Command invoked cannot execute
- `127` - “command not found”
- `128` - Invalid argument to exit
- `128+n` - Fatal error signal “n”
- `130` - Script terminated by Control-C
- `255\*` - Exit status out of range

### Setting Error Codes

Shell scripts can return exit codes of your choosing. It would not make sense to return anything other than `0` upon successful completion, but any kind of error could return an integer you select.

```
#!/bin/bash

/usr/bin/<mycommand>
exit 14
```

Here

### Evaluating Error Codes

Commands run within a portion of a shell script can be evaluated using normal if/then/else logic:

```bash
#!/bin/bash

date

if [ $? -eq 0 ]
then
  echo "Success"
  exit 0
else
  echo "Failure" >&2
  exit 1
fi
```
Notice the `then` and `else` stanzas each return their own exit codes.

### Suppressing / Customizing Exit Codes

You can also write conditional exit codes within specific commands in your script. 


## Logging

Logging is a useful way of capturing errors and other informative output
from your code. Broadly interpreted, logging can mean many things:

- Appending a log message to a file
- Sending a log message to a database
- Sending a log message to a remote logging service
- Notifying a user via email or other messaging service

Logging to files is made simple with basic output appended to a consistent log file. Here are some basic examples:

```bash
NOW=`date +'%Y-%m-%d-%H:%M:%S'`
STATUS="Warning"
MSG="This is a warning message in the log file"

echo $NOW - $STATUS - $MSG >> /var/log/app-error.log'

```

The log results from above:
```
2023-09-18 14:09:28,328 - WARNING - This is a warning message in the log file
```

## Read More

- [`bash` Errors HOWTO]()
- [`bash` Logging HOWTO]()
