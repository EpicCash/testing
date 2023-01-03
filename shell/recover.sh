#!/usr/bin/expect

set password [lindex $argv 0]
set walletbinary [lindex $argv 1]
set network [lindex $argv 2]
set passphrase [lindex $argv 3]

	cd $walletbinary
	spawn ./epic-wallet $network -p $password init -r
	expect "Please enter your recovery phrase:"
	send "$passphrase\r"
	expect "Command 'init' completed successfully"
	puts "ok\n"
