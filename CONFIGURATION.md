# CONFIGURATION

Copy maybe the data/cessna.xml file as a base.

## SWITCH elements

There are 13 **`switch`** elements, edit each one to configure the Panel switch to the aircraft configuraation, possibly using the 'properties' window in the simulator. A switch toggles the specified property by sending 1 (on) or 0 (off); this works for FGFS bool, integral or double-precision values. Note that the current switch state is maintained within this driver software, and is not read from the simulator. It is **`MANDATORY`** to provide all 13 elements.

## MAGNETO element

There is one of this, corresponding to the magneto value (0..5) in the simulator. The Magneto Start position also triggers the **`starter`** element on (or off when engaged and moving from the Magneto Start position).

## STARTER element

There is one **`starter`** element, which specifies the control to be activated when the Magneto Start position is reached.


## SWITCH special

In the Piper configuration file, the TAXI switch is unused, and the normal control string is replaced by `autopilot engage`. This special case is recognised by the `saitekswitch.nas` file, and operates the autopilot engage/disengage for KAP140 on PA28-116. 
