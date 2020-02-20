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

- `rust-autograder`: https://hub.docker.com/repository/docker/
rooks/rust-autograder/
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
```bash
git clone --recurse-submodules git@github.com:ethanabrooks/autograder.git # clone the sample-rust-submission submodule as well
cd autograder
docker build -f Dockerfile.debug -t ethanabrooks/debug-rust-autograder .
docker push ethanabrooks/debug-rust-autograder # unnecessary unless you changed the build somehow
cd submission
zip -r submission.zip assignment Cargo.* src/ -x '*/\target/*' # create a zip file of the submission, ignoring target/ directories
```
Now 
1. Go to https://www.gradescope.com/courses/78826/assignments/355815/configure_autograder.
2. Check the "Manual Docker Configuration" checkbox.
3. Write `ethanabrooks/debug-rust-autograder` in the **DOCKERHUB IMAGE NAME** field.
4. Click "Test Autograder" (lower right of the screen)
5. Upload the zip file we made earlier (`autograder/submission/submission.zip`).
6. (Optional) Click the "❯_ Debug via SSH". After a minute or two, Gradescope will spit out an ssh command -- something like
```
ssh root@ec2-34-216-119-27.us-west-2.compute.amazonaws.com -p 32790
```
If you run this on your terminal, you can poke around in the container that Gradescope is using to run the autograder. To reproduce what Gradescope actually does, run `./run_autograder`. This should produce a file at `/autograder/results/results.json`. The content of this file should match the output format specified here: https://gradescope-autograders.readthedocs.io/en/latest/specs/#output-format. 
