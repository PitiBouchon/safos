# The start function

The two arguments of this function are the hart id and the address of the devicetree

## Hart Id

"Hart" is the abbreviation of "Hardware Thread"

Basically one hart equal one cpu

## DeviceTree

The device tree is a data structure placed in memory by the firmware in order for the OS to know the architecture of the machine

Often you'll see FDT and DTB which refer to Flatten Device Tree and the Device Tree Blob which corresponds to the data 
structure and the binary data of the device tree
