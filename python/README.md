# Error Handling & Logging in Python

## Error & Exception Handling

### Use `try` / `except` / `finally` for exceptions

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
### Exception Types

The examples above use a default, catch-all `Exception` object that is called for all error types. Printing this to the screen (or logging)
is somewhat informative, but does not always provide granular detail. But consider a couple of simple examples that would be helped by
identifying a *specific type* of error:

- User input should be an `integer`, but a `string` or `bool` is given. This represents a `TypeError`.
- A package is installed in the system but cannot be loaded. This represents an `ImportError`.

Code does not need to anticipate **all** possible errors happening for any given operation or process, but common patterns of error tend to
emerge over time and developers should catch them and then (a) log appropriately; and (b) try to remediate within the flow. For instance, if
a wrong data type is passed, error handling could inform the user and request new input, instead of exiting with an error code.

### Built-in Exception Types

Here is a complete list of built-in Exception Types:

**Base Classes**
- `BaseException` — Root of the entire exception hierarchy; catches everything including system-exit events.
- `Exception` — Base for all non-system-exiting exceptions; the standard catch-all for application errors.
- `ArithmeticError` — Base for math-related errors like division by zero or overflow.
- `LookupError` — Base for errors raised when a lookup (by index or key) fails.
- `OSError` — Base for operating system-related errors (also aliased as `IOError`, `EnvironmentError`).

**Arithmetic**
- `ZeroDivisionError` — Raised when dividing (or using modulo) by zero.
- `OverflowError` — Raised when a numeric result is too large to be represented.
- `FloatingPointError` — Raised when a floating-point operation fails (rarely used; requires `fpectl`).

**Lookup / Index / Key**
- `IndexError` — Raised when a sequence index is out of range.
- `KeyError` — Raised when a dictionary key is not found.

**Type & Value**
- `TypeError` — Raised when an operation is applied to an object of the wrong type.
- `ValueError` — Raised when a function receives an argument of the right type but an invalid value.
- `UnicodeError` — Base for Unicode encoding/decoding errors.
- `UnicodeDecodeError` — Raised when a byte sequence can't be decoded into a string.
- `UnicodeEncodeError` — Raised when a string can't be encoded into bytes.
- `UnicodeTranslateError` — Raised when a string can't be translated during a Unicode translation.

**Attribute & Name**
- `AttributeError` — Raised when an attribute reference or assignment fails on an object.
- `NameError` — Raised when a local or global name is not found.
- `UnboundLocalError` — Raised when a local variable is referenced before being assigned.

**Import & Module**
- `ImportError` — Raised when an import statement fails to find or load a module.
- `ModuleNotFoundError` — Raised specifically when the module being imported cannot be found at all.

**OS / IO**
- `FileNotFoundError` — Raised when a file or directory is requested but doesn't exist.
- `FileExistsError` — Raised when trying to create a file or directory that already exists.
- `PermissionError` — Raised when an operation lacks the required access rights.
- `IsADirectoryError` — Raised when a file operation is attempted on a directory.
- `NotADirectoryError` — Raised when a directory operation is attempted on a non-directory.
- `InterruptedError` — Raised when a system call is interrupted by an incoming signal.
- `TimeoutError` — Raised when a system-level operation times out.
- `BlockingIOError` — Raised when an operation would block on a non-blocking I/O resource.
- `BrokenPipeError` — Raised when writing to a pipe whose read end has been closed.
- `ChildProcessError` — Raised when an operation on a child process fails.
- `ConnectionError` — Base for connection-related errors.
- `ConnectionAbortedError` — Raised when a connection attempt is aborted by the remote end.
- `ConnectionRefusedError` — Raised when a connection attempt is refused by the remote end.
- `ConnectionResetError` — Raised when a connection is reset by the remote end.

**Runtime**
- `RuntimeError` — Raised when an error doesn't fit into any other category.
- `NotImplementedError` — Raised when an abstract method requires a subclass to provide an implementation.
- `RecursionError` — Raised when the maximum recursion depth is exceeded.

