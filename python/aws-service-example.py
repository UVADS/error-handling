"""Generic boto3 error-handling skeleton.

The Python interface to AWS is `boto3`; its low-level error classes live in
`botocore.exceptions`. The two exception types most code paths need are:

  - ClientError           Raised when AWS returns an error response
                          (e.g. AccessDenied, ThrottlingException).
  - ParamValidationError  Raised client-side, before the request is sent,
                          when the arguments don't match the API schema.

This file shows the minimum scaffold. See `aws-sqs-example.py` for a concrete
example with response inspection and logging.

Reference: https://boto3.amazonaws.com/v1/documentation/api/latest/guide/error-handling.html
"""

import logging

import boto3
import botocore

logging.basicConfig(
    filename='aws-service-example.log',
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
)
logger = logging.getLogger(__name__)


def generic_api_call():
    # 'aws_service_name' is a placeholder — replace with a real service id
    # such as 's3', 'sqs', 'dynamodb', etc.
    client = boto3.client('aws_service_name')

    try:
        client.some_api_call(SomeParam='some_param')

    # Order matters: list more specific exceptions first. ParamValidationError
    # is raised client-side before the request goes out, so catching it
    # separately lets us turn an SDK-level failure into a clearer ValueError
    # for the caller.
    except botocore.exceptions.ParamValidationError as error:
        # `raise ... from error` preserves the original cause in the traceback,
        # which is the idiomatic way to translate one exception into another.
        raise ValueError(f'Invalid parameters for some_api_call: {error}') from error

    except botocore.exceptions.ClientError as error:
        # Log with traceback, then re-raise. A bare `raise` (no argument)
        # preserves the original traceback — preferred over `raise error`.
        logger.exception('AWS call failed: %s', error)
        raise


if __name__ == '__main__':
    generic_api_call()
