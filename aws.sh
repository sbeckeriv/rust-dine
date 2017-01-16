#!/bin/bash
yum update -y
cd /home/ec2-user/rust-dine
git pull
/root/.cargo/bin/cargo build --release
service rust-dine start




