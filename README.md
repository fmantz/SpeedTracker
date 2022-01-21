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

2. start "speedtestJson" and append its output at the file of the current month in 'data_dir'.
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


## Required Software:

 - package wireless-tools.dep  (iwgetid)  (for local mode and WLAN connection, required if the WLAN ID should be in the output)
 - requirements from SpeedTest
 - any webserver to 

## Install on a Raspberry Pi

0. Ensure that you cloned the repo including submodules
1. install rust & compile speedtracker 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
```
2. install speedtracker [./SpeedTest/README.md](./SpeedTest/README.md)
3. copy "target/release/speedtracker" and "SpeedTest/speedtestJson" into one new directory e.g. "/opt/speedtracker"
4. copy "pi_files" into "/opt/speedtracker"
5. create a cronjob for speedtracker via 'crontab -e' e.g.:
```bash
#run every two hours
0 */2 * * * /root/speedtracker run
```
7. install a webserver e.g. apache
```bash
sudo apt-get install apache
```
8. modify /speedtracker.toml, interesting settings are:
```bash
data_dir = './'  <- your data files are stored here, best practice not on the sdcard, on a usb thumb drive e.g 
output_file = '/var/www/html/index.html',  <- your output file must be served by the webserver, so pick a directory that is served
output_xdays = 14   <- numbers of days in the past you are intersted in (can be changed anytime, no data is deleted)
log_file = './speedtracker.log'  <- location where your log file is stored

[download_chart.expected_value]
value = 250.0   <- your expected download speed, it is in Mbits/s

[upload_chart.expected_value]
value = 25.0    <- your expected upload speed, it is in Mbits/s
```
9. ensure that all location you specified above are writable
10. enjoy and wait for your collected data

## Install with Docker (without github):

If you do not want to run SpeedTracker on a Raspberry Pi but on a NAS, you can use docker.
Interessting settings you might want to change are in directory  [./docker_files](./docker_files).
Files speedtracker.toml and cron_file.txt you might want to change. (see Install on a Raspberry Pi) 

### Build:

```bash
docker build . --tag speedtracker:0.1.0 
```

### Run:

```bash
docker run -dit --name mySpeedTracker -p 8080:80 speedtracker:0.1.0
```

## Install with Docker (with github):

```bash
docker run -dit --name mySpeedTracker -p 8080:80 speedtracker:0.1.0
```

