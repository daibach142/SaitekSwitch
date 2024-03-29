# RUNNING

Note that the Switch Panel software is configured at startup for a
specific aircraft using a configuration file. Files for the Cessna 172P and Piper 28-116 are currently provided, see `CONFIGURATION` for roll-your-own.

Add the following start option to FlightGear configuration:

	...
	--generic=socket,in,20,,60000,udp,saitekswitch
	...

Start the simulator first, and wait until initialisation complete.
Set the switches suitably on the panel.  
  
## On Windows
To use the default configuration file `switchdefaultconfig.xml`,  
double-click the `startup.bat` file on the Desktop.  

## On Linux
Enter the command `saitekswitch` or run from the file manager, to use the default file `switchdefaultconfig.xml`

The software will illuminate the forward landing gear light RED,
awaiting a switch change. Operate any switch, and the complete switch
settings on the panel will be transferred to the simulator.

### NOTES

On the Piper, the TAXI switch is repurposed to operate the KAP140 Autopilot 
ACTIVE/STANDBY switch.

On Linux, the default configuration file should be in the directory where you launch the `saitekswitch` program.

An optional argument may be used to change to a specific confiuration file e.g.  

`saitekswitch piper.xml`

