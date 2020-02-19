# autograder

This repository implements an autograder for Rust, intended to interface with the Gradescope autograding infrastructure.
Specifically, it contains Dockerfiles that set up the development environment necessary for running rust code
and a file `run_autograder`.
Upon receiving a submission, Gradescope

1. creates a docker container
2. copies the student's submission into the container
3. executes `./run_autograder`.

Gradescope acquires the image for the container from dockerhub. In the image it
runs some unspecified proprietary Gradescope magic code that copies the
submission and messes with the environment variables 😩

That said, there are two images that I have been using:

- `rust-autograder`: https://hub.docker.com/repository/docker/ethanabrooks/rust-autograder/
- `debug-rust-autograder`: https://hub.docker.com/repository/docker/ethanabrooks/debug-rust-autograder/

These correspond to the two Dockerfiles, `Dockerfile`, and `Dockerfile.debug`.

The first is distinguished by the fact that it does not copy the autograding/assignment logic into the Docker image.
The reason for this is to allow us to make changes to the autograding/assignment logic and submit them with our
sample assignment (https://github.com/ethanabrooks/sample-rust-submission)
instead of building a new image and pushing to dockerhub every time.

Obviously the student's submission will not include out autograding/assignment logic.
Therefore `Dockerfile`/`rust-autograder` copies the logic to the image before pushing to dockerhub.

The intended development workflow is to do all the work with `Dockerfile.debug` and `debug-rust-autograder`
and only once development is finished to build the final image with `Dockerfile` and push to `rust-autograder`.

# Instructions for actually running the code:
