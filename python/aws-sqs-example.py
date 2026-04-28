# For a specific service it might look something like this, with
# output that parses the error response and prints them.
# Logging is also incorporated here.

import botocore
import boto3
import logging

logging.basicConfig(
    filename='aws-sqs-example.log',
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
)
logger = logging.getLogger(__name__)


def send_sqs_message():
    logger.info('Starting send_sqs_message')

    logger.info('Creating SQS client')
    client = boto3.client('sqs')
    queue_url = 'SQS_QUEUE_URL'
    logger.info('Target queue URL: %s', queue_url)

    try:
        logger.info('Sending message to queue')
        response = client.send_message(QueueUrl=queue_url, MessageBody=('some_message'))
        logger.info('Message sent successfully. MessageId: %s', response.get('MessageId'))

    except botocore.exceptions.ClientError as err:
        if err.response['Error']['Code'] == 'InternalError': # Generic error
            # We grab the message, request ID, and HTTP code to give to customer support
            logger.error('Error Message: %s', err.response['Error']['Message'])
            logger.error('Request ID: %s', err.response['ResponseMetadata']['RequestId'])
            logger.error('Http code: %s', err.response['ResponseMetadata']['HTTPStatusCode'])
        else:
            logger.exception('Unhandled ClientError while sending SQS message')
            raise err


if __name__ == '__main__':
    send_sqs_message()
