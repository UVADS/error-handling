# The Python interface to the AWS cloud is called boto3
# https://boto3.amazonaws.com/v1/documentation/api/latest/index.html
# 
# Boto has an error handling package and bundle of classes.
# 
# Here is the typical format for exception handling in boto3
# using the try/except syntax:


import botocore
import boto3

client = boto3.client('aws_service_name')

try:
    client.some_api_call(SomeParam='some_param')

except botocore.exceptions.ClientError as error:
    # Put your error handling logic here
    raise error

except botocore.exceptions.ParamValidationError as error:
    raise ValueError('The parameters you provided are incorrect: {}'.format(error))


# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # 

# For a specific service it might look something like this, with
# output that parses the error response and prints them. Note the
# print lines are pulling out specific JSON values:


import botocore
import boto3

client = boto3.client('sqs')
queue_url = 'SQS_QUEUE_URL'

try:
    client.send_message(QueueUrl=queue_url, MessageBody=('some_message'))

except botocore.exceptions.ClientError as err:
    if err.response['Error']['Code'] == 'InternalError': # Generic error
        # We grab the message, request ID, and HTTP code to give to customer support
        print('Error Message: {}'.format(err.response['Error']['Message']))
        print('Request ID: {}'.format(err.response['ResponseMetadata']['RequestId']))
        print('Http code: {}'.format(err.response['ResponseMetadata']['HTTPStatusCode']))
    else:
        raise err
