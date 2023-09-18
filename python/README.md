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
