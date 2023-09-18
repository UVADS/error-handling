# Error Handling in Python

## `try` / `except`

The foundation of error handling in Python is based on appropriate use 
of `try` and `except` statements in your code. You `try` to run your
functions and routines as you would expect to, but you must be ready
when an `except` occurs, i.e. a non-zero result that triggers an error
in Python.

Here is a simple framework:
```python

try:
  # do something
except Exception as e:
  # handle an error, where e is now an object for that error
  # and Exception is a base class for all errors.

```

The `try` stanza, if successful, will continue to run your process.
However, in the `except` stanza you may want to *do something* with
the raised exception:

1. Print it to the screen, if your script is run interactively.
3. Stop the process completely.
2. Log it to an error log for later review. (See below)

Here is a revised example, incorporating options 1 and 2 above:

```python
import sys

try:
  # do something
except Exception as e:
  # print out the error
  print(e)

  # stop the process and exit with a non-zero status
  sys.exit(1)

```

## Logging

Logging is a useful way of capturing errors and other informative output
from your code. Broadly interpreted, logging can mean many things:

- Appending a log message to a file
- Sending a log message to a database
- Sending a log message to a remote logging service
- Notifying a user via email or other messaging service

Logging to files is made simple with the `logging` package. This package
is generally part of standard Python distributions and does not need to be
installed. The `logging` package provides a `Logger` class that can be
instantiated and used to log messages to a file. Here are the basic options:

### Level
Decide the level of events you want to capture:

- Debug
- Info
- Warning
- Error
- Critical

### Filename
This should be the full path to your error log file.

### Filemode
The mode in which your `filename` is opened. The default is `a` for append.

### Format
The format of the log message. This is a string that can contain select output
from your code.

```python
import logging

logging.basicConfig(filename='/var/log/app-error.log', filemode='a', format='%(asctime)s - %(levelname)s - %(message)s')
logging.warning("This is a warning message in the log file")
logging.error("LOOK OUT! This is an error!!")
```

Results in an error log entry:
```
2023-09-18 14:09:28,328 - WARNING - This is a warning
2023-09-18 14:10:26,890 - ERROR - LOOK OUT! This is an error!!
```

Passing in the `e` exception message to be logged is then simple:

```python
import sys
import logging

# set up logging
logging.basicConfig(filename='/var/log/app-error.log', filemode='a', format='%(asctime)s - %(levelname)s - %(message)s')

# try/except things:
try:
  # do something
except Exception as e:
  # log the error
  logging.error(e)
  # stop the process and exit
  sys.exit(1)

```
