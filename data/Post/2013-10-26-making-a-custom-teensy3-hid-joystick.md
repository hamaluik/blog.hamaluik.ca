---
title: Making A Custom Teensy3 HID Joystick
slug: making-a-custom-teensy3-hid-joystick
author: kenton
published: 2013-10-26 02:59:00
category: programming
tags: Arduino, Teensy3, Tutorial, Unity
preview_image: /assets/images/making-a-custom-teensy3-hid-joystick/making-a-custom-teensy3-hid-joystick.jpg
preview_summary: I recently got married and for our wedding we decided we wanted to include some arcade games for a more unique, personal, and fun wedding experience. Me being the overly-ambitious type that I am decided it would be even more spectacular to create our own wedding arcade ("Wedcade" for short)! These actually turned out pretty decently in the end, and I'll try to write up a post or two about them (as well as the arcade cabinets!) later, but for now I want to talk about using the Teensy3 as a joystick (namely using Teensyduino).
---

I recently got married and for our wedding we decided we wanted to include some arcade games for a more unique, personal, and fun wedding experience. Me being the overly-ambitious type that I am decided it would be even more spectacular to create our **own** wedding arcade ("Wedcade" for short)! These actually turned out pretty decently in the end, and I'll try to write up a post or two about them (as well as the arcade cabinets!) later, but for now I want to talk about using the Teensy3 as a joystick (namely using Teensyduino).

<!-- PELICAN_END_SUMMARY -->

