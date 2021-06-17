# example-actix-web-on-app-runner

This repository is example web app powered by Actix Web and running on AWS App Runner.

## Build image

```bash
$ docker build -t example-actix-web .
```

(Run for test in local)

```bash
$ docker run -it -p 8080:8080 -e AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY -e AWS_SESSION_TOKEN example-actix-web:latest
$ curl http://localhost:8080/
```

## Push image to ECR Public

```bash
$ aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin public.ecr.aws/y4f6t3p4
$ docker tag example-actix-web:latest public.ecr.aws/y4f6t3p4/example-actix-web:latest
$ docker push public.ecr.aws/y4f6t3p4/example-actix-web:latest
```
