extends Node2D


func _ready():
	
	# Starts the IPFS Daemon for a Dag test
	
	IPFS.new()
	IPFS.testing()
	
