# example-actix-web-on-app-runner

This repository is example web app powered by Actix Web and running on AWS App Runner.

## Development

```bash
$ DOCKER_BUILDKIT=1 docker build -t example-actix-web:latest .
$ docker run -d -p 8080:8080 example-active-web:latest
```
