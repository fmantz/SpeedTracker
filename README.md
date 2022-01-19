# SpeedTracker

purpose: track and visualize DSL speed over a longer period  (runnable on a Raspberry Pi)

The program 'speedTracker' will have two modes:

- a mode 1 for producing data periodically via a cron job and generating the output at a fixed location:

```bash
speedtracker run
```

It will do:

1. read config file having following information:
    - data_dir     : directory where data is collected
    - output_file  : should be on a path served by a webserver (apache e.g.)
    - output_xdays : number of days in the past (from today) the data should be visualized
    - log_file     : logfile name

2. start "speedtestJson" and append its output at the file of the current month in 'data_dir'.visualize
3. read and filter files(s) to get the data of the last 'output_xdays'.
4. transform into a self-containing html.page at location 'output_file'.


- a mode 2 to produce a self a self-containing html.page at a given location (without calling speedtestJson):

```bash
speedtracker 2022-01-01 2021-12-31 ./index.html
```

1. parse following parameter from command line by position (as in the example above):
   - output_file
   - from_date
   - to_date

2. read config file to get 'data_dir'.
3. read and filter files(s) to get the data of the dates 'from_date' - 'to_date'
4. transform into a self-containing html.page at location 'output_file' (given as command parameter).


#Required Software:

 - package wireless-tools.dep  (iwgetid)  (for local mode and WLAN connection, required if the WLAN ID should be in the output)
 - requirements from SpeedTest
 - any webserver to 