Now, normally with the Teensy3, if you load up the joystick example, it will give you a generic joystick with 32 buttons, 6 axes, and 1 hat switch. This should be more than enough for anybody, and it was what I was planning on using. However, for whatever unknown reason, when I tried to use this joystick with [Unity](http://unity3d.com/) (the game development suite I decided to use for the wedcade), Unity refused to acknowledge it and said the joystick was unsupported. This was a bit frustrating as it is really just a generic HID joystick, no drivers needed. I had my suspicions however that there were just simply too many buttons and / or axes or whatever for Unity to deal with, so it shut down the joystick altogether. With that in mind, I set out to make the Teensy enumerate as a generic joystick with only the axes and buttons that I needed it for!

Everything I did here was for Teensyduino, so all files should be located in the Arduino/hardware/teensy/cores/teensy3 folder. Further, I am defining a new USB device / type called "arcade" which is two pairs of axes (one for each player) and a total of 16 buttons (though I'm not using all 16 in my current version of the arcade).

I started off by editing **usb_desc.h**. This file is just a bunch of definitions that help set up the USB defitions in all the other files. After the **USB_FLIGHTSIM** definition area, I added a definition set which looks like:

```cpp
#elif defined(USB_ARCADE)
	#define VENDOR_ID   0x16C0
	#define PRODUCT_ID	0x0489
	#define DEVICE_CLASS	0x03
	#define MANUFACTURER_NAME {'B', 'l', 'a', 'z', 'i', 'n', 'g', 'M', 'a', 'm', 'm', 'o', 't', 'h'}
	#define MANUFACTURER_NAME_LEN 14
	#define PRODUCT_NAME	{'W', 'e', 'd', 'c', 'a', 'd', 'e', ' ', 'C', 'o', 'n', 't', 'r', 'o', 'l', 'l', 'e', 'r'}
	#define PRODUCT_NAME_LEN  18
	#define EP0_SIZE			  64
	#define NUM_ENDPOINTS		 2
	#define NUM_USB_BUFFERS	   30
	#define NUM_INTERFACE		 1
	#define ARCADE_INTERFACE	  0 // Joystick
	#define ARCADE_ENDPOINT	   1
	#define ARCADE_SIZE		   16
	#define ARCADE_INTERVAL	   1
	#define ARCADE_DESC_OFFSET  (9)
	#define CONFIG_DESC_SIZE  (9 + 9+9+7)
	#define ENDPOINT1_CONFIG  ENDPOINT_TRANSIMIT_ONLY
```
	
  * In this section, if the **USB_ARCADE** definition is given, we set up all the included files to work as a "Wedcade Controller". Specifically, I left the vendor ID to be the same as all the rest of the teensy devices, and created a new product id (0x0489).
  * The class is defined as **0x03** (or, "**Human Interface Device**").
  * The manufacturer name and product name are pretty self-explanatory.
  * **EP0_SIZE** actually refers to the maximum packet size that the device will transmit. Note that I won't be sending 64 byte packets, though I could. I believe this could easily be changed to what my actual packet size is, but it works like this so why mess with it?
  * To understand the **NUM_ENDPOINTS** field, note that in USB communications each place that receives information counts as an "endpoint". Also note that endpoint 0 is reserved and necessary, so if there is only one place receiving information (the computer in this case), there will be a total of 2 endpoints (the reserved one and the place my arcade controller is sending data). If I somehow had a force-feedback controller, there would be 3 endpoints (the third would be for the controller to read force-feedback data from the computer). I think.
  * **NUM_USB_BUFFERS** is the number of packets to buffer. 30 was just the default for the USB_SERIAL_HID section so I figured it would be ok here. It could probably be less as there is a lot less data transmission in my case.
  * **NUM_INTERFACE** defines how many virtual things this usb device will represent. In my case, there is just the single controller, so this is 1.
  * With only 1 interface, **ARCADE_INTERFACE** is the first one (0)
  * The arcade controller also connects to the first available endpoint (**ARCADE_ENDPOINT** 1) (0 is reserved, remember?)
  * Since my device only sends information, I configured the endpoint to be transmit only
  * The remaining parameters all relate to sizes of the data fields that describe the USB device. Nothing to do here but count bytes in the _usb_desc.c_ file.


With **usb_desc.h** in place, I added the following to the usb_desc.c file (right below the **#ifdef JOYSTICK_INTERFACE ... #endif** section:

```cpp
#ifdef ARCADE_INTERFACE
static uint8_t arcade_report_desc[] = {
	0x05, 0x01,					// USAGE_PAGE (Generic Desktop)
	0x09, 0x04,					// USAGE (Joystick)
	0xa1, 0x01,					// COLLECTION (Application)
	0x09, 0x04,					//   USAGE (Joystick)
	0xa1, 0x00,					//   COLLECTION (Physical)
	0x09, 0x30,					//	 USAGE (X)
	0x09, 0x31,					//	 USAGE (Y)
	0x09, 0x33,					//	 USAGE (Rx)
	0x09, 0x34,					//	 USAGE (Ry)
	0x75, 0x08,					//	 REPORT_SIZE (8)
	0x95, 0x04,					//	 REPORT_COUNT (4)
	0x45, 0x7f,					//	 PHYSICAL_MAXIMUM (127)
	0x35, 0x81,					//	 PHYSICAL_MINIMUM (-127)
	0x81, 0x02,					//	 INPUT (Data,Var,Abs)
	0x05, 0x09,					//	 USAGE_PAGE (Button)
	0x19, 0x01,					//	 USAGE_MINIMUM (Button 1)
	0x29, 0x10,					//	 USAGE_MAXIMUM (Button 16)
	0x15, 0x00,					//	 LOGICAL_MINIMUM (0)
	0x25, 0x01,					//	 LOGICAL_MAXIMUM (1)
	0x75, 0x01,					//	 REPORT_SIZE (1)
	0x95, 0x10,					//	 REPORT_COUNT (16)
	0x81, 0x02,					//	 INPUT (Data,Var,Abs)
	0xc0,						//   END_COLLECTION
	0xc0						// END_COLLECTION
};
#endif
```

This is the actual USB descriptor and was created using the somewhat clunky [USB Descriptor Tool](http://www.usb.org/developers/hidpage#HID Descriptor Tool). It defines that we have a joystick device with 4 physical axes and 16 physical buttons. Each of the 4 axes will report a single byte with a value between -127 and 127 and each button will report a value of 0 or 1. All 16 buttons will be combined into 2 bytes of data so they're all packed together (for a total packet size of 6 bytes). For more information on how to create something like this you can try [http://www.frank-zhao.com/cache/hid_tutorial_1.php](http://www.frank-zhao.com/cache/hid_tutorial_1.php) and/or the pdf at [http://www.picbasic.co.uk/forum/showthread.php?t=11950](http://www.picbasic.co.uk/forum/showthread.php?t=11950). It took me a little bit to get this right, so I wish you the best of luck if you want to deviate substantially from this!

With the descriptor written, add the following to the **usb_desc.c** file (right below the line "**#endif // JOYSTICK_INTERFACE**"):

```cpp
#ifdef ARCADE_INTERFACE
		// interface descriptor, USB spec 9.6.5, page 267-269, Table 9-12
		9,									  // bLength
		4,									  // bDescriptorType
		ARCADE_INTERFACE,					 // bInterfaceNumber
		0,									  // bAlternateSetting
		1,									  // bNumEndpoints
		0x03,								   // bInterfaceClass (0x03 = HID)
		0x00,								   // bInterfaceSubClass
		0x00,								   // bInterfaceProtocol
		0,									  // iInterface
		// HID interface descriptor, HID 1.11 spec, section 6.2.1
		9,									  // bLength
		0x21,								   // bDescriptorType
		0x11, 0x01,							 // bcdHID
		0,									  // bCountryCode
		1,									  // bNumDescriptors
		0x22,								   // bDescriptorType
		LSB(sizeof(arcade_report_desc)),		// wDescriptorLength
		MSB(sizeof(arcade_report_desc)),
		// endpoint descriptor, USB spec 9.6.6, page 269-271, Table 9-13
		7,									  // bLength
		5,									  // bDescriptorType
		ARCADE_ENDPOINT | 0x80,			   // bEndpointAddress
		0x03,								   // bmAttributes (0x03=intr)
		ARCADE_SIZE, 0,					   // wMaxPacketSize
		ARCADE_INTERVAL,					  // bInterval
#endif // ARCADE_INTERFACE
```


This is another section that defines the usb interface we're using. It's pretty much copy-pasted from the JOYSTICK_INTERFACE section, with name changes to reference the arcade aspect of it.

Finally, near the bottom of the file, add (just below the **#ifdef JOYSTICK_INTERACE ... #endif** section):

```cpp
#ifdef ARCADE_INTERFACE
		{0x2200, ARCADE_INTERFACE, arcade_report_desc, sizeof(arcade_report_desc)},
		{0x2100, ARCADE_INTERFACE, config_descriptor+ARCADE_DESC_OFFSET, 9},
#endif
```


This just actually includes the descriptor in the list and should be all we need to do to set up the USB descriptors. Now we just need to write a class to actually use our joystick with. I'll post my code here, but it is more or less a direct copy-paste and slight strip-down of the usb_joystick.h / usb_joystick.c files found in the directory:

```cpp
// usb_arcade.h
#ifndef _USB_ARCADE_H_
#define _USB_ARCADE_H_

#if defined(USB_ARCADE)

#include <inttypes.h>

// C language implementation
#ifdef __cplusplus
extern "C" {
#endif
int usb_arcade_send(void);
// we have packets that are 6 bytes long
extern uint8_t usb_arcade_data[6];
#ifdef __cplusplus
}
#endif

// C++ interface
#ifdef __cplusplus
class usb_arcade_class
{
private:
	static uint8_t auto_send;

public:
	void button(uint8_t button, bool val) {
		if (--button >= 16) return;
		if (val) usb_arcade_data[button >= 8 ? 5 : 4] |= (1 << (button >= 8 ? (button - 8) : button));
		else usb_arcade_data[button >= 8 ? 5 : 4] &= ~(1 << (button >= 8 ? (button - 8) : button));
		if(auto_send) usb_arcade_send();
	}

	void axis(uint8_t axis, int8_t val) {
		if(axis >= 4) return;
		if(val > 0) usb_arcade_data[axis] = 127;
		else if(val < 0) usb_arcade_data[axis] = -127;
		else usb_arcade_data[axis] = 0;
		if(auto_send) usb_arcade_send();
	}

	void setAutoSend(bool send) {
		auto_send = send;
	}

	void send() {
		usb_arcade_send();
	}
};
extern usb_arcade_class Arcade;

#endif // __cplusplus
#endif // USB_ARCADE
#endif // _USB_ARCADE_H_
```

```
// usb_arcade.c
#include "usb_dev.h"
#include "usb_arcade.h"
#include "core_pins.h" // for yield()
#include "HardwareSerial.h"
#include <string.h> // for memcpy()

#ifdef USB_ARCADE // defined by usb_dev.h -> usb_desc.h

uint8_t usb_arcade_data[6];

// Maximum number of transmit packets to queue so we don't starve other endpoints for memory
#define TX_PACKET_LIMIT 3

static uint8_t transmit_previous_timeout=0;

// When the PC isn't listening, how long do we wait before discarding data?
#define TX_TIMEOUT_MSEC 30

#if F_CPU == 96000000
  #define TX_TIMEOUT (TX_TIMEOUT_MSEC * 596)
#elif F_CPU == 48000000
  #define TX_TIMEOUT (TX_TIMEOUT_MSEC * 428)
#elif F_CPU == 24000000
  #define TX_TIMEOUT (TX_TIMEOUT_MSEC * 262)
#endif

int usb_arcade_send(void)
{
	uint32_t wait_count=0;
	usb_packet_t *tx_packet;

	while (1) {
		if (!usb_configuration) {
			return -1;
		}
		if (usb_tx_packet_count(ARCADE_ENDPOINT) < TX_PACKET_LIMIT) {			tx_packet = usb_malloc();			if (tx_packet) break;		}		if (++wait_count > TX_TIMEOUT || transmit_previous_timeout) {
																	 	transmit_previous_timeout = 1;
																	 	return -1;
		}
		yield();
	}

	transmit_previous_timeout = 0;
	memcpy(tx_packet->buf, usb_arcade_data, 6);
	tx_packet->len = 6;
	usb_tx(ARCADE_ENDPOINT, tx_packet);

	return 0;
}

#endif // USB_ARCADE
```


All this code does is provide an interface for defining usb packets to be sent to the computer and gives us the functions Arcade.button(button, val) and Arcade.axis(axis, val) to use instead of manually writing the packets and sending them ourselves. In order to make these classes accessible in the Arduino environment, add the following to the **usb_inst.cpp** file:

```cpp
#ifdef USB_ARCADE
usb_arcade_class Arcade;
uint8_t usb_arcade_class::auto_send = 0;
#endif
```

And the following to **WProgram.h**:

```cpp
#include "usb_arcade.h"
```

Finally, in order to actually make the Arduino environment use the new arcade descriptors, add the following lines to the file _Arduino/hardware/teensy/boards.txt_ (after the teensy3.menu.usb.flightsim entries):

```cpp
teensy3.menu.usb.arcade.name=Wedcade Controller
teensy3.menu.usb.arcade.build.define0=-DUSB_ARCADE
teensy3.menu.usb.arcade.fake_serial=teensy_gateway
```

And that's pretty much it! I included the sketch I used so you can see how it works, but basically I have my buttons connected to a bunch of pins. I then poll these at 50 Hz and send an update packet to the computer at each iteration. I ended up not using the axes at all because although they worked in Windows, they did not work properly in Unity. Instead I just mapped each of the up / right / down / left directions to a button on the controller and used the buttons for directional input instead of the axes.

```cpp
const int ledPin = 13;

// joystick colour mappings
// g  l
// y  r
// o  d
// r  u

// player 1 bank
const int P1_A = 2;
const int P1_B = 3;
const int P1_LEFT = 4;
const int P1_RIGHT = 5;
const int P1_DOWN = 6;
const int P1_UP = 7;

const int START_BUTTON = 8;

// player 2 bank
const int P2_UP = 15;
const int P2_DOWN = 14;
const int P2_RIGHT = 17;
const int P2_LEFT = 16;
const int P2_B = 18;
const int P2_A = 19;

boolean ledOn = false;
unsigned long lastTime = 0;

void setup()
{
	pinMode(ledPin, OUTPUT);

	pinMode(P1_A, INPUT_PULLUP);
	pinMode(P1_B, INPUT_PULLUP);
	pinMode(P1_LEFT, INPUT_PULLUP);
	pinMode(P1_RIGHT, INPUT_PULLUP);
	pinMode(P1_DOWN, INPUT_PULLUP);
	pinMode(P1_UP, INPUT_PULLUP);
	pinMode(START_BUTTON, INPUT_PULLUP);
	pinMode(P2_A, INPUT_PULLUP);
	pinMode(P2_B, INPUT_PULLUP);
	pinMode(P2_LEFT, INPUT_PULLUP);
	pinMode(P2_RIGHT, INPUT_PULLUP);
	pinMode(P2_DOWN, INPUT_PULLUP);
	pinMode(P2_UP, INPUT_PULLUP);

	// read the entire joystick at once instead of per event
	Arcade.setAutoSend(false);

	lastTime = millis();
}

void loop()
{
	unsigned long time = millis();
	// run at 50 Hz
	if(time - lastTime >= 20)
	{
		lastTime = time;

		// read the data of all our buttons
		// our buttons
		Arcade.button(1, 1 - digitalRead(P1_A));
		Arcade.button(2, 1 - digitalRead(P1_B));
		Arcade.button(3, 1 - digitalRead(P2_A));
		Arcade.button(4, 1 - digitalRead(P2_B));

		Arcade.button(5, 1 - digitalRead(START_BUTTON));

		// now our axes
		Arcade.axis(0, digitalRead(P1_LEFT) == 0 ? -1 : (digitalRead(P1_RIGHT) == 0 ? 1 : 0));
		Arcade.axis(1, digitalRead(P1_UP) == 0 ? -1 : (digitalRead(P1_DOWN) == 0 ? 1 : 0));
		Arcade.axis(2, digitalRead(P2_LEFT) == 0 ? -1 : (digitalRead(P2_RIGHT) == 0 ? 1 : 0));
		Arcade.axis(3, digitalRead(P2_UP) == 0 ? -1 : (digitalRead(P2_DOWN) == 0 ? 1 : 0));

		// also use buttons for the axes cuz unity is a derp
		Arcade.button(6, 1 - digitalRead(P1_UP));
		Arcade.button(7, 1 - digitalRead(P1_RIGHT));
		Arcade.button(8, 1 - digitalRead(P1_DOWN));
		Arcade.button(9, 1 - digitalRead(P1_LEFT));
		Arcade.button(10, 1 - digitalRead(P2_UP));
		Arcade.button(11, 1 - digitalRead(P2_RIGHT));
		Arcade.button(12, 1 - digitalRead(P2_DOWN));
		Arcade.button(13, 1 - digitalRead(P2_LEFT));

		// send a packet now
		Arcade.send();

		// toggle our led
		ledOn = !ledOn;
		digitalWrite(ledPin, ledOn);
	}
}
```

I apologize for sloppy or incoherent code, but I was mostly figuring things out from scratch with a tight time budget, and my only goal was to get this to **just work**, which they thankfully did in the end!

That's all for now, please let me know if you run into any issues or if what I wrote here didn't make sense! I can't promise I can fix it but I will certainly promise to try!
