A program to generate maps from codes in the same way as Virtual Hydlide, made in an effort to help find better codes for speedrunning. 

SETUP

The program needs to be run in a directory with the 5 base maps in a `basemaps` folder. The base maps are the `GR_BASE<n>.BIN` files found in the `HYDLIDE\MAP01` directory on your Virtual Hydlide CD. 

If you wish to save maps, you will also need the `genmaps` directory. The program does no management of it, just spits out seeds and maps (you can view them in a hex editor if you are so inclined.)

If you wish to develop, you will need to install rust.

PROGRAM USAGE 

The program currently does 3 things. Option 1 lets you set the difficulty of the maps generated (in case there are differences, and there are differences between Easy/Medium and Hard/PRO.)

* Option 2 - Generate a single seed with your current difficulty.
* Option 3 - Show you all 5 base maps.
* Option 4 - Generate a large group of maps in linear seed order. You may choose the starting point (unsigned maxint 32), the amount (unsigned maxint32, but if both together >= 2^32, it's not gonna work), whether you want to 'winnow' (currently winnowing is only map 4 where the Tablet <- Sealed Caves <- Volcano path is minimized), and whether you want to save. There is no console output for this ATM, just saved maps.
