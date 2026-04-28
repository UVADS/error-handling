# For a specific service it might look something like this, with
# output that parses the error response and prints them. Note the
# print lines are pulling out specific JSON values:

import botocore
import boto3


def send_sqs_message():
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


if __name__ == '__main__':
    send_sqs_message()
