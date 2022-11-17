export PROJECT=.
export BUILD_OUTPUT=./output-reproducible
export IMAGE=elrondnetwork/build-contract-rust:v3.2.2

python3 ./build_with_docker.py --image=${IMAGE} \
    --project=${PROJECT} \
    --output=${BUILD_OUTPUT}