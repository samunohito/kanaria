package com.kanaria.exception

class UnsupportedPlatformException(val name: String?, val arch: String?) :
        RuntimeException("The library for the currently running platform could not be found. [os.name=$name, os.arch=$arch]", null)