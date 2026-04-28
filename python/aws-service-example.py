# The Python interface to the AWS cloud is called boto3
# https://boto3.amazonaws.com/v1/documentation/api/latest/index.html
#
# Boto has an error handling package and bundle of classes.
#
# Here is the typical format for exception handling in boto3
# using the try/except syntax:

import botocore
import boto3

def generic_api_call():
    client = boto3.client('aws_service_name')

    try:
        client.some_api_call(SomeParam='some_param')

    except botocore.exceptions.ClientError as error:
        # Put your error handling logic here
        raise error

    except botocore.exceptions.ParamValidationError as error:
        raise ValueError('The parameters you provided are incorrect: {}'.format(error))


if __name__ == '__main__':
    generic_api_call()
