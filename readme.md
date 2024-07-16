A program to generate maps from codes in the same way as Virtual Hydlide, made in an effort to help find better codes for speedrunning. 

Additionally, a small webpage app to aid in the understanding of how seed generation works in gory binary detail. 

SETUP

The program needs to be run in a directory with the 5 base maps in a `basemaps` folder. The base maps are the `GR_BASE<n>.BIN` files found in the `HYDLIDE\MAP01` directory on your Virtual Hydlide CD. 

If you wish to save maps, you will also need the `genmaps` directory. The program does no management of it, just spits out seeds and maps (you can view them in a hex editor if you are so inclined.)

If you wish to develop, you will need to install rust.

PROGRAM USAGE 

The program currently does 4 actual things. Option 1 lets you set the difficulty of the maps generated (in case there are differences, and there are differences between Easy/Medium and Hard/PRO.)

* Option 2 - Generate a single seed with your current difficulty.
* Option 3 - Show you all 5 base maps.
* Option 4 - Generate a large group of maps in linear seed order. You may choose the starting point (unsigned maxint 32), the amount (unsigned maxint32, but if both together >= 2^32, it's not gonna work), whether you want to 'winnow', and whether you want to save. There is no console output for this ATM, just saved maps.
* Option 5 - Given a bunch of maps in the /genmaps directory, it will output a sorted list of bins of minimum length as per the 3rd winnow method.

WINNOWING

We have several winnowing processes available that you will be prompted for
* Only results with Map 4: This winnows out all the other map bases because their shortest last 3 will be worse than map 4's shortest last 3 by 10 or more. This does cut out possibilities that could be optimal.
* Only results with the shortest last 3: This winows out all the maps where the V-S-C path is not the minimal 9. This does not take crystals into account. This will always be maps with MAp 4, so it's smart to use the last one and this one.
* Results with a shortest path (under Thunder Sword routing assumptions) under 50: With a Thunder Sword the route can be just (fairy, thunder sword, ruins), volcano, sealed, castle. This method ignores any overworld obstacles and doesn't use crystals. This will almost assuredly produce the shortest path without teleports or occlusions. There are overall shorter paths than anything I found with the previous method.

WEBPAGE
The webpage should be able to be run by loading the index.html in the www directory. Enter valid characters for strings and it will calculate out for you. Remember that all the default characters are spaces.