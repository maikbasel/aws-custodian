#!/bin/bash

AWS_ACCESS_KEY_ID=000000000000 awslocal iam create-user --user-name devUser
AWS_ACCESS_KEY_ID=000000000000 awslocal iam create-access-key --user-name devUser

AWS_ACCESS_KEY_ID=000000000001 awslocal iam create-user --user-name qaUser
AWS_ACCESS_KEY_ID=000000000001 awslocal iam create-access-key --user-name qaUser

AWS_ACCESS_KEY_ID=000000000000 awslocal ssm put-parameter --name "/config/dev/foo" --value "bar" --type "String"
AWS_ACCESS_KEY_ID=000000000000 awslocal ssm put-parameter --name "/config/dev/baz" --value "qux" --type "String"

AWS_ACCESS_KEY_ID=000000000001 awslocal ssm put-parameter --name "/config/qa/foo" --value "bar" --type "String"
AWS_ACCESS_KEY_ID=000000000001 awslocal ssm put-parameter --name "/config/qa/baz" --value "qux" --type "String"