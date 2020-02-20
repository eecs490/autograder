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
- `rust-autograder-dev`: https://hub.docker.com/repository/docker/ethanabrooks/dev-rust-autograder/

These correspond to the two Dockerfiles, `Dockerfile`, and `Dockerfile.dev`.

The first is distinguished by the fact that it does not copy the autograding/assignment logic into the Docker image.
The reason for this is to allow us to make changes to the autograding/assignment logic and submit them with our
sample assignment (https://github.com/ethanabrooks/sample-rust-submission)
instead of building a new image and pushing to dockerhub every time.

Obviously the student's submission will not include out autograding/assignment logic.
Therefore `Dockerfile`/`rust-autograder` copies the logic to the image before pushing to dockerhub.

The intended development workflow is to do all the work with `Dockerfile.dev` and `rust-autograder-dev`
and only once development is finished to build the final image with `Dockerfile` and push to `rust-autograder`.

# Getting started

```bash
git clone git@github.com:ethanabrooks/autograder.git
cd autograder
```

## Building the docker image

### dev version

```bash
docker build -f Dockerfile.dev -t ethanabrooks/rust-autograder-dev .
```

### release version

```bash
docker build -t ethanabrooks/rust-autograder .
```

## Running the docker image locally

### dev version

```bash
docker run --rm -it --name running-autograder ethanabrooks/rust-autograder-dev /bin/bash
```

From outside the image, run

```bash
docker cp ~/autograder/autograder running-autograder:/autograder/submission
```

This simulates the copying of the submission into the docker image.
From inside the image, run

```bash
./run_autograder
```

### release version

```bash
docker run --rm -it --name running-autograder ethanabrooks/rust-autograder /bin/bash
```

From outside the image, run

```bash
docker cp ~/autograder/autograder/submission running-autograder:/autograder/submission
```

This simulates the copying of the submission into the docker image.
From inside the image, run

```bash
./run_autograder
```

## Updating the dockerhub image

Note that in order to update the dockerhub image, you will need a dockerhub
account, which you can register for here: https://hub.docker.com/signup

### dev version

```bash
docker push ethanabrooks/rust-autograder-dev
```

### release version

```bash
docker push ethanabrooks/rust-autograder
```

## Submitting to Gradescope

The first step is to create an assignment zip file. The dev version allows you
to submit the grading/assignment logic with the submission, allow the developer
to make changes without needing to update the dockerhub image.

### dev version

For this version, we zip all three directories. From the root of the project:

```bash
cd autograder
zip -r submission.zip **/Cargo.* **/src/ -x '*/\target/*'
```

### release version

For this version, we only zip the `submission/` directory. From the root of the
project:

```bash
cd autograder/submission/
zip -r submission.zip **/Cargo.* **/src/ -x '*/\target/*'
```

### Submitting to Gradescope

1. Go to https://www.gradescope.com/courses/78826/assignments/355815/configure_autograder.
2. Check the "Manual Docker Configuration" checkbox.
3. In the **DOCKERHUB IMAGE NAME** field, write
   - `ethanabrooks/rust-autograder-dev` for the dev version.
   - `ethanabrooks/rust-autograder` for the release version.
4. Click "Update Autograder" (lower right of the screen)
5. Click "Test Autograder" (lower right of the screen)
6. Upload the zip file we made earlier
   - `autograder/submission.zip` for dev.
   - `autograder/submission/submission.zip` for release.
7. (Optional) Click the "❯\_ Debug via SSH". After a minute or two, Gradescope will spit out an ssh command -- something like

```
ssh root@ec2-34-216-119-27.us-west-2.compute.amazonaws.com -p 32790
```

If you run this on your terminal, you can poke around in the container that Gradescope is using to run the autograder. To reproduce what Gradescope actually does, run `./run_autograder`. This should produce a file at `/autograder/results/results.json`. The content of this file should match the output format specified here: https://gradescope-autograders.readthedocs.io/en/latest/specs/#output-format.

# Guide to the autograder

At a high level, the autograder

1. runs `cargo test` on the student's submission and on the `assignment` package.
2. scrapes the output and parses into `rust` `TestResult` structs.
3. creates a `TestReport` struct and writes the associated json object to the location where Gradescope looks for it.

For details on how Gradescope works with this, read https://github.com/ethanabrooks/autograder/blob/master/README.md.

`autograder/` contains three subdirectories:

```bash
.
├── assignment
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── submission
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── test_lib
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── lib.rs
        └── main.rs -> ../../assignment/src/main.rs
```

`test_lib/` contains code that should not change between assignmnets.
`assignment/` contains code that is specific to each assignment.
`submission/` simulates an example student submission.

To understand at a high level what the program is doing look at
`assignment/src/main.rs`.
