#!/bin/bash

for folder in `ls`; do
    cd $folder
    rm -rvf .git
    cd ../
done
