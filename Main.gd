extends Node2D

var t : IPFS
func _ready():
	
	# Starts the IPFS Daemon for a Dag test
	
	t = IPFS.new()
	#IPFS.run()
	
