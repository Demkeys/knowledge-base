+++
entry_name = "rtmp-server-with-srs"
title = "RTMP server with SRS"
date = "26/07/2025"
tags = "futureref,srs,rtmp,linux,obs"
+++

Setting up SRS server for RTMP streaming
---

Follow the regular setup process for SRS (which is probably building from source). After the setup cd into srs/trunk.

To start SRS server:
objs/srs -c conf/rtmp.conf

Using the srs binary in objs allows you to specify a conf file. The example code in the docs mentions using the 'srs.conf' file. If you only need an RTMP server then don't do that. There's a bunch of extra settings in that file that cause the RTMP server to not work properly. Instead, use the 'rtmp.conf' file. This file only contains settings pertaining to RTMP, nothing more.

Once the server is running, to query status or stop the server you'll use:
etc/init.d/srs status
etc/init.d/srs stop

NOTE: There are two different srs binaries, one in 'objs' and one in 'etc/init.d'. Each has different options so we use each for different purposes.
