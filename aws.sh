#!/bin/bash
yum update -y
cd /home/ec2-user/rust-dine
git pull
service rust-dine start




