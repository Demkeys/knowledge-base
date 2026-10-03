+++
entry_name = "installing-i3-wm-on-ubuntu-studio"
title = "Installing i3 WM on Ubuntu Studio"
date = "01/09/2025"
tags = "futureref,linux,ubuntu,i3,wm,de"
+++

Installing i3 WM on Ubuntu Studio is pretty straight forward. After installing, you logout and then at the login screen you can choose between other DEs, i3, etc. You choose i3, then login, and it loads up the WM instead of the DE. Great, cool. Couple of things to keep in mind:
- There's a difference between the login screen you see at system startup, and the login screen you screen after logging out of Plasma. The login screen for Plasma doesn't give you this option. So you'll wanna make sure you logout, not lock. Sometimes even then you might not see the right login screen. A method that definitely works, is to reboot the machine. At startup you'll see the login screen that lets you choose a DE, WM, etc. by clicking some sort of button. And then you're good to go.
- If you do have other DEs installed, their power managers might cause freezing or hanging issues if they get triggered while i3 is active. This could lead to issue where the system is in sleep mode and the monitor is off, but you can't using the mouse or keyboard to turn it back on, and it's just stuck like that, so you have to reboot. Solution:
  - Disable the startup of the other power managers. Because I'm using a version of Ubuntu Studio that has been upgraded over the years, I have Xfce and Plasma. Each has their own power manager. Xfce uses xfce4-power-manager and Plasma uses org_kde_powerdevil or something along those lines. You'll wanna go into both Xfce and Plasma each and make sure that the power managers are disabled in the startup settings. In my case Plasma's startup settings didn't show anything (even though it does have it's own list of stuff probably), but Xfce showed a whole bunch of stuff, including xfce4-power-manager. So I disabled that. I discovered that org_kde_powerdevil only starts up if you start up a Plasma session. But xfce4-power-manager seems to start up even if you directly load into i3, for some reason. So I disabled both. After this, reboot, and load directly into i3. Once in, start up btop and look for the other two power manager processes. If they're not there, you should be good.
- After this you should be good on the power manager issues. Now it'll be i3 and X11 that lock the screen after a period of inactivity, so you need to deal with that as well. To deal with this use the following commands:
  - xset s 0
  - xset -dpms
 These commands will disable the timeout. These commands will change the settings for your current session, but they're not permanent. To make it permanent edit or create ~/.xprofile , and add these commands in it. This completely disable timeout due to inactivity. Another reason to disable this is because, due to some configuration issue, even having a Twitch stream running in firefox doesn't seem to count as activity, and eventually the timeout is triggered. So completely disabling it gets rid of this issue. But if you're in a situation where you do want the timeout to happening after some time of inactivity, you'll want to look into the configuration.











