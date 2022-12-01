/**
 * Reads the input file for the given day to a string.
 * 
 * @param day The day for which to get the input file.
 * @return A string containing the content of the input file.
 */
function getDayInput(day) {
	const fs = require('fs')
	const path = require('path')
	
	var input = path.join(path.dirname(path.dirname(path.resolve(''))), 'input')
	fs.accessSync(input, fs.constants.F_OK)
	
	input = path.join(input, `Day${day}.txt`)
	fs.accessSync(input, fs.constants.R_OK)
	
	return fs.readFileSync(input).toString()
}

exports.getDayInput = getDayInput
