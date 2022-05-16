####################################################
#     FUNCTIONS in Saitek Switch Panel
####################################################

####################################################
# Issue the switch command
####################################################
var do_action = func {
    var thing = getprop("/saitek-switch-panel/switch");
    var action = getprop("/saitek-switch-panel/action");
    if (thing == "autopilot engage") {
       # looking for autopilot engage/disengage for KAP140 on PA28-116
       var state = getprop("autopilot/kap140/panel/state");
       if (state == 5 and action == 1) {
            # engaging - no sound
            setprop("autopilot/kap140/panel/state-old", 5);     
            setprop("autopilot/kap140/panel/state", 6);
       }
       else if (state == 6 and action == 0) {
            # disengaging - make sound
            setprop("autopilot/kap140/panel/ap-timer", 5);
            setprop("autopilot/kap140/panel/state", 5);
        }   
    } else if (thing =="instrumentation/nav-source/selector") {
    	if (action ==1) {
 		   	setprop(thing, 2);  # GPS select
    	}
    	else {
    		setprop(thing, 0);  # NAV1 select
    	}
    	setprop("autopilot/kap140/panel/nav-timer",getprop("sim/time/elapsed-sec"));
    	setprop("/sim/sounde/switch1", 1);
    } else if (thing == "controls/engines/engine/primer-lever") {
    		c172p.pumpPrimer();
    } else  setprop(thing, action);
}

setlistener ("/saitek-switch-panel/action", do_action);


