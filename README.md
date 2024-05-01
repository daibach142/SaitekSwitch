# FGFS DRIVER FOR SAITEK SWITCH PANEL

This project provides an interface between Flightgear Flight Simulator and the
Saitek Switch Panel. Operation of switch(es) on the Panel is reflected in the simulator. 

Executable images are provided for Linux (x86-64) and Windows. The code base is identical
for Linux and Windows and is written in Rust.

If you would like to improve the code or automate the Windows installation, please issue a 'pull' request. 

This software loads configuration which maps the switch keys to the **FSFG** simulator properties. The currently available configuration files are located in `cessna.xml`, `cessna182s.xml` and `piper.xml`
and are for:

* Cessna 172
* Cessna 182
* Piper PA28-116

It's quite easy to roll-your-own using the Cessna as an example.

This project is the successor to daibach142/FGFS_Saitek_Switch_Panel on GitHub.

This version (1.3.1) revises the license to MIT, and reconfigures the code base (no interface changes)

For problems or issues either enter an issue on Github, or email stksp@attwoods.org.uk.

With many thanks to Bruce Maggs (bmm@cs.duke.edu) for help with debugging.

---

## INSTALLATION

### General

1. Download and expand the software from [here](https://github.com/daibach142/SaitekSwitch). 

### For Windows
 
1. Copy `saitekswitch.exe` to a suitable location (Desktop?).
2. Copy `switchdefaultconfig.xml` to the same directory.
3. Copy `saitekswitch.nas` to your local FlightGear directory `<username>\AppData\Roaming\flightgear.org\Nasal`.
4. Copy `saitekswitch.xml` to  FGFS Protocol directory `Program Files\Flightgear 2020.3\data\Protocol`.
5. First run (in Powershell) `.\saitekswitch.exe ` & check for any error messages.
6. Copy `startup.bat` to same location as the program (Desktop?).
7. If the program `saitekradio.exe` is not present, delete the relevant line in `startup.bat`.
8. Program(s) can now be run by double-click on `startup.bat`.
 
 ---
 
### Linux

1. Run `sudo make install` in a terminal
2. Copy `*.xml` to a convenient local directory.
3. From a directory where `switchdefaultconfg.xml` is located,  run `saitekswitch`  & check for any error messages.

---

### Note

The default configuration file is `switchdefaultconfig.xml` and simply replace its contents with the desired configuration file (`cessna.xml`, `piper.xml`, or your own). This allows the program to be run without any arguments if desired; it is still possible to specify a config file as an argument if desired.

---

## RUNNING

Note that the Switch Panel software is configured at startup for a
specific aircraft using a configuration file. Files for the Cessna 172P and Piper 28-116 are currently provided, see `CONFIGURATION` for roll-your-own.

Add the following start option to FlightGear configuration:

	...
	--generic=socket,in,20,,60000,udp,saitekswitch
	...

Start the simulator first, and wait until initialisation complete.
Set the switches suitably on the panel.  
  
---
  
### On Windows
To use the default configuration file `switchdefaultconfig.xml`,  
double-click the `startup.bat` file on the Desktop.  

---

### On Linux
Enter the command `saitekswitch` or run from the file manager, to use the default file `switchdefaultconfig.xml`

---

The software will illuminate the forward landing gear light RED,
awaiting a switch change. Operate any switch, and the complete switch
settings on the panel will be transferred to the simulator.

---

### NOTES

On the Piper, the TAXI switch is repurposed to operate the KAP140 Autopilot 
ACTIVE/STANDBY switch.

On Linux, the default configuration file should be in the directory where you launch the `saitekswitch` program.

An optional argument may be used to change to a specific confiuration file e.g.  

`saitekswitch piper.xml`

---

## CONFIGURATION (ROLL-YOUR-OWN PANEL)

If you wish to interface to a new aircraft, these instructions may help!


Copy maybe the `cessna.xml` file as a base.


### SWITCH elements

There are 13 **`switch`** elements, edit each one to configure the Panel switch to the aircraft configuraation, possibly using the 'properties' window in the simulator. A switch toggles the specified property by sending 1 (on) or 0 (off); this works for FGFS bool, integral or double-precision values. Note that the current switch state is maintained within this driver software, and is not read from the simulator. It is **`MANDATORY`** to provide all 13 elements.

---

### MAGNETO element

There is one of this, corresponding to the magneto value (0..5) in the simulator. The Magneto Start position also triggers the **`starter`** element on (or off when engaged and moving from the Magneto Start position).

---

### STARTER element

There is one **`starter`** element, which specifies the control to be activated when the Magneto Start position is reached.

---

### SWITCH special


In the Piper configuration file, the TAXI switch is unused, and the normal control string is replaced by `autopilot engage`. This special case is recognised by the `saitekswitch.nas` file, and operates the autopilot engage/disengage for KAP140 on PA28-11x. 

Also for any aircraft with a three-way NAV1/NAV2/GPS such the KAP140 on the PA28-11x, using the switch value  
`instrumentation/nav-source/selector` will cause the switch to select either the NAV1 or GPS settings.



