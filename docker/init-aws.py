import logging
import os
import threading
import boto3  # Use the standard boto3 client

# Configure logging
logging.basicConfig(level=logging.DEBUG)

def create_user_and_access_key(user_name, access_key_id):
    iam_client = boto3.client(
        "iam",
        aws_access_key_id=access_key_id,
        aws_secret_access_key="test",  # Dummy key for LocalStack
        region_name="us-east-1",
        endpoint_url="http://localhost:4566"  # LocalStack endpoint
    )
    iam_client.create_user(UserName=user_name)
    response = iam_client.create_access_key(UserName=user_name)
    return response['AccessKey']


def create_ssm_parameters(access_key_id, secret_access_key, prefix, count):
    threads = []
    ssm_client = boto3.client(
        "ssm",
        aws_access_key_id=access_key_id,
        aws_secret_access_key=secret_access_key,
        region_name="us-east-1",
        endpoint_url="http://localhost:4566",
    )

    def put_parameter(name, value):
        ssm_client.put_parameter(
            Name=name,
            Value=value,
            Type="String"
        )

    for i in range(count):
        param_name = f"{prefix}/param{i}"
        thread = threading.Thread(target=put_parameter, args=(param_name, f"value{i}"))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

logging.info("Starting LocalStack setup...")

access_key_dev = create_user_and_access_key('devUser', '000000000001')
access_key_qa = create_user_and_access_key('qaUser', '000000000002')  # Corrected line

logging.info(f"Dev access key: {access_key_dev}")
logging.info(f"Qa access key: {access_key_qa}")

create_ssm_parameters(access_key_dev['AccessKeyId'], access_key_dev['SecretAccessKey'], '/config/dev', 20)
create_ssm_parameters(access_key_qa['AccessKeyId'], access_key_qa['SecretAccessKey'], '/config/qa', 20)

logging.info("...finished LocalStack setup")