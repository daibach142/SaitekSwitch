# INSTALLATION

## General

1. Download and expand the software from [here](https://github.com/daibach142/SaitekSwitch). 

### For Windows
 
1. Copy `saitekswitch.exe` to a suitable location (Desktop?).
2. Copy `switchdefaultconfig.xml` to the same directory.
3. Copy `saitekswitch.nas` to your local FlightGear directory `<username>\AppData\Roaming\flightgear.org\Nasal`.
4. Copy `saitekswitch.xml` to  FGFS Protocol directory `Program Files\Flightgear 2020.3\data\Protocol`.
5. First run (in Powershell) `.\saitekswitch.exe ` & check for any error messages.
 
## Linux

1. Run `sudo make install` in a terminal
2. Copy `*.xml` to a convenient local directory.
3. From a directory where `switchdefaultconfg.xml` is located,  run `saitekswitch`  & check for any error messages.

## Note

The default configuration file is `switchdefaultconfig.xml` and simply replace its contents with the desired configuration file (`cessna.xml`, `piper.xml`, or your own). This allows the program to be run without any arguments if desired; it is still possible to specify a config file as an argument if desired.

