# HDD Relay Switcher

To enable and disable my harddrives, I have installed a relay in my PC to disconnect and connect their power
programmatically.

The relay I am using is this: https://www.amazon.de/dp/B0B9LM5B33?ref_=ppx_hzsearch_conn_dt_b_fed_asin_title_16

Unfortunately, this only comes with some chinese debugging software to test the individual channels.
You can also talk to the relay via HID.
SO I created a rust program that will switch all channels on or off.

The icon I use for the program is taken from https://github.com/free-icons/free-icons and is licensed under the Creative Commons Attribution 4.0 International (CC BY 4.0) license.
Additionally, I modified it to include a background color to indicate on (green) or off (red).

The whole program is licensed under the GPL3.

To build the programs, go into the on and off folder and call
    cargo build --release
This is mainly due to the build.rs setting hte icon and I have yet to figure out how to do this within a project instead of 
two even if they share 100% of the code basically.