**Stop / Control Flow**
- `StopIteration` — Raised by `next()` to signal that an iterator has no more items.
- `StopAsyncIteration` — Raised by `__anext__()` to signal the end of an async iterator.
- `GeneratorExit` — Raised inside a generator when it is closed via `.close()`.
- `SystemExit` — Raised by `sys.exit()` to request interpreter shutdown.
- `KeyboardInterrupt` — Raised when the user presses Ctrl+C to interrupt execution.

**Memory & System**
- `MemoryError` — Raised when an operation runs out of memory.
- `SystemError` — Raised when the interpreter itself encounters an internal error.
- `ReferenceError` — Raised when a weak reference is accessed after its referent has been garbage collected.
- `BufferError` — Raised when a buffer-related operation can't be performed.

**Syntax & Encoding**
- `SyntaxError` — Raised when the parser encounters a syntax error in Python code.
- `IndentationError` — Raised when indentation is inconsistent or incorrect.
- `TabError` — Raised when indentation mixes tabs and spaces in an incompatible way.

**Async (3.11+)**
- `ExceptionGroup` — Wraps multiple exceptions that can be raised and handled simultaneously.
- `BaseExceptionGroup` — Like `ExceptionGroup` but can also contain `BaseException` instances (e.g., `KeyboardInterrupt`).

**Warnings**
- `UserWarning` — Default category for warnings generated by user code via `warnings.warn()`.
- `DeprecationWarning` — Warns about deprecated features aimed at developers.
- `PendingDeprecationWarning` — Warns about features that will be deprecated in the future.
- `RuntimeWarning` — Warns about dubious runtime behavior.
- `SyntaxWarning` — Warns about dubious syntax that is still valid Python.
- `ResourceWarning` — Warns about resource usage issues, such as unclosed files.
- `FutureWarning` — Warns end users about deprecated features (higher visibility than `DeprecationWarning`).
- `ImportWarning` — Warns about likely mistakes in module imports.
- `UnicodeWarning` — Warns about Unicode-related issues.
- `BytesWarning` — Warns when `bytes` or `bytearray` objects are compared to strings.

### Package-Specific Exception Types

Many packages such as `pandas`, `scikit-learn`, and `boto3` have their own specific exception classes. These should be utilized whenever possible,
since they provide the most detailed information about an error and how to remedy it.

Some examples:

- [`pandas` exceptions & warnings](https://pandas.pydata.org/docs/reference/testing.html#exceptions-and-warnings)
- [`scikit-learn` exception classes](https://scikit-learn.org/stable/api/sklearn.exceptions.html)
- [`boto3` exception classes](https://docs.aws.amazon.com/boto3/latest/guide/error-handling.html)

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

### Log Levels
Decide the level of events you want to capture. The level will be flagged in
each line of your log, which is useful for future filtering and sorting.
The developer can choose what level to use with each actual `logging` call:

- Debug
- Info
- Warning
- Error
- Critical

### Filename
This is the full path to your error log file.

### Filemode
The mode in which your `filename` is opened. The default is `a` for append.

### Format
The format of the log message. This is a string that can contain select output from your code.

```python
import logging

logging.basicConfig(filename='/var/log/app-error.log', filemode='a', format='%(asctime)s - %(levelname)s - %(message)s')
logging.warning("This is a warning message in the log file")
logging.error("LOOK OUT! This is an error!!")
```

The log results from above:
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

### Stack Traces

The `logging` module also allows you to capture stack traces into the log by passing the `exc_info` parameter
as `True`:

```python
import logging

x = 23
y = 0

try:
  z = x / y
except Exception as e:
  logging.error("EXCEPTION", exc_info=True)

```

Which results in the following log message:
```
ERROR:root:EXCEPTION
Traceback (most recent call last):
  File "logging-demo.py", line 7, in <module>
    z = x / y
ZeroDivisionError: division by zero
[Finished in 0.1s]
```

## Read More

- [Error Handling with boto3 and AWS](aws-example.py)


- [Python Errors HOWTO](https://docs.python.org/3/tutorial/errors.html)
- [Python Logging HOWTO](https://docs.python.org/3/howto/logging.html)
