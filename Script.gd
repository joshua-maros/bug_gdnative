extends Node

const Dummy = preload("res://Dummy.gdns")

var dummy: Dummy

func _init():
	dummy = Dummy.new()
	# Does not crash with:
	# dummy.make()
	dummy = dummy.make()
