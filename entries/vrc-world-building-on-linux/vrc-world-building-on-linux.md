+++
entry_name = "vrc-world-building-on-linux"
title = "VRC World Building on Linux"
date = "28/07/2025"
tags = "vrc,worldbuilding,linux,unity"
+++

Setting up VRC World Building environment on Linux.

Install UnityHub using the regular method from the Unity site.

At the time of writing this, VCC is not available for Linux, so you have to use the CLI which is vpm. vpm can be installed using dotnet tool install. See VRC website for install details. Because there's no VCC, all operations that would otherwise be done using VCC GUI, have to be done using the vpm CLI instead.

At this point go and locate the 'settings.json' file. It should be in "~/.local/share/VRChatCreatorCompanion".

Next use "vpm check hub" to make sure vpm can find the hub. If it can't you'll have to go into the settings file and set the path for the hub manually. Use "which unityhub" to find the location. At the time of writing this, setting the path for the Unity editor in the settings file is pointless because whenever you run vpm it overwrites the file.

Next you wanna install the required Unity editor version. There is a "vpm install unity" command but I've found that it doesn't work on my end. So I checked the current VRChat Unity version on the VRC website, and went to the Unity site to grab that exact version. There's an Install option that opens it up within your UnityHub. Use that. Once the install option opens up in Unity, make sure to also check the "Android build support" option, along with the NDK and JDK options within it. Then start the install. 

Once the install completes, next you need to install the templates. Use "vpm install templates". You can then use "vpm list templates" to see which templates are installed.

After that you can create your project using vpm. This part can seem a bit confusing. You use "vpm new <project name> <template> [-p path]". So to create a project called FirstVRCproj01 at path /home/demkeys/UntyProjects/ you would type:
vpm new FirstVRCProj01 "World" -p /home/demkeys/UntyProjects

This will create a directory with the project name, and also create the project. The confusing part is that the command will still say that it can't find a Unity editor, despite you having installed it. Atleast that's what kept happening for me. Fortunately it goes ahead and creates the project anyways.

Once the project is created, go into your UnityHub and add the project to the list. You can then open the project from the UnityHub, and it should open up in the Unity editor.

This is what worked for me after a bunch of trial and error.
