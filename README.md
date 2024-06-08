
# TCP protocol developed in rust (Work in Progress)

I*IDEA* - using TUN functionality of Linux, create a virtual NIC to send/receive our own TCP packets without kernel packing them into original TCP (thus our own tcp protocol)

## Setup

Just run 'run.sh' to run the code and setup the NIC

In separate terminal run:
'''
ping -I tun0 192.168.0.2 or nc 192.168.0.2 80
'''