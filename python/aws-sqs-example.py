"""Concrete boto3 error-handling example: sending an SQS message.

Demonstrates the patterns introduced in `aws-service-example.py` against a
real AWS service, including:

  - Two-tier exception handling (BotoCoreError vs ClientError).
  - Pulling structured fields out of a ClientError response (Code, Message,
    RequestId, HTTPStatusCode) for incident reports / customer support.
  - Logging at appropriate levels (INFO for the operation, ERROR with
    traceback on failure).
"""

import logging
import sys

import boto3
import botocore

logging.basicConfig(
    filename='aws-sqs-example.log',
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
)
logger = logging.getLogger(__name__)

# Replace this placeholder with your queue URL before running.
QUEUE_URL = 'SQS_QUEUE_URL'
MESSAGE_BODY = 'some_message'


def send_sqs_message(queue_url: str = QUEUE_URL, body: str = MESSAGE_BODY) -> str | None:
    """Send `body` to `queue_url`. Returns the SQS MessageId on success."""
    client = boto3.client('sqs')

    try:
        response = client.send_message(QueueUrl=queue_url, MessageBody=body)
        message_id = response['MessageId']
        logger.info('Sent message to %s (MessageId=%s)', queue_url, message_id)
        return message_id

    except botocore.exceptions.ClientError as err:
        # ClientError is raised when AWS returns an error *response*. The
        # response dict carries the fields you'd want in a bug report.
        error = err.response['Error']
        meta = err.response['ResponseMetadata']
        logger.error(
            'SQS ClientError: code=%s message=%s request_id=%s http_status=%s',
            error.get('Code'),
            error.get('Message'),
            meta.get('RequestId'),
            meta.get('HTTPStatusCode'),
        )
        # Re-raise so the caller (or the process) sees the failure. Silently
        # swallowing errors after logging is a common anti-pattern — don't.
        raise

    except botocore.exceptions.BotoCoreError:
        # BotoCoreError covers SDK-side problems that never reached AWS:
        # endpoint resolution, credential lookup, connection timeouts, etc.
        # `logger.exception` automatically includes the traceback.
        logger.exception('SQS call failed before reaching AWS')
        raise


if __name__ == '__main__':
    if QUEUE_URL == 'SQS_QUEUE_URL':
        sys.exit('Set QUEUE_URL to a real SQS queue URL before running this example.')
    send_sqs_message()
