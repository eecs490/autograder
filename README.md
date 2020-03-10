# autograder

This repository implements an autograder for Rust, intended to interface with the Gradescope autograding infrastructure.
Specifically, it contains a Dockerfile that set up the development environment necessary for running rust code
and a file `run_autograder`.
Upon receiving a submission, Gradescope

1. creates a docker container
2. copies the student's submission into the container
3. executes `./run_autograder`.

Gradescope acquires the image for the container from dockerhub. In the image it
runs some unspecified proprietary Gradescope magic code that copies the
submission and messes with the environment variables 😩

# Getting started

[Install rust.](https://www.rust-lang.org/tools/install)

Clone the repository:

```bash
git clone git@github.com:ethanabrooks/autograder.git
cd autograder
```

## Building and running the docker image

```bash
docker build -t ethanabrooks/rust-autograder .
docker run --rm -it --name running-autograder ethanabrooks/rust-autograder ./run_autograder
```

## Updating the dockerhub image

Note that in order to update the dockerhub image, you will need a dockerhub
account, which you can register for [here](https://hub.docker.com/signup).
I will also need to add you as a collaborator on the project.

```bash
docker push ethanabrooks/rust-autograder
```

## Submitting to Gradescope

The first step is to create an assignment zip file.
From the root of the project:

```bash
cd submission/
zip -r submission.zip **/Cargo.* **/src/ -x '*/\target/*'
```

### Submitting to Gradescope

1. Go to https://www.gradescope.com/courses/78826/assignments/355815/configure_autograder.
2. Check the "Manual Docker Configuration" checkbox.
3. In the **DOCKERHUB IMAGE NAME** field, write `ethanabrooks/rust-autograder`.
4. Click "Update Autograder" (lower right of the screen)
5. Click "Test Autograder" (lower right of the screen)
6. Upload the zip file we made earlier `submission/submission.zip`.
7. (Optional) Click the "❯\_ Debug via SSH". After a minute or two, Gradescope will spit out an ssh command -- something like

```
ssh root@ec2-34-216-119-27.us-west-2.compute.amazonaws.com -p 32790
```

If you run this on your terminal, you can poke around in the container that Gradescope is using to run the autograder. To reproduce what Gradescope actually does, run `./run_autograder`. This should produce a file at `/autograder/results/results.json`. The content of this file should match the output format specified [here](https://gradescope-autograders.readthedocs.io/en/latest/specs/#output-format).

# Guide to the autograder

The "top level" script for the autograder is `run_autograder`. This bash script has
three steps:

1. It runs the student's tests on the student's solution and measures code
   coverage.
2. It runs our tests on the student's solution and measures correctness.
3. It rune the student's tests on our solution and measures the correctness of
   the student's tests.
4. It runs [`assignment/src/main.rs`](https://github.com/ethanabrooks/autograder/blob/master/assignment/src/main.rs) to process these results into a JSON file formatted for Gradescope (per <https://gradescope-autograders.readthedocs.io/en/latest/specs/#output-format>),

For more details on how Gradescope works, look at the [Gradescope autograding docs](https://gradescope-autograders.readthedocs.io/en/latest/).
